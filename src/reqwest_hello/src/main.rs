/*
 * Rust自習（HTTPリクエスト reqwest）。
 * CreatedAt: 2019-08-01
 */
use std::collections::HashMap;

fn main() -> Result<(), Box<std::error::Error>> {
    // GET
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
        .json()?;
    println!("{:#?}", resp);
    
    // POST
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post");
    println!("{:#?}", res);
    
    Ok(())
}
