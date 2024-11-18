mod chat;
mod loader;
mod search;
use crate::chat::gemini_chat;
use crate::search::google_search;
use clap::Parser;
use tokio::join;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    query: String,

    #[arg(short, long, default_value = "google")]
    search: String,
    #[arg(short, long, default_value = "false")]
    chat: bool,
    #[arg(short, long, default_value_t = 5)]
    number_of_results: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // println!("args: {:?}", args);

    if args.chat {
        let query = args.query.clone();

        let (google_search_result, gemini_chat_result) = tokio::join!(
            google_search(query.clone(), args.number_of_results),
            gemini_chat(query.clone())
        );

        // Check the results
    }
    Ok(())
}
