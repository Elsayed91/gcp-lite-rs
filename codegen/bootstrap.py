#!/usr/bin/env python3
"""Bootstrap a TOML manifest from a GCP Discovery Document.

Usage:
    python3 codegen/bootstrap.py iam v1
    python3 codegen/bootstrap.py compute v1
    python3 codegen/bootstrap.py cloudscheduler v1

Fetches the discovery document (or uses cache), analyzes resources and schemas,
and emits a draft TOML manifest at codegen/manifests/<api_name>.toml.

The draft includes all discovered types and operations with coverage stats.
Review and curate before using with generate.py.
"""

import json
import re
import sys
import urllib.request
from pathlib import Path

CACHE_DIR = Path(__file__).parent / "discovery_cache"
MANIFESTS_DIR = Path(__file__).parent / "manifests"
STANDARD_URL = "https://www.googleapis.com/discovery/v1/apis/{name}/{version}/rest"

# Discovery doc APIs that use a non-standard URL
CUSTOM_URLS = {
    "serviceusage": "https://serviceusage.googleapis.com/$discovery/rest?version={version}",
    "cloudresourcemanager": "https://cloudresourcemanager.googleapis.com/$discovery/rest?version={version}",
    "apikeys": "https://apikeys.googleapis.com/$discovery/rest?version={version}",
}


def to_snake_case(name: str) -> str:
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    return s.lower()


def fetch_or_load(api_name: str, version: str) -> dict:
    """Fetch or load a cached discovery document."""
    cache_file = CACHE_DIR / f"{api_name}.{version}.json"
    if cache_file.exists():
        print(f"  Using cached: {cache_file}")
        with open(cache_file) as f:
            return json.load(f)

    url_template = CUSTOM_URLS.get(api_name, STANDARD_URL)
    url = url_template.format(name=api_name, version=version)
    print(f"  Downloading: {url}")
    req = urllib.request.Request(url, headers={"Accept": "application/json"})
    with urllib.request.urlopen(req) as resp:
        data = json.loads(resp.read())

    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    with open(cache_file, "w") as f:
        json.dump(data, f, indent=2)
    print(f"  Cached to: {cache_file}")
    return data


def detect_lro_pattern(discovery: dict) -> dict:
    """Detect what kind of LRO (if any) this API uses."""
    schemas = discovery.get('schemas', {})

    # Check if any mutating method returns an "Operation" schema
    has_operation_schema = 'Operation' in schemas
    if not has_operation_schema:
        return {"pattern": "none"}

    op_schema = schemas['Operation']
    op_props = op_schema.get('properties', {})

    # Selflink pattern: has "selfLink" and "status" (Compute-style)
    if 'selfLink' in op_props and 'status' in op_props:
        return {
            "pattern": "selflink",
            "response_type": "OperationResponse",
            "poll_config": "disk_operation",
        }

    # Name-based pattern: has "name" and "done" (Google LRO-style)
    if 'name' in op_props and 'done' in op_props:
        base_url = discovery.get('rootUrl', '').rstrip('/')
        version = discovery.get('version', 'v1')
        return {
            "pattern": "name_based",
            "response_type": f"{discovery.get('title', 'Api').replace(' ', '')}Lro",
            "poll_config": f"{to_snake_case(discovery.get('name', 'api'))}_operation",
            "base_url": base_url,
            "poll_path": f"/{version}/{{name}}",
        }

    return {"pattern": "none"}


def collect_resources(discovery: dict) -> list[dict]:
    """Walk the resource tree and collect all methods."""
    results = []

    def walk(resources: dict, prefix: str = ""):
        for name, res in resources.items():
            full_name = f"{prefix}{name}" if prefix else name
            for method_name, method_data in res.get('methods', {}).items():
                results.append({
                    "resource": full_name,
                    "method": method_name,
                    "data": method_data,
                })
            if 'resources' in res:
                walk(res['resources'], f"{full_name}.")

    walk(discovery.get('resources', {}))
    return results


def collect_schemas_for_methods(methods: list[dict], schemas: dict) -> set[str]:
    """Find all schema names referenced by the given methods."""
    refs = set()
    for m in methods:
        data = m['data']
        req_ref = data.get('request', {}).get('$ref', '')
        resp_ref = data.get('response', {}).get('$ref', '')
        if req_ref:
            refs.add(req_ref)
        if resp_ref:
            refs.add(resp_ref)
    return refs


def infer_rust_name(method: dict) -> str:
    """Infer a Rust function name from a discovery method."""
    resource = method['resource']
    method_name = method['method']

    # Standard REST -> Rust name mapping
    name_map = {
        'insert': 'create',
        'patch': 'update',
    }
    action = name_map.get(method_name, method_name)

    # Singularize resource for non-list methods
    singular = resource.rstrip('s') if resource.endswith('s') and method_name != 'list' else resource
    singular = to_snake_case(singular)
    resource_snake = to_snake_case(resource)

    if method_name == 'list':
        return f"list_{resource_snake}"
    else:
        return f"{action}_{singular}"


