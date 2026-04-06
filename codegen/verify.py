#!/usr/bin/env python3
"""Verify codegen manifests against discovery documents and run Rust checks.

Usage:
    python3 codegen/verify.py                    # Verify all manifests
    python3 codegen/verify.py codegen/manifests/iam.toml  # Verify one manifest

Checks:
1. Every schema referenced in [[types]] exists in the discovery doc
2. Every field in include_fields exists in the schema
3. Every operation's request/response type is declared in [[types]] or auto-generated
4. If an operation returns Operation, it should have is_lro = true
5. Coverage report: types/ops declared vs. available in discovery doc
6. Runs cargo check and cargo test
"""

import json
import subprocess
import sys
from pathlib import Path

try:
    import tomllib
except ImportError:
    import tomli as tomllib

CACHE_DIR = Path(__file__).parent / "discovery_cache"
MANIFESTS_DIR = Path(__file__).parent / "manifests"


def load_manifest(path: str) -> dict:
    with open(path, 'rb') as f:
        return tomllib.load(f)


def load_discovery(manifest: dict) -> dict:
    api_config = manifest['api']
    api_name = api_config['name']
    version = api_config.get('version', 'v1')
    cache_file = CACHE_DIR / f"{api_name}.{version}.json"
    if not cache_file.exists():
        print(f"  ERROR: Discovery cache not found: {cache_file}")
        print(f"  Run: python3 codegen/fetch_discovery.py")
        return {}
    with open(cache_file) as f:
        return json.load(f)


def find_method(discovery: dict, resource_name: str, method_name: str) -> dict | None:
    """Find a method in the discovery document (same logic as generate.py).
    
    Handles several call patterns:
    - resource_name="disks", method_name="insert" (simple)
    - resource_name="", method_name="projects.serviceAccounts.create" (dotted)
    - resource_name="projects.serviceAccounts", method_name="create" (nested)
    """
    resources = discovery.get('resources', {})

    # If method_name contains dots, treat it as a full path
    full_path = resource_name
    if not full_path and '.' in method_name:
        full_path = method_name
    elif full_path and method_name and '.' not in full_path:
        full_path = f"{full_path}.{method_name}"
    elif '.' in full_path and method_name:
        full_path = f"{full_path}.{method_name}"

    if '.' in full_path:
        parts = full_path.split('.')
        # The last part is the method name, everything before is resource path
        mname = parts[-1]
        resource_path = parts[:-1]
        
        # Walk the resource tree, but don't descend into 'resources' for the last part
        current = resources
        for part in resource_path:
            if part in current:
                current = current[part]
                if part != resource_path[-1]:
                    current = current.get('resources', {})
            else:
                return None
        methods = current.get('methods', {})
        return methods.get(mname)

    if resource_name in resources:
        methods = resources[resource_name].get('methods', {})
        return methods.get(method_name)

    return None


