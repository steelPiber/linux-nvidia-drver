mod version;
use version::get_nvidia_driver_version;

use reqwest;
use scraper::{Html, Selector};
use serde_json::json;
use std::error::Error;
use std::io::{self, Write};
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 0) 현재 설치된 드라이버 버전 확인
    match get_nvidia_driver_version() {
        Ok(version) => {
            println!("Current driver version: {}", version);
        }
        Err(e) => {
            eprintln!("Failed to get driver version: {}", e);
        }
    }

    // 1) 웹 페이지에서 HTML을 가져옴
    let nvidia_url = "https://www.nvidia.com/ko-kr/drivers/unix/";
    let response = reqwest::get(nvidia_url).await?.text().await?;
    let document = Html::parse_document(&response);

    // 2) "div#rightContent" 안에 있는 모든 <p> 태그 선택
    let content_selector = Selector::parse("div#rightContent p").unwrap();
    let strong_selector = Selector::parse("strong").unwrap();

    // 결과 JSON 구조
    let mut driver_data = json!({
        "Linux x86_64/AMD64/EM64T": []
    });

    // <p>를 전부 순회하면서 "Linux x86_64/AMD64/EM64T" 문자열이 포함된 <strong>이 있는지 확인
    for p_element in document.select(&content_selector) {
        if let Some(strong_el) = p_element.select(&strong_selector).next() {
            let strong_text = strong_el.text().collect::<Vec<_>>().join("");
            if strong_text.contains("Linux x86_64/AMD64/EM64T") {
                let mut current_title = String::new();
                for node in p_element.children() {
                    // 텍스트 노드(#text)
                    if let Some(text_node) = node.value().as_text() {
                        let trimmed = text_node.trim();
                        if !trimmed.is_empty() {
                            current_title = trimmed.replace(":", "").trim().to_string();
                        }
                    }
                    // a 태그
                    else if let Some(el) = node.value().as_element() {
                        if el.name() == "a" {
                            if let Some(href) = el.attr("href") {
                                let link_text = node
                                    .children()
                                    .filter_map(|child| child.value().as_text())
                                    .map(|txt| txt.to_string())
                                    .collect::<Vec<String>>()
                                    .join("")
                                    .trim()
                                    .to_string();

                                driver_data["Linux x86_64/AMD64/EM64T"]
                                    .as_array_mut()
                                    .unwrap()
                                    .push(json!({
                                        "title": current_title,
                                        "version": link_text,
                                        "link": href
                                    }));
                            }
                        }
                    }
                }
                // 해당 <p>만 확인하고 나간다고 가정
                break;
            }
        }
    }
    
    let vec_empty = vec![];
    // 3) 추출된 드라이버 목록 표시
    let drivers = driver_data["Linux x86_64/AMD64/EM64T"]
        .as_array()
        .unwrap_or(&vec_empty);
    if drivers.is_empty() {
        println!("No drivers found!");
        return Ok(());
    }

    println!("Available drivers:");
    for (i, driver) in drivers.iter().enumerate() {
        let title = driver["title"].as_str().unwrap_or("Unknown");
        let version = driver["version"].as_str().unwrap_or("Unknown");
        println!("[{}] {} - {}", i, title, version);
    }

    // 4) 사용자 입력 받기
    println!("Select a driver index to download:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(9999);
    if choice >= drivers.len() {
        eprintln!("Invalid choice!");
        return Ok(());
    }

    let chosen_driver = &drivers[choice];
    let chosen_version = chosen_driver["version"].as_str().unwrap_or("Unknown");

    // 5) 다운로드 URL 구성
    let final_url = format!(
        "https://kr.download.nvidia.com/XFree86/Linux-x86_64/{v}/NVIDIA-Linux-x86_64-{v}.run",
        v = chosen_version
    );
    println!("Downloading from: {}", final_url);

    // 6) HTTP GET으로 파일 다운로드
    let mut resp = reqwest::get(&final_url).await?;
    if !resp.status().is_success() {
        eprintln!("Download request failed with status: {}", resp.status());
        return Ok(());
    }

    // 저장할 파일명 예: NVIDIA-Linux-x86_64-560.35.03.run
    let out_file_name = format!("NVIDIA-Linux-x86_64-{}.run", chosen_version);
    let mut out_file = File::create(&out_file_name)?;

    // (1) 전체 파일 크기 (알 수 없으면 None)
    let total_size = resp.content_length();

    // (2) 지금까지 받은 바이트
    let mut downloaded: u64 = 0;

    // (3) chunk 단위로 받으며 진행 상황 표시
    while let Some(chunk) = resp.chunk().await? {
        // chunk: Bytes (Sized)
        out_file.write_all(&chunk)?;

        downloaded += chunk.len() as u64;
        if let Some(total) = total_size {
            let percent = (downloaded as f64 / total as f64) * 100.0;
            eprint!("\rDownloaded: {:.2}%", percent);
        } else {
            eprint!("\rDownloaded: {} bytes", downloaded);
        }
    }

    eprintln!();
    println!("Saved as '{}'", out_file_name);

    Ok(())
}
