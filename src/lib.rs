use std::fs::File;
use std::io::Write;

pub fn download_to_csv(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // HTTP 요청을 통해 URL에서 데이터를 가져옵니다.
    let response = reqwest::blocking::get(url)?;

    // 가져온 데이터의 내용을 읽습니다.
    let content = response.text()?;

    // 지정된 경로에 파일을 생성하고 내용을 기록합니다.
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

// 사용 예시:
// download_to_csv("https://example.com/data.csv", "local_data.csv").unwrap();
