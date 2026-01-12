use reqwest::blocking::Client;
use reqwest::Error;

// #[tokio::main]
fn main() -> Result<(), Error> {
    let client = Client::new();
    let user = "Test user".to_string();
    let passwd: Option<String> = None;

    let response = client
        .get("https://httpbin.org/get")
        .basic_auth(user, passwd)
        .send();

    println!("Reponse is: {:?}", response);

    Ok(())
}
