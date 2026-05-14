use criterion::{black_box, criterion_group, criterion_main, Criterion};
use omokoda_core::providers::{MockProvider, ProviderRegistry};
use std::time::Duration;

fn provider_routing_benchmark(c: &mut Criterion) {
    let mut registry = ProviderRegistry::new();
    registry.register(Box::new(MockProvider::new("mock-a".to_string())));
    registry.register(Box::new(MockProvider::new("mock-b".to_string())));

    c.bench_function("provider_routing", |b| {
        b.iter(|| {
            let _ = registry.route_think("hello", &[], false);
        })
    });
}

fn wasm_tool_execution_benchmark(c: &mut Criterion) {
    let wasm_path = std::path::Path::new("benches/test_bench.wasm");
    let wasm_bytes = wat::parse_str(r#"(module (func (export \"run\") (nop)))"#).unwrap();
    std::fs::create_dir_all("benches").unwrap();
    std::fs::write(wasm_path, wasm_bytes).unwrap();

    c.bench_function("wasm_tool_execution", |b| {
        b.iter(|| {
            let sandbox = omokoda_core::sandbox::WasmSandbox::new().unwrap();
            let result = sandbox.execute_module(wasm_path, &[], true);
            black_box(result).unwrap();
        })
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(3));
    targets = provider_routing_benchmark, wasm_tool_execution_benchmark
}
criterion_main!(benches);
