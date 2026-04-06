use criterion::{Criterion, black_box, criterion_group, criterion_main};
use gcp_lite::{
    GcpHttpClient, MockClient,
    token::{StaticTokenProvider, TokenProvider},
};
use serde_json::json;

fn benchmark_operation_polling(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("operation_polling_immediate", |b| {
        b.to_async(&rt).iter(|| async {
            let mut mock = MockClient::new();

            mock.expect_delete("/compute/v1/projects/p/zones/z/disks/d")
                .returning_json(json!({
                    "name": "op-123",
                    "selfLink": "https://compute.googleapis.com/compute/v1/projects/p/zones/z/operations/op-123",
                    "status": "PENDING"
                }))
                .times(1);

            mock.expect_get("/compute/v1/projects/p/zones/z/operations/op-123")
                .returning_json_sequence(vec![json!({"name": "op-123", "status": "DONE"})])
                .times(1);

            let client = GcpHttpClient::from_mock(mock);
            let compute = client.compute();

            let _ = compute.delete_disk("p", "z", "d").await;
        });
    });
}

fn benchmark_retry_logic(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("retry_success_immediate", |b| {
        b.to_async(&rt).iter(|| async {
            let mut mock = MockClient::new();

            mock.expect_get("/test/endpoint")
                .returning_json(json!({"status": "ok"}))
                .times(1);

            let client = GcpHttpClient::from_mock(mock);
            let _ = client
                .get("https://compute.googleapis.com/test/endpoint")
                .await;
        });
    });
}

fn benchmark_token_provider(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("static_token_provider", |b| {
        b.to_async(&rt).iter(|| async {
            let provider = StaticTokenProvider::new("test-token");
            black_box(provider.get_token(&["scope"]).await.unwrap());
        });
    });
}

fn benchmark_service_usage(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("service_usage");

    group.bench_function("single_check", |b| {
        b.to_async(&rt).iter(|| async {
            let mut mock = MockClient::new();
            mock.expect_get("/v1/projects/bench/services/compute.googleapis.com")
                .returning_json(json!({"name": "projects/bench/services/compute.googleapis.com", "state": "ENABLED"}))
                .times(1);

            let client = GcpHttpClient::from_mock(mock);
            let _ = client
                .service_usage()
                .is_service_enabled("bench", "compute.googleapis.com")
                .await;
        });
    });

    group.finish();
}

fn benchmark_instance_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("instance_stop_immediate", |b| {
        b.to_async(&rt).iter(|| async {
            let mut mock = MockClient::new();
            mock.expect_post("/compute/v1/projects/bench/zones/us-central1-a/instances/test/stop")
                .returning_json(json!({
                    "name": "op",
                    "status": "DONE",
                    "selfLink": "https://compute.googleapis.com/compute/v1/projects/bench/zones/us-central1-a/operations/op"
                }))
                .times(1);
            mock.expect_get("/compute/v1/projects/bench/zones/us-central1-a/operations/op")
                .returning_json(json!({"name": "op", "status": "DONE"}))
                .times(1);

            let client = GcpHttpClient::from_mock(mock);
            let _ = client
                .compute()
                .stop_instance("bench", "us-central1-a", "test")
                .await;
        });
    });
}

criterion_group!(
    benches,
    benchmark_operation_polling,
    benchmark_retry_logic,
    benchmark_token_provider,
    benchmark_service_usage,
    benchmark_instance_operations
);
criterion_main!(benches);
