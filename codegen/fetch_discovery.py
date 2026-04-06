#!/usr/bin/env python3
"""Fetch Google Discovery API documents into the local cache.

Usage:
    # Fetch all discovery docs referenced by manifests:
    python3 codegen/fetch_discovery.py

    # Fetch (or refresh) a specific API by name + version:
    python3 codegen/fetch_discovery.py compute v1
    python3 codegen/fetch_discovery.py storage v1
    python3 codegen/fetch_discovery.py container v1

    # Force-refresh all cached docs:
    python3 codegen/fetch_discovery.py --refresh
"""

import json
import sys
import urllib.request
from pathlib import Path

CACHE_DIR = Path(__file__).parent / "discovery_cache"
MANIFESTS_DIR = Path(__file__).parent / "manifests"

# Standard Google discovery URL pattern (covers most APIs).
STANDARD_URL = "https://www.googleapis.com/discovery/v1/apis/{name}/{version}/rest"


def fetch_doc(url: str, dest: Path):
    """Download a discovery doc and pretty-print it into dest."""
    print(f"  Downloading {url}")
    req = urllib.request.Request(url, headers={"Accept": "application/json"})
    with urllib.request.urlopen(req) as resp:
        data = json.loads(resp.read())
    dest.parent.mkdir(parents=True, exist_ok=True)
    with open(dest, "w") as f:
        json.dump(data, f, indent=2)
    schemas = len(data.get("schemas", {}))
    print(f"  -> {dest.name}  ({schemas} schemas)")


def fetch_from_manifests(refresh: bool = False):
    """Walk codegen/manifests/*.toml and fetch any missing (or all if refresh) docs."""
    import tomllib

    manifests = sorted(MANIFESTS_DIR.glob("*.toml"))
    if not manifests:
        print("No manifests found in codegen/manifests/")
        return

    for mpath in manifests:
        with open(mpath, "rb") as f:
            manifest = tomllib.load(f)
        api = manifest["api"]
        name = api["name"]
        version = api.get("version", "v1")
        url = api["discovery_url"]
        dest = CACHE_DIR / f"{name}.{version}.json"

        if dest.exists() and not refresh:
            print(f"  {dest.name} already cached (use --refresh to re-download)")
            continue

        fetch_doc(url, dest)


def fetch_by_name(api_name: str, version: str = "v1"):
    """Fetch a discovery doc by API name using the standard URL pattern."""
    dest = CACHE_DIR / f"{api_name}.{version}.json"
    url = STANDARD_URL.format(name=api_name, version=version)
    fetch_doc(url, dest)


def list_available():
    """Print some commonly-used GCP APIs you can fetch."""
    apis = [
        ("compute",        "v1",  "Compute Engine"),
        ("storage",        "v1",  "Cloud Storage"),
        ("container",      "v1",  "GKE / Kubernetes Engine"),
        ("sqladmin",       "v1",  "Cloud SQL Admin"),
        ("dns",            "v1",  "Cloud DNS"),
        ("iam",            "v1",  "IAM"),
        ("logging",        "v2",  "Cloud Logging"),
        ("monitoring",     "v3",  "Cloud Monitoring"),
        ("pubsub",         "v1",  "Pub/Sub"),
        ("cloudresourcemanager", "v3", "Resource Manager"),
        ("secretmanager",  "v1",  "Secret Manager"),
        ("run",            "v2",  "Cloud Run"),
        ("cloudfunctions", "v2",  "Cloud Functions"),
        ("cloudbuild",     "v1",  "Cloud Build"),
        ("cloudscheduler", "v1",  "Cloud Scheduler"),
        ("bigquery",       "v2",  "BigQuery"),
        ("redis",          "v1",  "Memorystore / Redis"),
        ("firestore",      "v1",  "Firestore"),
        ("spanner",        "v1",  "Cloud Spanner"),
    ]
    print("Common GCP APIs (pass name + version to fetch):\n")
    for name, ver, desc in apis:
        cached = (CACHE_DIR / f"{name}.{ver}.json").exists()
        tag = " [cached]" if cached else ""
        print(f"  {name:<28} {ver}   {desc}{tag}")
    print(f"\nUsage: python3 codegen/fetch_discovery.py <name> [version]")


def main():
    args = sys.argv[1:]

    if not args:
        print("Fetching discovery docs for all manifests...\n")
        fetch_from_manifests(refresh=False)
        return

    if args[0] == "--refresh":
        print("Refreshing all manifest discovery docs...\n")
        fetch_from_manifests(refresh=True)
        return

    if args[0] == "--list":
        list_available()
        return

    api_name = args[0]
    version = args[1] if len(args) > 1 else "v1"
    fetch_by_name(api_name, version)


if __name__ == "__main__":
    main()
