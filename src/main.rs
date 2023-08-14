use reqwest::blocking::Client;   
use reqwest::Error;

fn main() -> Result<(),Error> {
    let client = Client::new();
    let user = "vishal".to_string();
    let passwd:Option<String> = None;

    let respone = client.get("http://httpbin.org/").basic_auth(user,passwd).send();

    println!("This is the response {:?}",respone);

    Ok(())

}


