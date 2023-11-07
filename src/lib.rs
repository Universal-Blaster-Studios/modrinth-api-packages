use reqwest::blocking::{Client,ClientBuilder};

fn main() {
    let http_client = Client::new();
    let res = http_client.get("https://google.com").send();
    
    if res.is_ok() {
        print!("{:#?}", res.ok().unwrap());
    }
}