use anyhow::Result;
// parser
use scraper::{Html, Selector};

async fn fetch(url: &str) -> Result<String> {
    Ok(reqwest::get(url).await?.text().await?)
}

fn parse(dom: &str) -> String {
    let html = Html::parse_document(dom);
    let root_ref = html.root_element();
    let title = match root_ref.select(&Selector::parse("title").unwrap()).next() {
        Some(element_ref) => element_ref.inner_html(),
        None => "".to_string(),
    };

    title
}

#[test]
fn parse_test() {
    assert_eq!(
        parse("<!DOCTYLE><head><title>THIS IS TITLE</title></head>"),
        "THIS IS TITLE"
    );
    assert_eq!(
        parse("<!DOCTYLE><head><div>THIS IS NO TITLE</div></head>"),
        ""
    );
}

pub async fn url2md(url: &String) -> Result<String> {
    let dom = fetch(url).await?;
    let title = parse(dom.as_str());
    let title = title.trim();
    let md = convert(url, title);
    Ok(md)
}

fn convert(url: &str, title: &str) -> String {
    format!("[{}]({})", url, title)
}

// "\n        \n            Rust Programming Language\n        \n        "
