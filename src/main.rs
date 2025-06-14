use reqwest::blocking::get;
use select::document::Document;
use select::predicate::Name;
use slug::slugify;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("❗ Usage: url_to_md <url1> [url2] ...");
        std::process::exit(1);
    }

    let output_dir = Path::new("docs/internal");
    fs::create_dir_all(&output_dir).unwrap();

    for url in args {
        match fetch_and_convert(&url, &output_dir) {
            Ok(path) => println!("✅ Saved: {}", path.display()),
            Err(e) => eprintln!("❌ Failed for {}: {}", url, e),
        }
    }
}

fn fetch_and_convert(url: &str, output_dir: &Path) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let resp = get(url)?.text()?;
    let document = Document::from(resp.as_str());

    let title = document
        .find(Name("title"))
        .next()
        .map(|n| n.text())
        .unwrap_or_else(|| "untitled".to_string());

    let slug = slugify(&title);
    let article_html = document
        .find(Name("article"))
        .next()
        .map(|n| n.html())
        .unwrap_or_else(|| resp.clone());

    let markdown = html2md::parse_html(&article_html);

    let file_path = output_dir.join(format!("{}.md", slug));
    let content = format!(
        "---\ntitle: \"{}\"\nsource: \"{}\"\n---\n\n{}",
        title, url, markdown
    );

    fs::write(&file_path, content)?;
    Ok(file_path)
}