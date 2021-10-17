use anyhow::Result;

pub async fn url2md(url: &String) -> Result<String> {
    let dom = fetcher::fetch(url).await?;
    let title = match parser::parse(dom.as_str()) {
        Some(title) => title,
        None => url.to_string(),
    };
    let title = title.trim();
    let md = converter::convert(title, url);
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
    pub fn parse(dom: &str) -> Option<String> {
        let html = Html::parse_document(dom);
        let root_ref = html.root_element();
        let parsed = &Selector::parse("title");
        let selector = match parsed {
            Ok(selector) => selector,
            Err(_err) => {
                // eprintln!("{:?}", err);
                return None;
            }
        };

        let title = match root_ref.select(selector).next() {
            Some(element_ref) => element_ref.inner_html(),
            None => return None,
        };

        Some(title)
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            parse("<!DOCTYLE><head><title>THIS IS TITLE</title></head>").unwrap(),
            "THIS IS TITLE"
        );
    }

    #[test]
    #[should_panic]
    fn parse_cannot_find_title_test() {
        parse("<!DOCTYLE><head><div>THIS IS NO TITLE</div></head>").unwrap();
    }
}

mod converter {
    pub fn convert(title: &str, url: &str) -> String {
        format!("[{}]({})", title, url)
    }

    #[test]
    fn convert_test() {
        assert_eq!(
            convert("Google", "https://google.com"),
            "[Google](https://google.com)"
        );
    }
}
