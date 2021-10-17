use anyhow::Result;

pub async fn url2md(url: &String) -> Result<String> {
    let dom = fetcher::fetch(url).await?;
    let title = parser::parse(dom.as_str());
    let title = title.trim();
    let md = converter::convert(url, title);
    Ok(md)
}

mod fetcher {
    use anyhow::Result;
    pub async fn fetch(url: &str) -> Result<String> {
        Ok(reqwest::get(url).await?.text().await?)
    }
}

mod parser {
    use scraper::{Html, Selector};
    pub fn parse(dom: &str) -> String {
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
}

mod converter {
    pub fn convert(url: &str, title: &str) -> String {
        format!("[{}]({})", url, title)
    }

    #[test]
    fn convert_test() {
        assert_eq!(
            convert("https://google.com", "Google"),
            "[https://google.com](Google)"
        );
    }
}
// "\n        \n            Rust Programming Language\n        \n        "
