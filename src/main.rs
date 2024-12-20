use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://www.nvidia.com/ko-kr/drivers/unix/";
    println!("🔍 Scraping NVIDIA driver versions from: {}", url);

    let res = client.get(url).send()?.text()?;
    let document = Html::parse_document(&res);
    let selector = Selector::parse("a").unwrap();

    let mut driver_links = Vec::new();
    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if href.contains("Download/driverResults.aspx") {
                let full_url = if href.starts_with("https://") {
                    href.to_string()
                } else {
                    format!("https://www.nvidia.com{}", href)
                };
                driver_links.push(full_url);
            }
        }
    }

    if driver_links.is_empty() {
        println!("❌ No driver links found!");
        return Ok(());
    }

    println!("\n🎉 Found {} driver versions:", driver_links.len());
    for (index, link) in driver_links.iter().enumerate() {
        println!("[{}]. {}", index + 1, link);
    }

    println!("📥 Downloading the latest driver version...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?; //사용자로부터 입력을 받음
    let selected_index: usize = input.trim().parse()?; //사용자가 선택한 버전의 인덱스

    if selected_index < 1 || selected_index > driver_links.len() {
        println!("❌ Invalid selection!");
        return Ok(());
    }

    let down_url: &String = &driver_links[selected_index - 1];
    println!("\n🚀 Downloading: {}", down_url);

    let file_name: &str = down_url.split('/').last().unwrap();
    println!("💾 Saving to: {}", file_name);

    let driver_response = client.get(down_url).send()?;
    let total_size: u64 = driver_response.content_length().unwrap_or(0);

    let file_path: &Path = Path::new(file_name);
    let mut file: File = File::create(file_path)?;

    let mut down: u64 = 0;
    let bytes = driver_response.bytes()?;

    for chunk in bytes.chunks(4096) {
        file.write_all(chunk)?;
        down += chunk.len() as u64;
        let percent = (down as f64 / total_size as f64) * 100.0;
        print!("\r⬇️ Downloading... {:.2}%", percent);
        io::stdout().flush()?;
    }

    println!("\n✅ Download complete!");
    Ok(())
}