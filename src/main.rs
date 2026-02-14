use std::env;

/// Entry point for the isaword CLI tool.
/// 
/// Uses #[tokio::main] to set up the tokio async runtime.
/// 
/// Why tokio and not smol?
/// We investigated using smol (lighter weight, single-threaded) but found that:
/// - reqwest depends on hyper, which requires tokio for its reactor
/// - hickory-dns (used for DNS resolution) also depends on tokio
/// - Even with default-features=false + rustls-tls, tokio is unavoidable via transitive deps
/// 
/// For a CLI tool, tokio's overhead is negligible (~100ms startup). The ecosystem
/// is mature, well-tested, and widely used. Alternative HTTP clients (surf, isahc)
/// have better smol integration but would require a complete API rewrite.
/// 
/// See CONVERSION_PLAN.md and issue isaword-8uv for full investigation.
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <word>", args[0]);
        std::process::exit(1);
    }

    let word = &args[1];
    isaword::run_cli(word).await
}
