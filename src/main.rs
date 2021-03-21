extern crate crabler;
extern crate futures;
extern crate tokio;
use crabler::*;
use std::path::Path;

use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc;
use std::io::stdin;
use std::thread;
use websocket::result::WebSocketError;
use websocket::{ClientBuilder, OwnedMessage};

#[derive(WebScraper)]
#[on_response(response_handler)]
#[on_html("a[href]", walk_handler)]
struct Scraper {}

impl Scraper {
    async fn response_handler(&self, response: Response) -> Result<()> {
        if response.url.ends_with(".jpg") && response.status == 200 {
            println!("Finished downloading {} -> {}", response.url, response.download_destination.unwrap());
        }
        Ok(())
    }

    async fn walk_handler(&self, mut response: Response, a: Element) -> Result<()> {
        if let Some(href) = a.attr("href") {
            // attempt to download an image
            if href.ends_with(".jpg") {
                let p = Path::new("/tmp").join("image.jpg");
                let destination = p.to_string_lossy().to_string();

                if !p.exists() {
                    println!("Downloading {}", destination);
                    // schedule crawler to download file to some destination
                    // downloading will happen in the background, await here is just to wait for job queue
                    response.download_file(href, destination).await?;
                } else {
                    println!("Skipping exist file {}", destination);
                }
            } else if(href.starts_with("http://data")) {
                println!("href is : {:?}", href.clone());
                let result = fetch_path(href.clone()).await;

                let mut result_str:String = String::new();
                if(result.is_ok()) {
                    result_str = result.unwrap()
                }

                println!("请求输出：{:?}", result_str);
            } else {
                // or schedule crawler to navigate to a given url
                println!("href is : {:?}", href.clone());
                response.navigate(href).await?;

            };
        }

        Ok(())
    }
}

const CONNECTION: &'static str = "http://www.webxml.com.cn/WebServices/ChinaStockWebService.asmx";

// Async websocket chat client
#[tokio::main]
async fn main() {
    let scraper = Scraper {};

    // Run scraper starting from given url and using 20 worker threads
    scraper.run(Opts::new().with_urls(vec!["https://www.eastmoney.com/"]).with_threads(20)).await;

    println!("Connecting to {}", CONNECTION);

    // Construct new Tokio runtime environment


    let (usr_msg, stdin_ch) = mpsc::channel(0);

    // Spawn new thread to read user input
    // stdin isn't supported in mio yet, so we use a thread
    // see https://github.com/carllerche/mio/issues/321
    thread::spawn(|| {
        let mut input = String::new();
        let mut stdin_sink = usr_msg.wait();
        loop {
            // Read user input from stdin
            input.clear();
            stdin().read_line(&mut input).unwrap();

            // Trim whitespace and match input to known chat commands
            // If input is unknown, send trimmed input as a chat message
            let trimmed = input.trim();
            let (close, msg) = match trimmed {
                "/close" => (true, OwnedMessage::Close(None)),
                "/ping" => (false, OwnedMessage::Ping(b"PING".to_vec())),
                _ => (false, OwnedMessage::Text(trimmed.to_string())),
            };
            // Send message to websocket server
            stdin_sink
                .send(msg)
                .expect("Sending message across stdin channel.");
            // If user entered the "/close" command, break the loop
            if close {
                break;
            }
        }
    });

    // Construct a new connection to the websocket server
    let runner = ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket")
        .async_connect_insecure()
        .and_then(|(duplex, _)| {
            let (sink, stream) = duplex.split();
            stream
                // Iterate over message as they arrive in stream
                .filter_map(|message| {
                    println!("Received Message: {:?}", message);
                    // Respond to close or ping commands from the server
                    match message {
                        OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
                        _ => None,
                    }
                })
                // Takes in messages from both sinks
                .select(stdin_ch.map_err(|_| WebSocketError::NoDataAvailable))
                // Return a future that completes once all incoming data from the above streams has been processed into the sink
                .forward(sink)
        });
    // Start our websocket client runner in the Tokio environment
    runner.wait();
}

async fn fetch_path(path:String) -> surf::Result<String>{
    let mut back_string = String::new();
    match reqwest::get(&path).await {
        Ok(response) => {
            match response.text().await{
                Ok(text) =>{
                    println!("Read response text {},{}" ,text.len(),text);
                    back_string = format!("Read response text {} \t {}\t {}",path,text.len(),text)
                }
                Err(_) => {
                    println!("Read response text Error!")
                }
            };
        }
        Err(_) => {
            println!("reqwest get Error!")
        }
    }
    Ok(back_string)
}