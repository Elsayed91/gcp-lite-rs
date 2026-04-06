#!/usr/bin/env python3
"""Browse GCP Discovery Documents and generate manifest entries.

Usage:
    # Show types/operations available but not in manifest
    python3 codegen/extend.py compute --available-types
    python3 codegen/extend.py compute --available-ops

    # Generate manifest entry for a type or operation
    python3 codegen/extend.py compute --add-type Snapshot
    python3 codegen/extend.py compute --add-op snapshots.insert

    # Audit format:"byte" fields
    python3 codegen/extend.py compute --audit
    python3 codegen/extend.py --audit-all

    # Diff manifest against discovery doc (detect upstream API changes)
    python3 codegen/extend.py compute --diff
    python3 codegen/extend.py --diff-all
"""

import json
import re
import sys
from pathlib import Path

try:
    import tomllib  # Python 3.11+
except ImportError:
    try:
        import tomli as tomllib
    except ImportError:
        print("ERROR: Need Python 3.11+ (tomllib) or `pip install tomli`", file=sys.stderr)
        sys.exit(1)


CACHE_DIR = Path(__file__).parent / "discovery_cache"
MANIFESTS_DIR = Path(__file__).parent / "manifests"

# Rust reserved words that need renaming
RESERVED_WORDS = frozenset([
    'type', 'match', 'self', 'ref', 'mod', 'use', 'fn', 'let', 'mut',
    'pub', 'return', 'if', 'else', 'for', 'while', 'loop', 'break',
    'continue', 'struct', 'enum', 'impl', 'trait', 'where', 'async',
    'await', 'move', 'static', 'const', 'crate', 'super', 'extern',
    'unsafe', 'dyn', 'abstract', 'become', 'box', 'do', 'final',
    'macro', 'override', 'priv', 'typeof', 'unsized', 'virtual', 'yield',
])


def to_snake_case(name: str) -> str:
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    return s.lower()


def load_manifest(api_name: str) -> dict | None:
    """Load the TOML manifest for an API, or None if not found."""
    snake = to_snake_case(api_name)
    for path in MANIFESTS_DIR.glob("*.toml"):
        with open(path, 'rb') as f:
            m = tomllib.load(f)
        if m.get('api', {}).get('name') == snake:
            return m
    return None


def load_discovery(api_name: str, version: str) -> dict | None:
    """Load cached discovery doc, or None."""
    cache_file = CACHE_DIR / f"{to_snake_case(api_name)}.{version}.json"
    if not cache_file.exists():
        # Try exact name
        cache_file = CACHE_DIR / f"{api_name}.{version}.json"
    if not cache_file.exists():
        return None
    with open(cache_file) as f:
        return json.load(f)


def collect_all_methods(discovery: dict) -> list[dict]:
    """Walk resource tree and collect all methods with dotted paths."""
    results = []

    def walk(resources: dict, prefix: str = ""):
        for name, res in resources.items():
            full = f"{prefix}{name}" if prefix else name
            for method_name, method_data in res.get('methods', {}).items():
                results.append({
                    'resource': full,
                    'method': method_name,
                    'dotted': f"{full}.{method_name}",
                    'data': method_data,
                })
            if 'resources' in res:
                walk(res['resources'], f"{full}.")

    walk(discovery.get('resources', {}))
    return results


def get_manifest_schemas(manifest: dict) -> set[str]:
    """Get set of schema names in the manifest."""
    return {t['schema'] for t in manifest.get('types', [])}


def get_manifest_operations(manifest: dict) -> set[str]:
    """Get set of dotted method paths in the manifest."""
    ops = set()
    for op in manifest.get('operations', []):
        resource = op.get('discovery_resource', '')
        method = op.get('discovery_method', '')
        if resource and '.' not in method:
            ops.add(f"{resource}.{method}")
        else:
            ops.add(method)
    return ops


