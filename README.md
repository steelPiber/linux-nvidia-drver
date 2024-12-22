# NVIDIA Linux Driver Downloader

이 프로젝트는 NVIDIA 공식 사이트([https://www.nvidia.com/ko-kr/drivers/unix/](https://www.nvidia.com/ko-kr/drivers/unix/))에서  
Linux x86_64/AMD64/EM64T용 드라이버 버전 목록을 스크레이핑하고,  
원하는 버전을 직접 선택하여 `.run` 파일을 다운로드할 수 있는 **콘솔 프로그램**입니다.

## 기능
- **스크레이핑**: `scraper` 크레이트로 HTML 파싱 후, "Linux x86_64/AMD64/EM64T" 섹션 정보를 추출
- **버전 선택**: 추출된 드라이버 목록 중 원하는 인덱스를 입력받아 선택
- **다운로드 진행 표시**: 파일 다운로드 시 **진행률(%)** 혹은 다운로드한 바이트 수를 콘솔에 표시

## 사용 방법

1. **Rust 환경** 준비  
   - Rust 1.68 이상 + Cargo  
   - (Optional) `rustup`으로 환경을 관리하는 것을 권장

2. **프로젝트 클론 & 빌드**
   ```bash
   git clone https://github.com/<user>/<repo>.git
   cd <repo>
   cargo build --release
