use reqwest;
use scraper::{Html, Selector};
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1) 웹 페이지에서 HTML을 가져옴
    let nvidia_url = "https://www.nvidia.com/ko-kr/drivers/unix/";
    let response = reqwest::get(nvidia_url).await?.text().await?;
    let documenta = Html::parse_document(&response);

    // 2) "div#rightContent" 안에 있는 모든 <p> 태그 선택
    let content_selector = Selector::parse("div#rightContent p").unwrap();
    let strong_selector = Selector::parse("strong").unwrap();

    // 결과 JSON 구조
    let mut driver_data = json!({
        "Linux x86_64/AMD64/EM64T": []
    });

    // <p> 들을 전부 순회
    for p_element in document.select(&content_selector) {
        // <p> 안에 strong 태그가 있는지 확인
        //   예: <strong>Linux x86_64/AMD64/EM64T</strong>
        if let Some(strong_el) = p_element.select(&strong_selector).next() {
            let strong_text = strong_el.text().collect::<Vec<_>>().join("");

            // "Linux x86_64/AMD64/EM64T"라는 문구가 있는지 검사
            if strong_text.contains("Linux x86_64/AMD64/EM64T") {
                // ---- 여기서부터 우리가 원하는 <p> 발견! ----
                let mut current_title = String::new();

                // <p> 안의 모든 자식 노드를 순회
                for node in p_element.children() {
                    // 텍스트 노드(#text)인 경우
                    if let Some(text_node) = node.value().as_text() {
                        let trimmed = text_node.trim();
                        if !trimmed.is_empty() {
                            // 예) "최신 프로덕션 브랜치 버전:"
                            current_title = trimmed.replace(":", "").trim().to_string();
                        }
                    }
                    // a 태그인 경우
                    else if let Some(el) = node.value().as_element() {
                        if el.name() == "a" {
                            if let Some(href) = el.attr("href") {
                                // a 태그 내부 텍스트
                                let link_text = node
                                    .children()
                                    .filter_map(|child| child.value().as_text())
                                    .map(|txt_node| txt_node.to_string())  // &Text -> String
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
                // 원하는 <p>는 딱 하나면 되므로, 찾고 나서 break
                break;
            }
        }
    }

    // 결과 출력
    println!("{}", serde_json::to_string_pretty(&driver_data)?);

    Ok(())
}