def cmd_available_types(manifest: dict, discovery: dict):
    """Show schemas in discovery doc but not in manifest."""
    schemas = discovery.get('schemas', {})
    manifest_schemas = get_manifest_schemas(manifest)

    # Also collect list_response type_names (these are auto-generated, not in [[types]])
    list_types = set()
    for op in manifest.get('operations', []):
        lr = op.get('list_response')
        if lr:
            list_types.add(lr.get('type_name', ''))

    available = sorted(set(schemas.keys()) - manifest_schemas - list_types)

    if not available:
        print("All schemas are already in the manifest.")
        return

    print(f"Available schemas ({len(available)} not in manifest):\n")
    for name in available:
        schema = schemas[name]
        desc = schema.get('description', '')
        field_count = len(schema.get('properties', {}))
        short_desc = re.sub(r'\s+', ' ', desc).strip()[:80] if desc else ''
        print(f"  {name} ({field_count} fields)")
        if short_desc:
            print(f"    {short_desc}")


def cmd_available_ops(manifest: dict, discovery: dict):
    """Show operations in discovery doc but not in manifest."""
    all_methods = collect_all_methods(discovery)
    manifest_ops = get_manifest_operations(manifest)

    available = [m for m in all_methods if m['dotted'] not in manifest_ops]

    if not available:
        print("All operations are already in the manifest.")
        return

    print(f"Available operations ({len(available)} not in manifest):\n")
    for m in available:
        data = m['data']
        http = data.get('httpMethod', '?')
        path = data.get('path', '')
        desc = re.sub(r'\s+', ' ', data.get('description', '')).strip()[:80]
        resp = data.get('response', {}).get('$ref', '')
        req = data.get('request', {}).get('$ref', '')
        print(f"  {m['dotted']}")
        print(f"    {http} {path}")
        if req:
            print(f"    Request: {req}")
        if resp:
            print(f"    Response: {resp}")
        if desc:
            print(f"    {desc}")
        print()


def cmd_add_type(manifest: dict, discovery: dict, schema_name: str):
    """Generate a manifest [[types]] entry for a schema."""
    schemas = discovery.get('schemas', {})
    schema = schemas.get(schema_name)

    if not schema:
        print(f"ERROR: Schema '{schema_name}' not found in discovery document.", file=sys.stderr)
        similar = [s for s in schemas if schema_name.lower() in s.lower()]
        if similar:
            print(f"  Similar: {', '.join(similar[:10])}", file=sys.stderr)
        sys.exit(1)

    props = schema.get('properties', {})
    field_names = sorted(props.keys())

    lines = []
    lines.append(f'[[types]]')
    lines.append(f'schema = "{schema_name}"')

    # Show all fields as a comment, include a subset by default
    lines.append(f'# Discovery doc fields ({len(field_names)} total):')
    for fname in field_names:
        fprop = props[fname]
        ftype = fprop.get('type', fprop.get('$ref', '?'))
        fmt = fprop.get('format', '')
        is_enum = 'enum' in fprop
        is_repeated = fprop.get('repeated', False)
        annotations = []
        if fmt:
            annotations.append(f'format: {fmt}')
        if is_enum:
            vals = fprop.get('enum', [])
            annotations.append(f'enum: [{", ".join(vals[:5])}{"..." if len(vals) > 5 else ""}]')
        if is_repeated:
            annotations.append('repeated')
        ann_str = f' ({", ".join(annotations)})' if annotations else ''
        lines.append(f'# - {fname} ({ftype}{ann_str})')

    lines.append(f'include_fields = [{", ".join(f"{chr(34)}{f}{chr(34)}" for f in field_names)}]')
    lines.append('')

    # Detect common field overrides
    overrides = []
    byte_fields = []
    for fname in field_names:
        fprop = props[fname]

        # Name field is usually required
        if fname == 'name':
            overrides.append(f'name = {{ required = true }}')

        # Reserved words
        if fname in RESERVED_WORDS:
            rust_name = f'{to_snake_case(fname)}_value'
            overrides.append(f'{fname} = {{ rust_name = "{rust_name}", serde_rename = "{fname}" }}')

        # Enum detection
        if 'enum' in fprop and fname != 'name':
            enum_name = f'{schema_name}{fname[0].upper()}{fname[1:]}'
            # Use PascalCase for enum name
            overrides.append(f'# {fname} = {{ enum_type = "{enum_name}" }}  # enum values: {fprop["enum"][:5]}')

        # format: byte detection
        if fprop.get('format') == 'byte':
            byte_fields.append(fname)
            overrides.append(f'{fname} = {{ format = "bytes" }}  # WARNING: format:"byte" in discovery')

    if overrides:
        lines.append('[types.field_overrides]')
        for o in overrides:
            lines.append(o)
        lines.append('')

    if byte_fields:
        lines.append(f'# WARNING: {len(byte_fields)} format:"byte" field(s) detected: {", ".join(byte_fields)}')
        lines.append('# These MUST have format = "bytes" override to avoid data corruption.')
    elif not any('byte' in p.get('format', '') for p in props.values()):
        lines.append('# No format:"byte" fields detected in this schema.')

    print('\n'.join(lines))


