"""GCP provider plugin — resolves GCP Discovery Documents + TOML manifests into IR.

Ports the GCP-specific logic from the original codegen/generate.py into the
ProviderPlugin interface. All discovery doc parsing, $ref resolution, and
manifest interpretation lives here.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

try:
    import tomllib  # Python 3.11+
except ImportError:
    try:
        import tomli as tomllib  # type: ignore[import-not-found]
    except ImportError:
        print("ERROR: Need Python 3.11+ (tomllib) or `pip install tomli`", file=sys.stderr)
        sys.exit(1)

from cloud_lite_codegen.ir import (
    ApiArgsConfig,
    ApiDef,
    ClientConfig,
    EnumDef,
    EnumVariant,
    FieldDef,
    FieldFormat,
    HttpMethod,
    ListResponseConfig,
    LroConfig,
    OperationDef,
    PathParam,
    ProviderDef,
    QueryParam,
    TypeDef,
)
from cloud_lite_codegen.plugin import ProviderPlugin


# ---------------------------------------------------------------------------
# Helpers (ported from generate.py)
# ---------------------------------------------------------------------------

def _to_snake_case(name: str) -> str:
    """Convert camelCase or PascalCase to snake_case."""
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', s)
    s = re.sub(r'IP([A-Z])', r'Ip\1', s)
    return s.lower()


def _to_pascal_case(name: str) -> str:
    """Convert SCREAMING_SNAKE_CASE to PascalCase."""
    return ''.join(word.capitalize() for word in name.split('_'))


def _extract_path_param_order(path: str) -> list[str]:
    """Extract path parameter names in the order they appear in the URL."""
    return re.findall(r'\{[+]?([^}]+)\}', path)


RESERVED_WORDS = frozenset(('type', 'match', 'self', 'ref', 'mod', 'use'))


# ---------------------------------------------------------------------------
# Discovery doc method lookup
# ---------------------------------------------------------------------------

def _find_method(discovery: dict, resource_name: str, method_name: str) -> dict | None:
    """Find a method in the discovery document.

    Handles several call patterns:
    - resource_name="disks", method_name="insert" (simple)
    - resource_name="", method_name="projects.serviceAccounts.create" (dotted)
    - resource_name="projects.serviceAccounts", method_name="create" (nested)
    """
    resources = discovery.get('resources', {})

    full_path = resource_name
    if not full_path and '.' in method_name:
        full_path = method_name
    elif full_path and method_name and '.' not in full_path:
        full_path = f"{full_path}.{method_name}"
    elif '.' in full_path and method_name:
        full_path = f"{full_path}.{method_name}"

    if '.' in full_path:
        parts = full_path.split('.')
        mname = parts[-1]
        resource_path = parts[:-1]
        current = resources
        for part in resource_path:
            if part in current:
                current = current[part]
                if part != resource_path[-1]:
                    current = current.get('resources', {})
        methods = current.get('methods', {})
        return methods.get(mname)

    if resource_name in resources:
        methods = resources[resource_name].get('methods', {})
        return methods.get(method_name)

    return None


# ---------------------------------------------------------------------------
# Type resolution (ported from TypeResolver class)
# ---------------------------------------------------------------------------

class _TypeResolver:
    """Resolves discovery doc $ref names to Rust type names and collects dependencies."""

    def __init__(self, manifest: dict, discovery: dict):
        self.discovery = discovery
        self.schemas = discovery.get('schemas', {})
        self.manifest = manifest

        # Build a map: discovery schema name -> rust type name
        self.name_map: dict[str, str] = {}
        for type_conf in manifest.get('types', []):
            schema = type_conf['schema']
            rust_name = type_conf.get('rust_name', schema)
            self.name_map[schema] = rust_name

        # Track which schemas we need to auto-generate (discovered via $ref)
        self.auto_schemas: dict[str, dict] = {}

        # Track inline structs to generate (from inline_struct field overrides)
        self.inline_structs: list[tuple[str, dict]] = []

    def resolve_ref(self, ref_name: str) -> str:
        """Get the Rust name for a discovery $ref."""
        if ref_name in self.name_map:
            return self.name_map[ref_name]
        return ref_name

    def rust_type_for(self, prop: dict, enums_map: dict, field_name: str,
                      field_overrides: dict | None = None) -> str:
        """Convert a discovery doc property to a Rust type string."""
        if field_name in enums_map:
            return enums_map[field_name]

        # Check for inline_struct override
        if field_overrides and field_name in field_overrides:
            override = field_overrides[field_name]
            if isinstance(override, dict) and 'inline_struct' in override:
                struct_name = override['inline_struct']
                if prop.get('type') == 'array':
                    items = prop.get('items', {})
                    if items.get('type') == 'object' and 'properties' in items:
                        self.inline_structs.append((struct_name, items['properties']))
                    return f'Vec<{struct_name}>'
                elif prop.get('type') == 'object' and 'properties' in prop:
                    self.inline_structs.append((struct_name, prop['properties']))
                    return struct_name

        ptype = prop.get('type', '')
        fmt = prop.get('format', '')
        ref = prop.get('$ref', '')

        if ref:
            return self.resolve_ref(ref)
        elif ptype == 'string':
            return 'String'
        elif ptype == 'boolean':
            return 'bool'
        elif ptype == 'integer':
            if fmt == 'int32':
                return 'i32'
            elif fmt == 'uint32':
                return 'u32'
            else:
                return 'i64'
        elif ptype == 'number':
            return 'f32' if fmt == 'float' else 'f64'
        elif ptype == 'array':
            items = prop.get('items', {})
            inner = self.rust_type_for(items, {}, '')
            return f'Vec<{inner}>'
        elif ptype == 'object':
            add_props = prop.get('additionalProperties', {})
            if add_props:
                val_type = self.rust_type_for(add_props, {}, '')
                return f'HashMap<String, {val_type}>'
            return 'serde_json::Value'
        else:
            return 'serde_json::Value'

    def collect_refs(self, type_conf: dict) -> set[str]:
        """Collect all $ref dependencies for a type config."""
        schema_name = type_conf['schema']
        schema = self.schemas.get(schema_name, {})
        props = schema.get('properties', {})
        include_fields = type_conf.get('include_fields', None)
        field_overrides = type_conf.get('field_overrides', {})
        refs: set[str] = set()

        fields = include_fields if include_fields else list(props.keys())
        for fname in fields:
            override = field_overrides.get(fname, {})
            if isinstance(override, dict) and override.get('rust_type'):
                continue
            prop = props.get(fname, {})
            ref = prop.get('$ref', '')
            if ref:
                refs.add(ref)
            elif prop.get('type') == 'array':
                items = prop.get('items', {})
                ref = items.get('$ref', '')
                if ref:
                    refs.add(ref)
        return refs


# ---------------------------------------------------------------------------
# Field resolution — manifest type -> list[FieldDef]
# ---------------------------------------------------------------------------

def _resolve_fields(type_conf: dict, schema: dict, resolver: _TypeResolver) -> list[FieldDef]:
    """Resolve a manifest type config + discovery schema into a list of FieldDef."""
    props = schema.get('properties', {})
    field_overrides = type_conf.get('field_overrides', {})
    include_fields = type_conf.get('include_fields', None)

    enums_map: dict[str, str] = {}
    for fname, override in field_overrides.items():
        if isinstance(override, dict) and 'enum_type' in override:
            enums_map[fname] = override['enum_type']

    if include_fields is not None:
        fields_to_gen = [(f, props.get(f, {})) for f in include_fields if f in props]
    else:
        fields_to_gen = list(props.items())

    result: list[FieldDef] = []
    for field_name, prop in fields_to_gen:
        override = field_overrides.get(field_name, {})
        if isinstance(override, str):
            override = {}

        rust_name = _to_snake_case(override.get('rust_name', field_name))
        serde_rename = override.get('serde_rename', '')
        is_required = override.get('required', False)
        is_bytes_format = override.get('format') == 'bytes'

        field_desc = prop.get('description', '')
        is_output_only = 'Output only' in field_desc or 'output only' in field_desc

        enum_type_name = override.get('enum_type', '')
        explicit_rust_type = override.get('rust_type', '')

        if explicit_rust_type:
            rust_type = explicit_rust_type
        elif enum_type_name:
            rust_type = enum_type_name
        else:
            rust_type = resolver.rust_type_for(prop, enums_map, field_name, field_overrides)

        # When rust_type is explicitly overridden, don't infer is_repeated/is_map
        # from the discovery schema — the user chose a specific type.
        if explicit_rust_type:
            is_repeated = rust_type.startswith('Vec<')
            is_map = rust_type.startswith('HashMap<')
            map_value_type = ''
        else:
            is_repeated = prop.get('type') == 'array' or rust_type.startswith('Vec<')
            is_map = prop.get('type') == 'object' and 'additionalProperties' in prop
            map_value_type = ''
            if is_map:
                add_props = prop.get('additionalProperties', {})
                map_value_type = resolver.rust_type_for(add_props, {}, '')

        # Determine format
        fmt = FieldFormat.NONE
        if is_bytes_format:
            fmt = FieldFormat.BYTES

        # Warn if discovery doc has format:"byte" but manifest doesn't override
        disc_format = prop.get('format', '')
        if disc_format == 'byte' and fmt != FieldFormat.BYTES:
            schema_name = type_conf.get('schema', '?')
            print(f"  WARNING: {schema_name}.{field_name} has format:\"byte\" in discovery but no format=\"bytes\" override")

        # Inline enum values for doc comments
        enum_values: list[str] = []
        enum_descriptions: list[str] = []
        if 'enum' in prop:
            enum_values = prop['enum']
            enum_descriptions = prop.get('enumDescriptions', [])

        # Inline struct
        inline_struct = ''
        if isinstance(override, dict) and 'inline_struct' in override:
            inline_struct = override['inline_struct']

        result.append(FieldDef(
            name=field_name,
            rust_name=rust_name,
            rust_type=rust_type,
            required=is_required,
            repeated=is_repeated,
            format=fmt,
            is_map=is_map,
            map_value_type=map_value_type,
            enum_type=enum_type_name,
            inline_struct=inline_struct,
            description=field_desc,
            is_output_only=is_output_only,
            serde_rename=serde_rename,
            enum_values=enum_values,
            enum_descriptions=enum_descriptions,
            has_explicit_type=bool(explicit_rust_type),
        ))

    return result


def _resolve_auto_fields(schema: dict, resolver: _TypeResolver) -> list[FieldDef]:
    """Resolve all fields from a discovery schema for auto-dependency types."""
    props = schema.get('properties', {})
    result: list[FieldDef] = []

    for field_name, prop in props.items():
        rust_type = resolver.rust_type_for(prop, {}, field_name)
        field_desc = prop.get('description', '')

        # Handle reserved words
        needs_rename = field_name in RESERVED_WORDS
        if needs_rename:
            rust_name = f"{field_name}_value"
            serde_rename = field_name
        else:
            rust_name = _to_snake_case(field_name)
            serde_rename = ''

        is_repeated = prop.get('type') == 'array' or rust_type.startswith('Vec<')
        is_map = prop.get('type') == 'object' and 'additionalProperties' in prop
        map_value_type = ''
        if is_map:
            add_props = prop.get('additionalProperties', {})
            map_value_type = resolver.rust_type_for(add_props, {}, '')

        result.append(FieldDef(
            name=field_name,
            rust_name=rust_name,
            rust_type=rust_type,
            description=field_desc,
            repeated=is_repeated,
            is_map=is_map,
            map_value_type=map_value_type,
            serde_rename=serde_rename,
        ))

    return result


# ---------------------------------------------------------------------------
# Enum extraction
# ---------------------------------------------------------------------------

def _extract_enums(type_conf: dict, schema: dict, api_name: str, version: str) -> list[EnumDef]:
    """Extract EnumDef instances from field_overrides that declare enum_type.

    Iterates in include_fields order (or properties order if no include_fields)
    to match the field processing order used by the old pipeline.
    """
    props = schema.get('properties', {})
    field_overrides = type_conf.get('field_overrides', {})
    include_fields = type_conf.get('include_fields', None)
    schema_name = type_conf['schema']
    enums: list[EnumDef] = []

    # Use include_fields order if available, otherwise properties order
    if include_fields is not None:
        field_order = [f for f in include_fields if f in props]
    else:
        field_order = list(props.keys())

    for fname in field_order:
        override = field_overrides.get(fname, {})
        if not isinstance(override, dict):
            continue
        enum_type_name = override.get('enum_type', '')
        if not enum_type_name:
            continue

        prop = props.get(fname, {})
        if 'enum' not in prop:
            continue

        values = prop['enum']
        descriptions = prop.get('enumDescriptions', [])
        all_screaming = all(v == v.upper() for v in values if v)

        variants: list[EnumVariant] = []
        for i, val in enumerate(values):
            desc = descriptions[i] if i < len(descriptions) else ""
            if all_screaming:
                rust_variant = _to_pascal_case(val)
            else:
                rust_variant = val
            variants.append(EnumVariant(
                api_name=val,
                rust_name=rust_variant,
                description=desc,
            ))

        enums.append(EnumDef(
            name=enum_type_name,
            variants=variants,
            all_screaming=all_screaming,
            has_unknown=True,
            api_path=f"{api_name}.{version}.{schema_name}.{fname}",
        ))

    return enums


# ---------------------------------------------------------------------------
# Operation resolution
# ---------------------------------------------------------------------------

def _resolve_operation(op_conf: dict, discovery: dict, resolver: _TypeResolver,
                       api_name: str, base_url: str) -> OperationDef | None:
    """Resolve a manifest [[operations]] entry into an OperationDef."""
    rust_name = op_conf['rust_name']
    resource = op_conf.get('discovery_resource', '')
    method_key = op_conf.get('discovery_method', '')

    if not resource and '.' in method_key:
        method_data = _find_method(discovery, method_key, '')
    else:
        method_data = _find_method(discovery, resource, method_key)

    if not method_data:
        print(f"  WARNING: Cannot find method '{resource}.{method_key}' for '{rust_name}'",
              file=sys.stderr)
        return None

    http_method_str = method_data.get('httpMethod', 'GET')
    http_method = HttpMethod(http_method_str)
    path = method_data.get('path', '')
    description = method_data.get('description', '')
    request_ref = method_data.get('request', {}).get('$ref', '')
    response_ref = method_data.get('response', {}).get('$ref', '')
    params = method_data.get('parameters', {})

    disc_query_params = {k: v for k, v in params.items() if v.get('location') == 'query'}

    # Path params in URL order
    param_order = _extract_path_param_order(path)
    plus_params = set(re.findall(r'\{\+([^}]+)\}', path))

    path_params: list[PathParam] = []
    for pname in param_order:
        pval = params.get(pname, {})
        path_params.append(PathParam(
            name=pname,
            rust_name=_to_snake_case(pname),
            rust_type="&str",
            passthrough=(pname in plus_params),
            description=pval.get('description', ''),
        ))

    # Query params from manifest
    manifest_qps = op_conf.get('query_params', [])
    query_params: list[QueryParam] = []
    for qname in manifest_qps:
        qval = disc_query_params.get(qname, {})
        is_repeated = qval.get('repeated', False)
        query_params.append(QueryParam(
            name=qname,
            rust_name=_to_snake_case(qname),
            rust_type="&[&str]" if is_repeated else "&str",
            required=qval.get('required', False),
            repeated=is_repeated,
            description=qval.get('description', ''),
        ))

    # ALL query params from discovery doc (for doc comments), sorted alphabetically
    all_query_params: list[QueryParam] = []
    for qname in sorted(disc_query_params.keys()):
        qval = disc_query_params[qname]
        is_repeated = qval.get('repeated', False)
        all_query_params.append(QueryParam(
            name=qname,
            rust_name=_to_snake_case(qname),
            rust_type="&[&str]" if is_repeated else "&str",
            required=qval.get('required', False),
            repeated=is_repeated,
            description=qval.get('description', ''),
        ))

    # Resolve request/response types
    request_body_type = resolver.resolve_ref(request_ref) if request_ref else ""
    response_type = resolver.resolve_ref(response_ref) if response_ref else ""

    # List response config
    lr_conf = op_conf.get('list_response')
    list_response: ListResponseConfig | None = None
    if lr_conf:
        list_response = ListResponseConfig(
            type_name=lr_conf['type_name'],
            items_field=lr_conf['items_field'],
            item_type=lr_conf['item_type'],
        )

    return OperationDef(
        name=rust_name,
        http_method=http_method,
        url_template=path,
        path_params=path_params,
        query_params=query_params,
        request_body_type=request_body_type,
        response_type=response_type,
        is_lro=op_conf.get('is_lro', False),
        description=description,
        list_response=list_response,
        base_url_override=op_conf.get('base_url_override', ''),
        discovery_resource=resource,
        discovery_method=method_key,
        all_query_params=all_query_params,
    )


# ---------------------------------------------------------------------------
# GcpPlugin
# ---------------------------------------------------------------------------

class GcpPlugin(ProviderPlugin):
    """GCP provider plugin — resolves GCP Discovery Documents into IR."""

    def __init__(self, manifests_dir: str | None = None, cache_dir: str | None = None):
        codegen_dir = Path(__file__).parent
        self._manifests_dir = Path(manifests_dir) if manifests_dir else codegen_dir / "manifests"
        self._cache_dir = Path(cache_dir) if cache_dir else codegen_dir / "discovery_cache"

    def name(self) -> str:
        return "gcp"

    def target_crate(self) -> str:
        return "."

    def resolve(self, manifest_path: str) -> ApiDef:
        """Resolve a single TOML manifest + its discovery doc into an ApiDef."""
        with open(manifest_path, 'rb') as f:
            manifest = tomllib.load(f)

        api_config = manifest['api']
        api_name = api_config['name']
        version = api_config.get('version', 'v1')
        base_url = api_config.get('base_url', '')
        doc_url = api_config.get('doc_url', '')

        # Load discovery doc
        discovery = self._load_discovery(api_name, version, api_config.get('discovery_url', ''))
        schemas = discovery.get('schemas', {})

        resolver = _TypeResolver(manifest, discovery)

        # --- Resolve types ---
        types: list[TypeDef] = []
        enums: list[EnumDef] = []
        manifest_schemas: set[str] = set()

        lro_config = api_config.get('lro', {})
        lro_response_type = lro_config.get('response_type', '')

        for type_conf in manifest.get('types', []):
            schema_name = type_conf['schema']
            manifest_schemas.add(schema_name)
            schema = schemas.get(schema_name, {})
            if not schema:
                print(f"  WARNING: Schema '{schema_name}' not found in discovery doc",
                      file=sys.stderr)
                continue

            rust_name = type_conf.get('rust_name', schema_name)
            include_fields = type_conf.get('include_fields', None)
            omitted = type_conf.get('omitted', {})
            props = schema.get('properties', {})

            # Resolve fields
            fields = _resolve_fields(type_conf, schema, resolver)

            # Coverage info
            total_fields = len(props)
            included_fields = len([f for f in (include_fields or list(props.keys())) if f in props])

            # Fixture overrides — discovery `type: "object"` fields generate
            # Default::default() in the old pipeline. The emitter only knows Rust types,
            # so fields whose Rust type is a struct name (inline struct, $ref dependency)
            # or serde_json::Value override need an explicit fixture hint.
            # HashMap and Vec types are already handled by the emitter's _fixture_value().
            fixture_overrides: dict[str, str] = {}
            for field_def in fields:
                prop = props.get(field_def.name, {})
                disc_type = prop.get('type', '')
                if disc_type == 'object' \
                        and not field_def.rust_type.startswith('HashMap<') \
                        and not field_def.rust_type.startswith('Vec<'):
                    rust_field = field_def.rust_name or _to_snake_case(field_def.name)
                    fixture_overrides[rust_field] = 'Default::default()'

            td = TypeDef(
                name=rust_name,
                schema_name=schema_name,
                fields=fields,
                description=schema.get('description', ''),
                api_path=f"{api_name}.{version}",
                doc_url=doc_url,
                total_fields=total_fields,
                included_fields=included_fields,
                omitted=omitted,
                fixture_overrides=fixture_overrides,
            )
            types.append(td)

            # Extract enums from this type
            enums.extend(_extract_enums(type_conf, schema, api_name, version))

        # --- Collect $ref auto-dependencies ---
        needed_refs: set[str] = set()
        for type_conf in manifest.get('types', []):
            refs = resolver.collect_refs(type_conf)
            for ref in refs:
                if ref not in manifest_schemas:
                    needed_refs.add(ref)

        # Recursively find refs of refs (one level)
        more_refs: set[str] = set()
        for ref_name in needed_refs:
            schema = schemas.get(ref_name, {})
            for _fname, prop in schema.get('properties', {}).items():
                r = prop.get('$ref', '')
                if r and r not in manifest_schemas and r not in needed_refs:
                    more_refs.add(r)
                if prop.get('type') == 'array':
                    items = prop.get('items', {})
                    r = items.get('$ref', '')
                    if r and r not in manifest_schemas and r not in needed_refs:
                        more_refs.add(r)
        needed_refs.update(more_refs)

        auto_types: list[TypeDef] = []
        for ref_name in sorted(needed_refs):
            schema = schemas.get(ref_name, {})
            if schema:
                fields = _resolve_auto_fields(schema, resolver)
                auto_types.append(TypeDef(
                    name=resolver.resolve_ref(ref_name),
                    schema_name=ref_name,
                    fields=fields,
                    description=schema.get('description', ''),
                    api_path=f"{api_name}.{version}",
                    is_auto_dependency=True,
                ))

        # --- Inline struct types ---
        inline_types: list[TypeDef] = []
        for struct_name, properties in resolver.inline_structs:
            inline_fields: list[FieldDef] = []
            for fname, prop in properties.items():
                rust_type = resolver.rust_type_for(prop, {}, fname)
                needs_rename = fname in RESERVED_WORDS
                snake = _to_snake_case(fname)
                # Detect camelCase round-trip mismatch (e.g. IPProtocol -> ip_protocol -> ipProtocol != IPProtocol)
                parts = snake.split('_')
                camel_roundtrip = parts[0] + ''.join(p.capitalize() for p in parts[1:]) if len(parts) > 1 else parts[0]
                needs_serde_rename = needs_rename or (camel_roundtrip != fname)
                inline_fields.append(FieldDef(
                    name=fname,
                    rust_name=f"{fname}_value" if needs_rename else snake,
                    rust_type=rust_type,
                    description=prop.get('description', ''),
                    serde_rename=fname if needs_serde_rename else '',
                ))
            inline_types.append(TypeDef(
                name=struct_name,
                fields=inline_fields,
                api_path=f"{api_name}.{version}",
                is_inline_struct=True,
            ))

        # --- Resolve operations ---
        operations: list[OperationDef] = []
        list_response_types: list[TypeDef] = []
        seen_list_types: set[str] = {td.name for td in types}
        api_args_map: dict[str, ApiArgsConfig] = {}

        for op_conf in manifest.get('operations', []):
            op = _resolve_operation(op_conf, discovery, resolver, api_name, base_url)
            if op:
                operations.append(op)

                # Collect list response types
                if op.list_response and op.list_response.type_name not in seen_list_types:
                    seen_list_types.add(op.list_response.type_name)
                    list_response_types.append(TypeDef(
                        name=op.list_response.type_name,
                        is_list_response=True,
                        list_items_field=op.list_response.items_field,
                        list_item_type=op.list_response.item_type,
                    ))

                # Collect api_args config
                op_api_args = op_conf.get('api_args')
                op_arg_mapping = op_conf.get('arg_mapping', {})
                if op_api_args:
                    api_args_map[op.name] = ApiArgsConfig(
                        api_args=op_api_args,
                        arg_mapping=op_arg_mapping,
                    )

        # --- Build LRO config ---
        lro = LroConfig(
            pattern=lro_config.get('pattern', 'none'),
            response_type=lro_config.get('response_type', ''),
            poll_config=lro_config.get('poll_config', ''),
            operation_struct=lro_config.get('operation_struct', 'Operation'),
            base_url=lro_config.get('base_url', ''),
            poll_path=lro_config.get('poll_path', ''),
        )

        # --- Build client config ---
        client_config = api_config.get('client', {})
        client = ClientConfig(
            client_struct=client_config.get('client_struct', ''),
            accessor_name=client_config.get('accessor_name', api_name),
        )

        # --- Path helpers ---
        path_helpers = api_config.get('path_helpers', {})

        return ApiDef(
            name=api_name,
            display_name=api_config.get('display_name', api_name),
            version=version,
            base_url=base_url,
            doc_url=doc_url,
            discovery_url=api_config.get('discovery_url', ''),
            client=client,
            lro=lro,
            path_helpers=path_helpers,
            types=types,
            enums=enums,
            auto_types=auto_types,
            inline_types=inline_types,
            list_response_types=list_response_types,
            operations=operations,
            api_args=api_args_map,
        )

    def resolve_all(self) -> ProviderDef:
        """Resolve all manifests in the manifests directory into a ProviderDef."""
        apis: list[ApiDef] = []
        for manifest_path in sorted(self._manifests_dir.glob("*.toml")):
            apis.append(self.resolve(str(manifest_path)))
        return ProviderDef(
            provider="gcp",
            target_crate=".",
            client_struct="GcpHttpClient",
            apis=apis,
            rename_all="camelCase",
            wire_format="json",
            spec_source_name="the GCP Discovery Document",
            api_doc_label="GCP API",
            error_invalid_response="crate::GcpError::InvalidResponse",
            error_type="crate::GcpError",
            result_type="crate::Result",
        )

    def _load_discovery(self, api_name: str, version: str, url: str) -> dict:
        """Load a discovery doc from cache, downloading if needed."""
        cache_file = self._cache_dir / f"{api_name}.{version}.json"

        if not cache_file.exists():
            if url:
                import urllib.request
                print(f"  Downloading discovery doc from {url}...")
                self._cache_dir.mkdir(parents=True, exist_ok=True)
                urllib.request.urlretrieve(url, str(cache_file))
            else:
                raise FileNotFoundError(
                    f"Discovery cache file not found: {cache_file} and no URL to download from"
                )

        with open(cache_file) as f:
            return json.load(f)
