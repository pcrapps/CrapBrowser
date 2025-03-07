use scraper::{Html, Selector};

pub fn extract_text(html: &str) -> (Vec<String>, Vec<String>, Vec<(String, String)>) {
    let document = Html::parse_document(html);

    let mut headlines = Vec::new();
    let mut paragraphs = Vec::new();
    let mut links = Vec::new();

    // Extract <h1> tags
    let h1_selector = Selector::parse("h1").unwrap();
    for element in document.select(&h1_selector) {
        headlines.push(element.text().collect::<Vec<_>>().join(" "));
    }

    // Extract <p> tags
    let p_selector = Selector::parse("p").unwrap();
    for element in document.select(&p_selector) {
        paragraphs.push(element.text().collect::<Vec<_>>().join(" "));
    }

    // Extract <a> tags with href
    let a_selector = Selector::parse("a").unwrap();
    for element in document.select(&a_selector) {
        if let Some(link) = element.value().attr("href") {
            links.push((element.text().collect::<Vec<_>>().join(" "), link.to_string()));
        }
    }

    (headlines, paragraphs, links)
}