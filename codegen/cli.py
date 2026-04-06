"""GCP codegen CLI.

Usage:
    python3 -m codegen.cli apply          # Generate + apply
    python3 -m codegen.cli apply --api compute  # Single API
    python3 -m codegen.cli apply --dry-run      # Preview
    python3 -m codegen.cli generate             # Generate only
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from cloud_lite_codegen.emitter import RustEmitter
from cloud_lite_codegen.apply import apply
from codegen.plugin import GcpPlugin


def _resolve_provider(plugin, api: str | None):
    if api:
        manifest_path = Path("codegen") / "manifests" / f"{api}.toml"
        if not manifest_path.exists():
            print(f"ERROR: Manifest not found: {manifest_path}", file=sys.stderr)
            sys.exit(1)
        api_def = plugin.resolve(str(manifest_path))
        full_provider = plugin.resolve_all()
        full_provider.apis = [api_def]
        return full_provider
    return plugin.resolve_all()


def cmd_generate(args):
    print("=== Generating: gcp ===")
    plugin = GcpPlugin()
    provider_def = _resolve_provider(plugin, args.api)
    output_dir = str(Path("codegen") / "generated")
    emitter = RustEmitter(output_dir)
    emitter.emit(provider_def)
    print(f"  Generated to: {output_dir}")


def cmd_apply(args):
    print("=== Processing: gcp ===")
    plugin = GcpPlugin()
    provider_def = _resolve_provider(plugin, args.api)
    output_dir = str(Path("codegen") / "generated")
    print(f"  Generating to: {output_dir}")
    emitter = RustEmitter(output_dir)
    emitter.emit(provider_def)
    print(f"  Applying to: {plugin.target_crate()}/src/")
    actions = apply(output_dir, plugin.target_crate(), dry_run=args.dry_run)
    if args.dry_run:
        print("  [dry-run] Would perform:")
    for action in actions:
        print(f"    {action}")
    print("\nDone!")


def main():
    parser = argparse.ArgumentParser(prog="gcp-codegen", description="GCP Rust codegen")
    sub = parser.add_subparsers(dest="command")
    gen_p = sub.add_parser("generate")
    gen_p.add_argument("--api")
    apply_p = sub.add_parser("apply")
    apply_p.add_argument("--api")
    apply_p.add_argument("--dry-run", action="store_true")
    args = parser.parse_args()
    if args.command == "generate":
        cmd_generate(args)
    elif args.command == "apply":
        cmd_apply(args)
    else:
        parser.print_help()
        sys.exit(1)


if __name__ == "__main__":
    main()
