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
    #[arg(short, long,default_value="")]
    query: String,

    #[arg(short, long, default_value = "google")]
    search: String,
    #[arg(short, long, default_value = "false")]
    chat: bool,
    #[arg(short, long, default_value_t = 5)]
    number_of_results: u8,
    #[arg(short, long, default_value = "")]
    pirate: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // println!("args: {:?}", args);

    if args.chat && args.query != "" {
        let query = args.query.clone();

        let (_google_search_result, _gemini_chat_result) = tokio::join!(
            google_search(query.clone(), args.number_of_results),
            gemini_chat(query.clone())
        );
    }
    if args.pirate != "" {
        pirate_bay_scrapper(args.pirate).await?;
    }
    Ok(())
}
