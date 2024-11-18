use crate::loader::loader;
use select::document::Document;
use console::style;
use select::predicate::{ Name, Class };
use select::node::Node;
use serde_json::Value;
use serde::Deserialize;
use headless_chrome::Browser;

#[derive(Deserialize, Debug)]
struct PirateBayEntry {
    title: String,
    magnet: String,
    size: String,
    seeders: u32,
    leechers: u32,
    uploaded: String,
    uploader: String,
    category: String,
}

fn find_element_by_class<'a>(node: &'a Node<'a>, class: &'a str) -> String {
    node.find(Class(class))
        .map(|n| n.text())
        .next()
        .unwrap_or_else(|| "".to_string())
}

pub async fn pirate_bay_scrapper(query: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://thepiratebay.org/search.php?q={}&cat=0", query);
    let (tx, spinner_handle) = loader().await;
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    // Navigate to the page and wait for JavaScript to execute
    tab.navigate_to(url.as_str())?.wait_until_navigated()?;
    tab.wait_for_element(".list-entry")?;

    tx.send(true)?;
    spinner_handle.await?;

    let mut count = 0;
    // let body = tab.evaluate("Array.from(document.querySelectorAll('.list-entry')).map(el => el.innerHTML)", true)?.value.unwrap();
    let body = tab.evaluate("document.body.innerHTML", true)?.value.unwrap(); // .unwrap().value.unwrap();
    let body_html_str = match body {
        Value::String(html) => html,
        _ => {
            eprintln!("Failed to retrieve body HTML as string.");
            return Ok(());
        }
    };
    let document = Document::from(body_html_str.as_str());
    let mut entries = vec![];
    for node in document.find(Class("list-entry")) {
        let category = find_element_by_class(&node, "item-category");
        let title = find_element_by_class(&node, "item-title");
        let magnet = node
            .find(Class("item-icons"))
            .next()
            .unwrap()
            .children()
            .find(|node: &select::node::Node| node.name() == Some("a"))
            .unwrap()
            .attr("href")
            .unwrap()
            .to_string();
        let size = find_element_by_class(&node, "item-size");
        let seeders = find_element_by_class(&node, "item-seed")
            .parse::<u32>()
            .unwrap_or_else(|_| 0);
        let leechers = find_element_by_class(&node, "item-leech")
            .parse::<u32>()
            .unwrap_or_else(|_| 0);
        let uploaded = find_element_by_class(&node, "item-uploaded");
        let uploader = find_element_by_class(&node, "item-user");

        let entry = PirateBayEntry {
            title,
            magnet: magnet.clone(),
            size,
            seeders,
            leechers,
            uploaded,
            uploader,
            category,
        };
        if magnet != "" {
            entries.push(entry);
        }
    }

    for entry in entries {
        count += 1;
        if(count > 5) {
            break;
        }
        let hyperlink = format!("\x1b]8;;{0}\x1b\\{1}\x1b]8;;\x1b\\", entry.magnet, entry.title);
        println!("{}", style(hyperlink).cyan().blink());
        println!(
            "Category: {}, Size:{}, Seeders:{}, Leechers:{}",
            entry.category,
            entry.size,
            entry.seeders,
            entry.leechers
        );
        println!("Uploaded: {}, Uploader: {}", entry.uploaded, entry.uploader);
        println!("");
        println!();
    }
    Ok(())
}
