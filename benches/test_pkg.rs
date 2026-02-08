#![allow(clippy::unwrap_used)]
use std::path::PathBuf;

use criterion::{Criterion, criterion_group, criterion_main};
use snakedown::config::ConfigBuilder;
use snakedown::render_docs;

fn criterion_benchmark(c: &mut Criterion) {
    let bench_config_path = PathBuf::from("benchmark-config.toml");
    let config_builder = ConfigBuilder::from_path(&bench_config_path).unwrap();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();

    // render_docs(config).await?;
    c.bench_function("isolated", |b| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(&runtime)
            .iter(|| render_docs(config_builder.clone()));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