def detect_list_response(method: dict, schemas: dict) -> dict | None:
    """Detect if a method returns a list response."""
    resp_ref = method['data'].get('response', {}).get('$ref', '')
    if not resp_ref:
        return None

    schema = schemas.get(resp_ref, {})
    props = schema.get('properties', {})

    # Look for a Vec field + nextPageToken
    if 'nextPageToken' not in props:
        return None

    # Find the items field (the array field that isn't nextPageToken)
    for fname, fprop in props.items():
        if fname == 'nextPageToken':
            continue
        if fprop.get('type') == 'array':
            items_ref = fprop.get('items', {}).get('$ref', '')
            if items_ref:
                return {
                    "type_name": resp_ref,
                    "items_field": fname,
                    "item_type": items_ref,
                }
    return None


def generate_manifest(api_name: str, version: str, discovery: dict) -> str:
    """Generate a draft TOML manifest."""
    schemas = discovery.get('schemas', {})
    title = discovery.get('title', api_name)
    base_url = discovery.get('baseUrl', '').rstrip('/')
    doc_url = discovery.get('documentationLink', '')
    discovery_url = discovery.get('discoveryRestUrl', STANDARD_URL.format(name=api_name, version=version))

    methods = collect_resources(discovery)
    lro_info = detect_lro_pattern(discovery)
    referenced_schemas = collect_schemas_for_methods(methods, schemas)

    lines = []
    lines.append(f"# {title} API - Codegen Manifest (DRAFT)")
    lines.append(f"# Generated by bootstrap.py — review and curate before use.")
    lines.append(f"# Total schemas in discovery doc: {len(schemas)}")
    lines.append(f"# Total methods found: {len(methods)}")
    lines.append(f"")
    lines.append(f"[api]")
    lines.append(f'name = "{to_snake_case(api_name)}"')
    lines.append(f'display_name = "{title}"')
    lines.append(f'version = "{version}"')
    lines.append(f'discovery_url = "{discovery_url}"')
    lines.append(f'base_url = "{base_url}"')
    if doc_url:
        lines.append(f'doc_url = "{doc_url}"')
    lines.append(f"")

    # Client accessor
    snake_name = to_snake_case(api_name)
    client_struct = ''.join(w.capitalize() for w in snake_name.split('_')) + 'Client'
    lines.append(f"[api.client]")
    lines.append(f'accessor_name = "{snake_name}"')
    lines.append(f'client_struct = "{client_struct}"')
    lines.append(f"")

    # LRO config
    if lro_info['pattern'] != 'none':
        lines.append(f"[api.lro]")
        lines.append(f'pattern = "{lro_info["pattern"]}"')
        lines.append(f'response_type = "{lro_info["response_type"]}"')
        lines.append(f'poll_config = "{lro_info["poll_config"]}"')
        if 'base_url' in lro_info:
            lines.append(f'base_url = "{lro_info["base_url"]}"')
            lines.append(f'poll_path = "{lro_info["poll_path"]}"')
        lines.append(f"")

    # Types section
    lines.append(f"# --- Types ---")
    lines.append(f"# Review each type: remove unnecessary fields, add field_overrides as needed.")
    lines.append(f"")

    # Emit types for referenced schemas (excluding Operation which is the LRO response)
    lro_response_schema = None
    if lro_info['pattern'] != 'none':
        # Detect what schema is the Operation response
        for m in methods:
            resp_ref = m['data'].get('response', {}).get('$ref', '')
            if resp_ref == 'Operation':
                lro_response_schema = resp_ref
                break

    emitted_schemas = set()
    list_response_schemas = set()

    # First pass: collect list response type names so we don't emit them as [[types]]
    for m in methods:
        lr = detect_list_response(m, schemas)
        if lr:
            list_response_schemas.add(lr['type_name'])

    for schema_name in sorted(referenced_schemas):
        if schema_name in list_response_schemas:
            continue  # Will be generated as list_response on the operation

        schema = schemas.get(schema_name, {})
        if not schema:
            continue

        props = schema.get('properties', {})
        field_names = sorted(props.keys())

        # For the LRO Operation schema, give it a special name
        if schema_name == lro_response_schema and lro_info['pattern'] != 'none':
            lines.append(f"[[types]]")
            lines.append(f'schema = "{schema_name}"')
            lines.append(f'rust_name = "{lro_info["response_type"]}"')
            # For selflink pattern, include minimal fields
            if lro_info['pattern'] == 'selflink':
                lines.append(f'include_fields = ["name", "operationType", "status", "targetLink", "selfLink"]')
                lines.append(f"")
                lines.append(f"[types.field_overrides]")
                lines.append(f"name = {{ required = true }}")
            elif lro_info['pattern'] == 'name_based':
                lines.append(f'include_fields = ["name", "done", "error", "response", "metadata"]')
                lines.append(f"")
                lines.append(f"[types.field_overrides]")
                lines.append(f"name = {{ required = true }}")
                lines.append(f"done = {{ required = true }}")
                lines.append(f'error = {{ rust_type = "serde_json::Value" }}')
                lines.append(f'response = {{ rust_type = "serde_json::Value" }}')
                lines.append(f'metadata = {{ rust_type = "serde_json::Value" }}')
            lines.append(f"")
            emitted_schemas.add(schema_name)
            continue

        lines.append(f"[[types]]")
        lines.append(f'schema = "{schema_name}"')
        if field_names:
            # Include all fields by default — developer will curate
            fields_str = ', '.join(f'"{f}"' for f in field_names)
            lines.append(f"include_fields = [{fields_str}]")

        # Auto-detect common field overrides
        overrides = []
        for fname in field_names:
            fprop = props.get(fname, {})
            # name fields are usually required
            if fname == 'name':
                overrides.append(f'name = {{ required = true }}')
            # Reserved words need renaming
            if fname in ('type', 'match', 'self', 'ref', 'mod', 'use'):
                rust_name = to_snake_case(fname) + "_value" if fname == 'type' else to_snake_case(fname) + "_field"
                overrides.append(f'{fname} = {{ rust_name = "{rust_name}", serde_rename = "{fname}" }}')

        if overrides:
            lines.append(f"")
            lines.append(f"[types.field_overrides]")
            for o in overrides:
                lines.append(o)

        lines.append(f"")
        emitted_schemas.add(schema_name)

    # Operations section
    lines.append(f"# --- Operations ---")
    lines.append(f"# {len(methods)} methods found in discovery document.")
    lines.append(f"")

    for m in methods:
        data = m['data']
        rust_name = infer_rust_name(m)
        http_method = data.get('httpMethod', 'GET')
        resp_ref = data.get('response', {}).get('$ref', '')
        description = re.sub(r'\s+', ' ', data.get('description', '')).strip()[:120]

        # Determine if this is an LRO
        is_lro = (resp_ref == lro_response_schema and lro_info['pattern'] != 'none')

        # Determine discovery_method format
        resource = m['resource']
        method_name = m['method']
        if '.' in resource:
            discovery_method = f"{resource}.{method_name}"
        else:
            discovery_method = method_name

        lines.append(f"[[operations]]")

        # Use resource+method or just method depending on API style
        if '.' not in resource:
            lines.append(f'discovery_resource = "{resource}"')
            lines.append(f'discovery_method = "{method_name}"')
        else:
            lines.append(f'discovery_method = "{resource}.{method_name}"')

        lines.append(f'rust_name = "{rust_name}"')
        if description:
            lines.append(f'description = "{description}"')

        if is_lro:
            lines.append(f"is_lro = true")

        # Check for list response
        lr = detect_list_response(m, schemas)
        if lr:
            lines.append(
                f'list_response = {{ type_name = "{lr["type_name"]}", '
                f'items_field = "{lr["items_field"]}", '
                f'item_type = "{lr["item_type"]}" }}'
            )

        lines.append(f"")

    # Coverage stats
    lines.append(f"# === Coverage Stats ===")
    lines.append(f"# Schemas in discovery: {len(schemas)}")
    lines.append(f"# Schemas referenced by methods: {len(referenced_schemas)}")
    lines.append(f"# Schemas emitted as types: {len(emitted_schemas)}")
    lines.append(f"# Methods found: {len(methods)}")
    lines.append(f"# LRO pattern: {lro_info['pattern']}")

    return '\n'.join(lines)


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 codegen/bootstrap.py <api_name> [version]")
        print("       python3 codegen/bootstrap.py iam v1")
        print("       python3 codegen/bootstrap.py compute v1")
        sys.exit(1)

    api_name = sys.argv[1]
    version = sys.argv[2] if len(sys.argv) > 1 else "v1"

    # Use the manifest-local snake_case name for the output file
    snake_name = to_snake_case(api_name)

    print(f"Bootstrapping manifest for {api_name} {version}...")
    discovery = fetch_or_load(api_name, version)

    manifest_content = generate_manifest(api_name, version, discovery)

    output_path = MANIFESTS_DIR / f"{snake_name}.toml"
    if output_path.exists():
        print(f"\n  WARNING: {output_path} already exists!")
        print(f"  Writing draft to: {output_path}.draft")
        output_path = Path(str(output_path) + ".draft")

    MANIFESTS_DIR.mkdir(parents=True, exist_ok=True)
    with open(output_path, 'w') as f:
        f.write(manifest_content)

    print(f"\n  Draft manifest written to: {output_path}")
    print(f"  Next steps:")
    print(f"    1. Review and curate the manifest (remove unneeded fields/types)")
    print(f"    2. task codegen:apply  (generates + applies to src/)")


if __name__ == "__main__":
    main()
