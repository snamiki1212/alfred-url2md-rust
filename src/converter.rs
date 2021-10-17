pub async fn url2md(url: &String) -> Result<String, reqwest::Error> {
    let _url = "https://www.rust-lang.org";
    let title = reqwest::get(_url).await?.text().await?;
    println!("title is {}", title);
    Ok("[md](google.com)".to_string())
}