def cmd_add_op(manifest: dict, discovery: dict, op_path: str):
    """Generate a manifest [[operations]] entry for an operation."""
    all_methods = collect_all_methods(discovery)

    # Find the method
    matched = [m for m in all_methods if m['dotted'] == op_path]
    if not matched:
        # Try partial match
        matched = [m for m in all_methods if op_path in m['dotted']]
        if not matched:
            print(f"ERROR: Operation '{op_path}' not found.", file=sys.stderr)
            print(f"  Available operations:", file=sys.stderr)
            for m in all_methods[:20]:
                print(f"    {m['dotted']}", file=sys.stderr)
            sys.exit(1)
        if len(matched) > 1:
            print(f"Multiple matches for '{op_path}':", file=sys.stderr)
            for m in matched:
                print(f"  {m['dotted']}", file=sys.stderr)
            print(f"  Using first match: {matched[0]['dotted']}", file=sys.stderr)

    m = matched[0]
    data = m['data']
    resource = m['resource']
    method_name = m['method']
    http_method = data.get('httpMethod', 'GET')
    path = data.get('path', '')
    desc = re.sub(r'\s+', ' ', data.get('description', '')).strip()
    req_ref = data.get('request', {}).get('$ref', '')
    resp_ref = data.get('response', {}).get('$ref', '')
    params = data.get('parameters', {})

    # Infer rust_name
    name_map = {'insert': 'create', 'patch': 'update'}
    action = name_map.get(method_name, method_name)
    singular = resource.rstrip('s') if resource.endswith('s') and method_name != 'list' else resource
    if method_name == 'list':
        rust_name = f"list_{to_snake_case(resource)}"
    else:
        rust_name = f"{action}_{to_snake_case(singular)}"

    # Check if this returns an Operation (LRO)
    # Only flag as LRO if the response is the well-known "Operation" schema
    # (not just any schema that happens to have selfLink/status/done)
    schemas = discovery.get('schemas', {})
    is_lro = False
    if resp_ref == 'Operation' and resp_ref in schemas:
        resp_schema = schemas[resp_ref]
        resp_props = resp_schema.get('properties', {})
        if ('selfLink' in resp_props and 'status' in resp_props) or ('done' in resp_props):
            is_lro = True

    # Detect list response
    list_response = None
    if resp_ref and resp_ref in schemas:
        resp_schema = schemas[resp_ref]
        resp_props = resp_schema.get('properties', {})
        if 'nextPageToken' in resp_props:
            for fname, fprop in resp_props.items():
                if fname == 'nextPageToken':
                    continue
                if fprop.get('type') == 'array':
                    items_ref = fprop.get('items', {}).get('$ref', '')
                    if items_ref:
                        list_response = {
                            'type_name': resp_ref,
                            'items_field': fname,
                            'item_type': items_ref,
                        }
                        break

    # Query params
    query_params = {k: v for k, v in params.items() if v.get('location') == 'query'}
    repeated_qps = {k for k, v in query_params.items() if v.get('repeated', False)}

    lines = []
    lines.append('[[operations]]')

    if '.' not in resource:
        lines.append(f'discovery_resource = "{resource}"')
        lines.append(f'discovery_method = "{method_name}"')
    else:
        lines.append(f'discovery_method = "{resource}.{method_name}"')

    lines.append(f'rust_name = "{rust_name}"')

    if desc:
        # Escape quotes in description
        safe_desc = desc.replace('"', '\\"')[:120]
        lines.append(f'description = "{safe_desc}"')

    if is_lro:
        lines.append('is_lro = true')

    if list_response:
        lines.append(
            f'list_response = {{ type_name = "{list_response["type_name"]}", '
            f'items_field = "{list_response["items_field"]}", '
            f'item_type = "{list_response["item_type"]}" }}'
        )

    if query_params:
        all_qps = sorted(query_params.keys())
        lines.append(f'query_params = [{", ".join(f"{chr(34)}{q}{chr(34)}" for q in all_qps)}]')
        if repeated_qps:
            lines.append(f'# NOTE: repeated query params (codegen handles automatically): {", ".join(sorted(repeated_qps))}')

    lines.append('')
    lines.append(f'# Path: {path}')
    lines.append(f'# HTTP: {http_method}')
    if req_ref:
        lines.append(f'# Request body: $ref {req_ref}')
    if resp_ref:
        lines.append(f'# Response: $ref {resp_ref}')
    if query_params:
        lines.append(f'# Query params available: {", ".join(sorted(query_params.keys()))}')

    print('\n'.join(lines))


