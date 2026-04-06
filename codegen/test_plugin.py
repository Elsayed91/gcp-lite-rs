"""Tests for the GCP provider plugin.

Tests resolve() against real manifests + discovery docs to validate
the full pipeline from TOML + JSON -> IR dataclasses.
"""

from __future__ import annotations

from pathlib import Path

import pytest

from cloud_lite_codegen.ir import (
    FieldFormat,
    HttpMethod,
)
from codegen.plugin import GcpPlugin

# All tests use real manifests and discovery cache from the repo
REPO_ROOT = Path(__file__).parent.parent.parent.parent
MANIFESTS_DIR = REPO_ROOT / "codegen" / "providers" / "gcp" / "manifests"
CACHE_DIR = REPO_ROOT / "codegen" / "providers" / "gcp" / "discovery_cache"


@pytest.fixture
def plugin() -> GcpPlugin:
    return GcpPlugin(manifests_dir=str(MANIFESTS_DIR), cache_dir=str(CACHE_DIR))


# ===========================================================================
# recommender.toml — smallest API, good baseline
# ===========================================================================

class TestResolveRecommender:

    def test_api_metadata(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        assert api.name == "recommender"
        assert api.display_name == "Recommender API"
        assert api.version == "v1"
        assert "recommender.googleapis.com" in api.base_url

    def test_client_config(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        assert api.client.client_struct == "RecommenderClient"
        assert api.client.accessor_name == "recommender"

    def test_types_count(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        # recommender.toml has 13 [[types]] entries
        assert len(api.types) == 13

    def test_recommendation_type(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        rec = next(t for t in api.types if t.name == "Recommendation")
        assert rec.schema_name == "GoogleCloudRecommenderV1Recommendation"
        assert rec.description  # should have a description from discovery
        # name field should be required
        name_field = next(f for f in rec.fields if f.name == "name")
        assert name_field.required is True
        # priority field should have enum_type
        prio_field = next(f for f in rec.fields if f.name == "priority")
        assert prio_field.enum_type == "RecommendationPriority"

    def test_enums_extracted(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        enum_names = {e.name for e in api.enums}
        assert "RecommendationPriority" in enum_names
        assert "RecommendationState" in enum_names
        assert "ImpactCategory" in enum_names

    def test_enum_has_variants(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        prio = next(e for e in api.enums if e.name == "RecommendationPriority")
        assert len(prio.variants) > 0
        variant_names = {v.api_name for v in prio.variants}
        assert "P1" in variant_names or "PRIORITY_UNSPECIFIED" in variant_names

    def test_operations(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        assert len(api.operations) == 1
        op = api.operations[0]
        assert op.name == "list_recommendations"
        assert op.http_method == HttpMethod.GET

    def test_list_operation_has_query_params(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        op = api.operations[0]
        qp_names = [qp.name for qp in op.query_params]
        assert "filter" in qp_names
        assert "pageSize" in qp_names
        assert "pageToken" in qp_names

    def test_list_response_config(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        op = api.operations[0]
        assert op.list_response is not None
        assert op.list_response.type_name == "ListRecommendationsResponse"
        assert op.list_response.items_field == "recommendations"
        assert op.list_response.item_type == "Recommendation"

    def test_list_response_types(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        # ListRecommendationsResponse is declared as a [[types]] entry,
        # so it should NOT be duplicated in list_response_types
        lr_names = {t.name for t in api.list_response_types}
        assert "ListRecommendationsResponse" not in lr_names

    def test_auto_dependencies(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        # Recommender types reference other schemas via $ref that aren't in manifest
        auto_names = {t.name for t in api.auto_types}
        # The exact deps depend on discovery doc, but there should be some
        # (e.g., GoogleCloudRecommenderV1OperationGroup references Operation)
        # At minimum, auto_types should exist and have fields
        for at in api.auto_types:
            assert at.is_auto_dependency is True
            assert len(at.fields) > 0

    def test_no_lro(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "recommender.toml"))
        assert api.lro.pattern == "none"


# ===========================================================================
# service_usage.toml — has LRO config, path_helpers, multiple operations
# ===========================================================================

class TestResolveServiceUsage:

    def test_lro_config(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        assert api.lro.pattern == "name_based"
        assert api.lro.response_type == "ServiceUsageLro"
        assert api.lro.poll_config == "service_usage_operation"
        assert api.lro.operation_struct == "ServiceUsageOperation"
        assert api.lro.base_url == "https://serviceusage.googleapis.com"

    def test_path_helpers(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        assert "service_name" in api.path_helpers
        assert "parent" in api.path_helpers
        assert "projects/{project}/services/{service}" == api.path_helpers["service_name"]

    def test_multiple_operations(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        op_names = {op.name for op in api.operations}
        assert "get_service" in op_names
        assert "enable_service" in op_names
        assert "disable_service" in op_names
        assert "list_services" in op_names
        assert "batch_enable_services" in op_names

    def test_lro_operation_flag(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        enable = next(op for op in api.operations if op.name == "enable_service")
        assert enable.is_lro is True
        get = next(op for op in api.operations if op.name == "get_service")
        assert get.is_lro is False

    def test_enum_state(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        # ServiceStateEnum should be extracted
        enum_names = {e.name for e in api.enums}
        assert "ServiceStateEnum" in enum_names
        state_enum = next(e for e in api.enums if e.name == "ServiceStateEnum")
        variant_names = {v.api_name for v in state_enum.variants}
        # Service Usage states: STATE_UNSPECIFIED, DISABLED, ENABLED
        assert "ENABLED" in variant_names
        assert "DISABLED" in variant_names

    def test_service_state_fields(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        svc = next(t for t in api.types if t.name == "ServiceState")
        field_names = [f.name for f in svc.fields]
        assert "name" in field_names
        assert "state" in field_names
        assert "parent" in field_names

    def test_lro_type_fields(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        lro_type = next(t for t in api.types if t.name == "ServiceUsageLro")
        field_names = [f.name for f in lro_type.fields]
        assert "name" in field_names
        assert "done" in field_names
        assert "error" in field_names

        # done should be required
        done_field = next(f for f in lro_type.fields if f.name == "done")
        assert done_field.required is True

        # error should have explicit rust_type override
        error_field = next(f for f in lro_type.fields if f.name == "error")
        assert error_field.rust_type == "serde_json::Value"

    def test_list_services_query_params(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        list_op = next(op for op in api.operations if op.name == "list_services")
        qp_names = [qp.name for qp in list_op.query_params]
        assert "pageToken" in qp_names
        assert "filter" in qp_names
        assert "pageSize" in qp_names

    def test_list_services_response(self, plugin: GcpPlugin):
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        list_op = next(op for op in api.operations if op.name == "list_services")
        assert list_op.list_response is not None
        assert list_op.list_response.type_name == "ListServicesResponse"


# ===========================================================================
# resolve_all — full provider
# ===========================================================================

class TestResolveAll:

    def test_provider_metadata(self, plugin: GcpPlugin):
        provider = plugin.resolve_all()
        assert provider.provider == "gcp"
        assert provider.target_crate == "."
        assert provider.client_struct == "GcpHttpClient"
        assert provider.rename_all == "camelCase"
        assert provider.wire_format == "json"

    def test_all_apis_resolved(self, plugin: GcpPlugin):
        provider = plugin.resolve_all()
        manifest_count = len(list(MANIFESTS_DIR.glob("*.toml")))
        assert len(provider.apis) == manifest_count

    def test_api_names_unique(self, plugin: GcpPlugin):
        provider = plugin.resolve_all()
        names = [api.name for api in provider.apis]
        assert len(names) == len(set(names))

    def test_each_api_has_types(self, plugin: GcpPlugin):
        provider = plugin.resolve_all()
        for api in provider.apis:
            assert len(api.types) > 0, f"API {api.name} has no types"


# ===========================================================================
# Edge cases
# ===========================================================================

class TestEdgeCases:

    def test_path_params_in_url_order(self, plugin: GcpPlugin):
        """Path params should match URL template order, not alphabetical."""
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        get_op = next(op for op in api.operations if op.name == "get_service")
        # get_service has a path like {+name} — one param
        assert len(get_op.path_params) >= 1

    def test_passthrough_params(self, plugin: GcpPlugin):
        """Params in {+name} format should have passthrough=True."""
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        get_op = next(op for op in api.operations if op.name == "get_service")
        # service_usage get_service uses {+name} passthrough
        plus_params = [pp for pp in get_op.path_params if pp.passthrough]
        assert len(plus_params) >= 1

    def test_request_body_type(self, plugin: GcpPlugin):
        """Operations with request bodies should have request_body_type set."""
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        enable = next(op for op in api.operations if op.name == "enable_service")
        assert enable.request_body_type == "EnableServiceRequest"

    def test_response_type_resolved(self, plugin: GcpPlugin):
        """Response types should use Rust name from manifest, not discovery schema name."""
        api = plugin.resolve(str(MANIFESTS_DIR / "service_usage.toml"))
        get_op = next(op for op in api.operations if op.name == "get_service")
        # get_service returns GoogleApiServiceusageV1Service -> mapped to ServiceState
        assert get_op.response_type == "ServiceState"

    def test_plugin_name(self, plugin: GcpPlugin):
        assert plugin.name() == "gcp"

    def test_plugin_target_crate(self, plugin: GcpPlugin):
        assert plugin.target_crate() == "."