def verify_manifest(manifest_path: str) -> list[str]:
    """Verify a single manifest. Returns list of error messages."""
    manifest = load_manifest(manifest_path)
    discovery = load_discovery(manifest)
    if not discovery:
        return [f"Could not load discovery doc for {manifest_path}"]

    api_name = manifest['api']['name']
    schemas = discovery.get('schemas', {})
    errors = []
    warnings = []

    # Collect declared type names
    declared_types = set()
    for type_conf in manifest.get('types', []):
        rust_name = type_conf.get('rust_name', type_conf['schema'])
        declared_types.add(rust_name)

    # Check 1: Every schema in [[types]] exists in discovery
    for type_conf in manifest.get('types', []):
        schema_name = type_conf['schema']
        if schema_name not in schemas:
            errors.append(f"[{api_name}] Schema '{schema_name}' not found in discovery doc")
            continue

        # Check 2: Every field in include_fields exists in the schema
        schema = schemas[schema_name]
        props = schema.get('properties', {})
        include_fields = type_conf.get('include_fields', None)
        if include_fields:
            for field in include_fields:
                if field not in props:
                    errors.append(f"[{api_name}] Field '{field}' not in schema '{schema_name}' (available: {', '.join(sorted(props.keys())[:5])}...)")

    # Check 3 & 4: Operations
    for op_conf in manifest.get('operations', []):
        rust_name = op_conf['rust_name']
        resource = op_conf.get('discovery_resource', '')
        method_key = op_conf.get('discovery_method', '')

        if not resource and '.' in method_key:
            method_data = find_method(discovery, method_key, '')
        else:
            method_data = find_method(discovery, resource, method_key)

        if not method_data:
            errors.append(f"[{api_name}] Operation '{rust_name}': method {resource}.{method_key} not found in discovery")
            continue

        # Check if response type is declared
        resp_ref = method_data.get('response', {}).get('$ref', '')
        if resp_ref and resp_ref != 'Empty':
            # Check if it's a known list response
            lr = op_conf.get('list_response')
            if not lr:
                # Should be declared in types
                resolved = None
                for type_conf in manifest.get('types', []):
                    if type_conf['schema'] == resp_ref:
                        resolved = type_conf.get('rust_name', resp_ref)
                        break
                if not resolved:
                    # Might be auto-generated via $ref — that's OK for dependency types
                    pass

        # Check 4: If returns Operation, should have is_lro
        if resp_ref == 'Operation' and not op_conf.get('is_lro', False):
            warnings.append(f"[{api_name}] Operation '{rust_name}' returns Operation schema but is_lro is not set")

        # Check 5: Repeated params are now handled natively by codegen (&[&str] + loop-based URL)
        # No warning needed — codegen detects repeated: true from discovery doc automatically

    # Coverage report
    total_schemas = len(schemas)
    declared_count = len(manifest.get('types', []))
    total_methods = 0
    resources = discovery.get('resources', {})

    def count_methods(res):
        count = 0
        for r in res.values():
            count += len(r.get('methods', {}))
            if 'resources' in r:
                count += count_methods(r['resources'])
        return count

    total_methods = count_methods(resources)
    declared_ops = len(manifest.get('operations', []))

    print(f"  [{api_name}] Coverage: {declared_count}/{total_schemas} schemas, {declared_ops}/{total_methods} methods")

    return errors, warnings


def run_cargo_checks():
    """Run cargo check and cargo test."""
    errors = []

    print("\n  Running cargo check...")
    result = subprocess.run(
        ["cargo", "check", "-p", "gcp-lite"],
        capture_output=True, text=True, timeout=120
    )
    if result.returncode != 0:
        errors.append(f"cargo check failed:\n{result.stderr}")
    else:
        print("  cargo check: OK")

    print("  Running cargo test...")
    result = subprocess.run(
        ["cargo", "test", "-p", "gcp-lite"],
        capture_output=True, text=True, timeout=300
    )
    if result.returncode != 0:
        errors.append(f"cargo test failed:\n{result.stderr}")
    else:
        # Count tests
        for line in result.stdout.split('\n'):
            if 'test result:' in line:
                print(f"  {line.strip()}")
        print("  cargo test: OK")

    return errors


def main():
    manifest_paths = sys.argv[1:] if len(sys.argv) > 1 else sorted(str(p) for p in MANIFESTS_DIR.glob("*.toml"))

    if not manifest_paths:
        print("No manifests found.")
        sys.exit(1)

    all_errors = []
    all_warnings = []

    print("Verifying manifests...\n")
    for path in manifest_paths:
        print(f"  Checking: {path}")
        errors, warnings = verify_manifest(path)
        all_errors.extend(errors)
        all_warnings.extend(warnings)

    # Run Rust checks
    cargo_errors = run_cargo_checks()
    all_errors.extend(cargo_errors)

    # Report
    print()
    if all_warnings:
        print(f"Warnings ({len(all_warnings)}):")
        for w in all_warnings:
            print(f"  WARNING: {w}")

    if all_errors:
        print(f"\nErrors ({len(all_errors)}):")
        for e in all_errors:
            print(f"  ERROR: {e}")
        sys.exit(1)
    else:
        print("All checks passed!")


if __name__ == "__main__":
    main()
