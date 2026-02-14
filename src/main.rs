use std::env;

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
