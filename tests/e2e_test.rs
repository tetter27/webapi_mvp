use webapi_mvp;

#[cfg(test)]
#[allow(non_snake_case)]
mod E2Eテスト {

    use super::*;

    #[tokio::test]
    async fn projectsに対してpathを指定するとそのページの情報を取得できる() {

        let url = "http://localhost:8088/technologies/aws";

        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://selenium-chrome:4444", caps).await;

        driver.as_ref().expect("goto error").goto(url).await;

        let html_result = driver.as_ref().expect("source error").page_source().await;
        let html = html_result.unwrap();

        println!("{}", html);

        assert!(html.contains("AWS"));
        assert!(html.contains("ProjectA"));

        driver.expect("quit error").quit().await;
        
    }



}