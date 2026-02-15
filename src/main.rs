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
    /// longman, merriam-webster, oed, oxford-learners, urban-dictionary, wiktionary, wordnet, wordnik
    #[arg(short, long)]
    dictionary: Option<String>,
}

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
    let args = Args::parse();

    if let Some(dict) = &args.dictionary {
        isaword::run_cli_single(args.word.as_str(), dict).await
    } else {
        isaword::run_cli(args.word.as_str()).await
    }
}