def cmd_audit(manifest: dict, discovery: dict, api_name: str):
    """Audit manifest for format:"byte" fields missing format="bytes" overrides."""
    schemas = discovery.get('schemas', {})
    issues = []

    for type_conf in manifest.get('types', []):
        schema_name = type_conf['schema']
        schema = schemas.get(schema_name, {})
        props = schema.get('properties', {})
        include_fields = type_conf.get('include_fields', list(props.keys()))
        field_overrides = type_conf.get('field_overrides', {})

        for fname in include_fields:
            if fname not in props:
                continue
            fprop = props[fname]
            if fprop.get('format') == 'byte':
                override = field_overrides.get(fname, {})
                if isinstance(override, dict) and override.get('format') == 'bytes':
                    continue  # Properly handled
                issues.append((schema_name, fname))

    if not issues:
        print(f"  {api_name}: No unhandled format:\"byte\" fields found.")
        return

    print(f"  {api_name}: {len(issues)} unhandled format:\"byte\" field(s):")
    for schema_name, fname in issues:
        print(f"    {schema_name}.{fname}")
    print(f"\n  Fix by adding to [types.field_overrides]:")
    for schema_name, fname in issues:
        print(f'    {fname} = {{ format = "bytes" }}')


def cmd_audit_all():
    """Audit all manifests."""
    total_issues = 0
    for manifest_path in sorted(MANIFESTS_DIR.glob("*.toml")):
        with open(manifest_path, 'rb') as f:
            manifest = tomllib.load(f)
        api_name = manifest['api']['name']
        version = manifest['api'].get('version', 'v1')
        discovery = load_discovery(api_name, version)
        if not discovery:
            print(f"  {api_name}: Discovery doc not cached, run 'task codegen:fetch' first")
            continue

        schemas = discovery.get('schemas', {})
        issues = []
        for type_conf in manifest.get('types', []):
            schema_name = type_conf['schema']
            schema = schemas.get(schema_name, {})
            props = schema.get('properties', {})
            include_fields = type_conf.get('include_fields', list(props.keys()))
            field_overrides = type_conf.get('field_overrides', {})

            for fname in include_fields:
                if fname not in props:
                    continue
                fprop = props[fname]
                if fprop.get('format') == 'byte':
                    override = field_overrides.get(fname, {})
                    if isinstance(override, dict) and override.get('format') == 'bytes':
                        continue
                    issues.append((schema_name, fname))

        if issues:
            total_issues += len(issues)
            print(f"  {api_name}: {len(issues)} unhandled format:\"byte\" field(s)")
            for schema_name, fname in issues:
                print(f"    {schema_name}.{fname}")
        else:
            print(f"  {api_name}: OK")

    print(f"\nTotal: {total_issues} unhandled format:\"byte\" field(s)")


