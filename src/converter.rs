use scraper::{Html, Selector};

pub async fn url2md(url: &String) -> Result<String, reqwest::Error> {
    // fetch
    let _url = "https://www.rust-lang.org";
    let body = reqwest::get(_url).await?.text().await?;
    println!("title is {:?}", body);

    // parser
    let html = Html::parse_document(body.as_str());
    let root_ref = html.root_element();
    let title = root_ref
        .select(&Selector::parse("title").unwrap())
        .next()
        .unwrap()
        .inner_html();

    // serialize
    println!("in url md: title={:?}", title);
    Ok("[md](google.com)".to_string())
}
