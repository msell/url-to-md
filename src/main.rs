use reqwest;
use select::document::Document;
use select::predicate::Name;
use slug::slugify;
use std::env;
use std::fs;
use std::path::Path;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("❗ Usage: url_to_md <url1> [url2] ...");
        eprintln!("Example: url_to_md https://example.com/blog-post");
        std::process::exit(1);
    }

    let output_dir = Path::new("docs/internal");
    if let Err(e) = fs::create_dir_all(&output_dir) {
        eprintln!("❌ Failed to create output directory: {}", e);
        std::process::exit(1);
    }

    println!("🚀 Processing {} URL(s)...", args.len());

    for url in args {
        match fetch_and_convert(&url, &output_dir).await {
            Ok(path) => println!("✅ Saved: {}", path.display()),
            Err(e) => eprintln!("❌ Failed for {}: {}", url, e),
        }
    }
}

async fn fetch_and_convert(url: &str, output_dir: &Path) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    // Validate URL format
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err("URL must start with http:// or https://".into());
    }

    println!("📡 Fetching: {}", url);

    // Create HTTP client with user agent
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; url-to-md/1.0)")
        .build()?;

    let response = client.get(url).send().await?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let html_content = response.text().await?;
    let document = Document::from(html_content.as_str());

    // Extract title
    let title = document
        .find(Name("title"))
        .next()
        .map(|n| n.text())
        .unwrap_or_else(|| "untitled".to_string());

    // Extract content with fallback strategy
    let content_html = if let Some(article) = document.find(Name("article")).next() {
        article.inner_html()  // ✅ Fixed: use inner_html() instead of html()
    } else if let Some(main) = document.find(Name("main")).next() {
        main.inner_html()
    } else if let Some(body) = document.find(Name("body")).next() {
        body.inner_html()
    } else {
        html_content.clone()
    };

    // Convert HTML to Markdown
    let markdown = html2md::parse_html(&content_html);

    // Create filename from title
    let slug = slugify(&title);
    let file_path = output_dir.join(format!("{}.md", slug));

    // Create frontmatter with metadata
    let current_date = Utc::now().format("%Y-%m-%d").to_string();
    let content = format!(
        r#"---
title: "{}"
source: "{}"
date: "{}"
---

{}"#,
        title.replace('"', r#"\""#), // Escape quotes in title
        url,
        current_date,
        markdown
    );

    fs::write(&file_path, content)?;
    
    println!("📄 Title: {}", title);
    println!("📊 Content length: {} characters", markdown.len());
    
    Ok(file_path)
}