def cmd_diff(manifest: dict, discovery: dict, api_name: str):
    """Diff manifest against discovery doc to detect upstream API changes.

    Reports:
    - Schemas referenced by manifest types that have changed fields
    - Fields in manifest include_fields that no longer exist upstream
    - New fields added upstream to schemas we already use
    - Operations in manifest whose discovery method signatures changed
    - New methods added to resources we already use
    """
    schemas = discovery.get('schemas', {})
    all_methods = collect_all_methods(discovery)
    all_method_map = {m['dotted']: m for m in all_methods}

    changes = {
        'missing_schemas': [],
        'missing_fields': [],
        'new_fields': [],
        'field_type_changes': [],
        'missing_methods': [],
        'new_methods': [],
        'signature_changes': [],
    }

    # --- Check types ---
    for type_conf in manifest.get('types', []):
        schema_name = type_conf['schema']
        schema = schemas.get(schema_name)

        if not schema:
            changes['missing_schemas'].append(schema_name)
            continue

        props = schema.get('properties', {})
        include_fields = type_conf.get('include_fields', [])

        # Fields in manifest that no longer exist in discovery
        for fname in include_fields:
            if fname not in props:
                changes['missing_fields'].append((schema_name, fname))

        # New fields added upstream that aren't in our include list
        if include_fields:  # Only if we're using a whitelist
            for fname in sorted(props.keys()):
                if fname not in include_fields:
                    fprop = props[fname]
                    ftype = fprop.get('type', fprop.get('$ref', '?'))
                    fmt = fprop.get('format', '')
                    ann = f"{ftype}"
                    if fmt:
                        ann += f", format: {fmt}"
                    if 'enum' in fprop:
                        ann += f", enum"
                    changes['new_fields'].append((schema_name, fname, ann))

    # --- Check operations ---
    manifest_resources = set()
    for op_conf in manifest.get('operations', []):
        resource = op_conf.get('discovery_resource', '')
        method_key = op_conf.get('discovery_method', '')
        rust_name = op_conf.get('rust_name', '')

        # Build dotted path
        if resource and '.' not in method_key:
            dotted = f"{resource}.{method_key}"
        else:
            dotted = method_key

        if resource:
            manifest_resources.add(resource)

        # Check if the method still exists
        if dotted not in all_method_map:
            changes['missing_methods'].append((dotted, rust_name))
            continue

        # Check for signature changes (query params drift)
        method_data = all_method_map[dotted]['data']
        manifest_qps = set(op_conf.get('query_params', []))
        discovery_qps = {k for k, v in method_data.get('parameters', {}).items()
                         if v.get('location') == 'query'}
        new_qps = discovery_qps - manifest_qps
        removed_qps = manifest_qps - discovery_qps

        if removed_qps:
            changes['signature_changes'].append(
                (dotted, rust_name, f"query params removed upstream: {', '.join(sorted(removed_qps))}")
            )
        # Only flag new query params if the manifest uses query_params at all
        if new_qps and manifest_qps:
            changes['signature_changes'].append(
                (dotted, rust_name, f"new query params available: {', '.join(sorted(new_qps))}")
            )

    # --- Check for new methods on resources we already use ---
    for m in all_methods:
        resource = m['resource']
        dotted = m['dotted']
        if resource in manifest_resources:
            manifest_ops = get_manifest_operations(manifest)
            if dotted not in manifest_ops:
                http = m['data'].get('httpMethod', '?')
                changes['new_methods'].append((dotted, http))

    # --- Print report ---
    has_changes = any(v for v in changes.values())

    if not has_changes:
        print(f"  {api_name}: No changes detected between manifest and discovery doc.")
        return False

    print(f"  {api_name}: Changes detected\n")

    if changes['missing_schemas']:
        print(f"  REMOVED SCHEMAS (no longer in discovery doc):")
        for s in changes['missing_schemas']:
            print(f"    - {s}")
        print()

    if changes['missing_fields']:
        print(f"  REMOVED FIELDS (no longer in discovery doc):")
        for schema, field in changes['missing_fields']:
            print(f"    - {schema}.{field}")
        print()

    if changes['missing_methods']:
        print(f"  REMOVED METHODS (no longer in discovery doc):")
        for dotted, rust_name in changes['missing_methods']:
            print(f"    - {dotted} (rust: {rust_name})")
        print()

    if changes['signature_changes']:
        print(f"  SIGNATURE CHANGES:")
        for dotted, rust_name, detail in changes['signature_changes']:
            print(f"    - {dotted} (rust: {rust_name}): {detail}")
        print()

    if changes['new_fields']:
        print(f"  NEW FIELDS (added upstream to schemas we use):")
        for schema, field, ann in changes['new_fields']:
            print(f"    + {schema}.{field} ({ann})")
        print()

    if changes['new_methods']:
        print(f"  NEW METHODS (on resources we already use):")
        for dotted, http in changes['new_methods']:
            print(f"    + {dotted} ({http})")
        print()

    return True


