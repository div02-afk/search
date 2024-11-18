use tokio::task;
use console::{Term};
use std::{thread, time::Duration};
use tokio::sync::watch;


pub async fn loader()->(watch::Sender<bool>,task::JoinHandle<()>){
    let term = Term::stdout();
    let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut spinner_index = 0;
    let (tx, rx) = watch::channel(false);
    let spinner_handle = task::spawn(async move {
        while !*rx.borrow() {
            let spinner = spinner_chars[spinner_index % spinner_chars.len()];
            term.write_str(&format!("\r{} Fetching data...", spinner)).unwrap();
            term.flush().unwrap();
            spinner_index += 1;
            thread::sleep(Duration::from_millis(100));
        }
        term.write_str("\rDone fetching data.          \n").unwrap();
        term.flush().unwrap();
    });
    return (tx,spinner_handle);
}