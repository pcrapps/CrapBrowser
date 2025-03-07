mod fetch;
mod parser;
mod render;

use fetch::fetch_html;
use parser::extract_text;
use render::display_content;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://example.com";
    println!("🌐 Fetching: {}", url);

    match fetch_html(url).await {
        Ok(html) => {
            println!("✅ Page fetched successfully!\n");
            let (headlines, paragraphs, links) = extract_text(&html);
            display_content(headlines, paragraphs, links);
        }
        Err(err) => {
            println!("❌ Failed to fetch page: {}", err);
        }
    }

    Ok(())
}