def cmd_diff_all():
    """Diff all manifests against their discovery docs."""
    any_changes = False
    for manifest_path in sorted(MANIFESTS_DIR.glob("*.toml")):
        with open(manifest_path, 'rb') as f:
            manifest = tomllib.load(f)
        api_name = manifest['api']['name']
        version = manifest['api'].get('version', 'v1')
        discovery = load_discovery(api_name, version)
        if not discovery:
            print(f"  {api_name}: Discovery doc not cached, skipping")
            continue
        if cmd_diff(manifest, discovery, api_name):
            any_changes = True

    if not any_changes:
        print("\nAll manifests are in sync with discovery docs.")


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    # Handle global commands (no API arg needed)
    if sys.argv[1] == '--audit-all':
        cmd_audit_all()
        return
    if sys.argv[1] == '--diff-all':
        cmd_diff_all()
        return

    api_arg = sys.argv[1]
    if len(sys.argv) < 3:
        print(f"Usage: python3 codegen/extend.py {api_arg} <command>")
        print(f"  Commands: --available-types, --available-ops, --add-type <Name>,")
        print(f"            --add-op <path>, --audit, --diff")
        sys.exit(1)

    command = sys.argv[2]

    # Load manifest
    manifest = load_manifest(api_arg)
    if not manifest:
        print(f"ERROR: No manifest found for '{api_arg}'", file=sys.stderr)
        available = [p.stem for p in MANIFESTS_DIR.glob("*.toml")]
        print(f"  Available: {', '.join(sorted(available))}", file=sys.stderr)
        sys.exit(1)

    # Load discovery doc
    version = manifest['api'].get('version', 'v1')
    discovery = load_discovery(manifest['api']['name'], version)
    if not discovery:
        print(f"ERROR: Discovery doc not cached for '{api_arg}'", file=sys.stderr)
        print(f"  Run: task codegen:fetch", file=sys.stderr)
        sys.exit(1)

    if command == '--available-types':
        cmd_available_types(manifest, discovery)
    elif command == '--available-ops':
        cmd_available_ops(manifest, discovery)
    elif command == '--add-type':
        if len(sys.argv) < 4:
            print("Usage: python3 codegen/extend.py {api} --add-type <SchemaName>", file=sys.stderr)
            sys.exit(1)
        cmd_add_type(manifest, discovery, sys.argv[3])
    elif command == '--add-op':
        if len(sys.argv) < 4:
            print("Usage: python3 codegen/extend.py {api} --add-op <resource.method>", file=sys.stderr)
            sys.exit(1)
        cmd_add_op(manifest, discovery, sys.argv[3])
    elif command == '--audit':
        cmd_audit(manifest, discovery, manifest['api']['name'])
    elif command == '--diff':
        cmd_diff(manifest, discovery, manifest['api']['name'])
    else:
        print(f"Unknown command: {command}", file=sys.stderr)
        print(f"  Commands: --available-types, --available-ops, --add-type, --add-op, --audit, --diff", file=sys.stderr)
        sys.exit(1)


if __name__ == '__main__':
    main()
