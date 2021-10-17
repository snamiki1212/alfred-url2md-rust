// parser
use scraper::{Html, Selector};

pub async fn url2md(url: &String) -> Result<String, reqwest::Error> {
    // fetch
    let _url = "https://www.rust-lang.org";
    let body = reqwest::get(_url).await?.text().await?;

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
    Ok(title.to_string())
}

// "\n        \n            Rust Programming Language\n        \n        "
