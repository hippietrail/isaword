use clap::Parser;

/// isaword - Check if a word exists in multiple dictionaries
#[derive(Parser, Debug)]
#[command(name = "isaword")]
#[command(about = "Check if a word exists in 13 different dictionaries", long_about = None)]
struct Args {
    /// The word to check
    word: String,

    /// Only check a specific dictionary (e.g., 'chambers', 'oxford', 'cambridge')
    /// Available: american-heritage, cambridge, chambers, dictionary-com, etymonline,
    /// longman, merriam-webster, oed, oxford-learners, urban-dictionary, wiktionary, wordnik
    #[arg(short, long)]
    dictionary: Option<String>,
}

/// Entry point for the isaword CLI tool.
/// 
/// Uses #[tokio::main] to set up the tokio async runtime.
/// 
/// Why tokio and not smol?
/// We investigated smol (lighter-weight, single-threaded async) extensively.
/// Initial findings showed multiple dead-ends:
/// 
/// 1. TLS is the blocker, not the runtime
///    - reqwest + any TLS (rustls-tls, native-certs) → tokio-rustls → tokio
///    - smol-hyper (executor bridge) doesn't solve TLS dependency
///    - futures-rustls is runtime-agnostic, but requires ditching reqwest entirely
/// 
/// 2. Custom HTTP client alternative would cost ~200+ LOC per checker
///    - surf/isahc have smol support but are less mature than reqwest
///    - Would introduce scraping reliability risks for minor CLI startup savings
/// 
/// 3. For a CLI tool, tokio overhead is negligible (~100ms startup)
///    - Not running a server with 1000s of concurrent connections
///    - smol refactor is a sunk-cost trap with diminishing returns
/// 
/// Decision: Accept tokio dependency. Focus on fixing broken scrapers instead.
/// Full investigation details in CONVERSION_PLAN.md and issue isaword-8uv.
#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Some(dict) = &args.dictionary {
        isaword_lib::run_cli_single(args.word.as_str(), dict).await
    } else {
        isaword_lib::run_cli(args.word.as_str()).await
    }
}
