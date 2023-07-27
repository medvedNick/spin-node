pub fn install_tracing() {
    use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};

    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        "warn,spin_runtime,spin_primitives=debug,erc20,example_token=trace".to_owned()
    });
    println!("RUST_LOG={}", filter);

    let main_layer = fmt::layer()
        .event_format(fmt::format().with_ansi(true))
        .with_filter(EnvFilter::from(filter));

    let registry = registry().with(main_layer);

    registry.init();
}
