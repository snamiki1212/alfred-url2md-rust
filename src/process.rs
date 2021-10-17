use anyhow::Result;
// parser
use scraper::{Html, Selector};

async fn fetch(url: &str) -> Result<String> {
    Ok(reqwest::get(url).await?.text().await?)
}

pub async fn url2md(url: &String) -> Result<String> {
    // fetch
    let body = fetch(url).await?;

    // parser
    let html = Html::parse_document(body.as_str());
    let root_ref = html.root_element();
    let title = root_ref
        .select(&Selector::parse("title").unwrap())
        .next()
        .unwrap()
        .inner_html();

    println!("[parsed]title={:?}", title);
    // serialize
    // let re = Regex::new(r"(?![\s\\n]+$).").unwrap();
    // let title = re.captures(title.as_str()).unwrap();
    let title = title.trim();
    println!("[serialized]title={:?}", title);

    // converter
    let md = convert(url, title);
    println!("[converted]title={:?}", md);

    Ok(title.to_string())
}

fn convert(url: &str, title: &str) -> String {
    format!("[{}]({})", url, title)
}

// "\n        \n            Rust Programming Language\n        \n        "
