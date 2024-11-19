mod chat;
mod loader;
mod search;
mod web_scrapper;
use crate::chat::gemini_chat;
use crate::search::google_search;
use crate::web_scrapper::pirate_bay_scrapper;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    query: String,

    #[arg(short, long, default_value = "google")]
    search: String,
    #[arg(short, long, default_value = "false")]
    chat: bool,
    #[arg(short, long, default_value_t = 5)]
    number_of_results: u8,
    #[arg(short, long, default_value = "")]
    piratebay: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("args: {:?}", args);
    if args.query != "" && args.piratebay != "" {
        println!("Please provide only one of the following options: --query or --piratebay");
    }
    if args.number_of_results == 0 {
        println!("Please provide a number greater than 0 for --number-of-results");
    }
    if args.chat && args.query != "" {
        let query = args.query.clone();

        let (_google_search_result, _gemini_chat_result) = tokio::join!(
            google_search(query.clone(), args.number_of_results),
            gemini_chat(query.clone())
        );
    }
    if args.query != "" {
        println!("Searching for: {}", args.query);
        google_search(args.query, args.number_of_results).await?;
    }
    if args.piratebay != "" {
        if args.chat {
            println!("Chat option is not available with --piratebay option");
        } else {
            pirate_bay_scrapper(args.piratebay, args.number_of_results).await?;
        }
    }
    Ok(())
}
