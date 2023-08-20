use thirtyfour::{WebDriver, DesiredCapabilities};
use std::panic;

#[cfg(test)]
#[allow(non_snake_case)]
mod E2Eテスト {
    use super::*;

    #[tokio::test]
    async fn projectsに対してpathを指定するとそのページの情報を取得できる() {
        let url = "http://server:8088/technologies/aws";

        let caps = DesiredCapabilities::chrome();
        // Seleniumで使用するWebDriverの生成
        let driver = WebDriver::new("http://selenium-chrome:4444", caps)
                                    .await
                                    .expect("failed to create driver");

        // APIサーバへ接続
        if let Err(e) = driver.goto(url).await {
            driver.quit().await
                            .expect("quit error due to goto failure");
            panic!("goto error: {:?}", e);
        }

        // ソースの取得
        let html = match driver.source().await {
            Ok(content) => content,
            Err(e) => {
                driver.quit().await
                            .expect("quit error due to source failure");
                panic!("source error: {:?}", e);
            }
        };

        assert!(html.contains("AWS"));
        assert!(html.contains("ProjectA"));

        driver.quit().await.expect("quit error");
    }
}