use webapi_mvp;

#[cfg(test)]
#[allow(non_snake_case)]
mod integration_API_DBテスト {

    use super::*;
    use actix_web::{test, App, http::StatusCode};

    #[actix_web::test]
    async fn get_technologiy_page関数はpathに指定した文字列を元にDBから情報を取得する() {
        let path = "/technologies/aws";
        let tech_name = "AWS";

        // テスト用サーバの立ち上げ
        let service = App::new()
                        .service(webapi_mvp::apis::get_technologiy_page);
        let app = test::init_service(service).await;

        // リクエストを生成してレスポンスを取得
        let req = test::TestRequest::get().uri(path).to_request();
        let resp = test::call_service(&app, req).await;

        // 取得した値をパースして確認
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.contains(tech_name));
    }

    #[actix_web::test]
    async fn 存在しない技術にアクセスした場合は404NotFoundエラーが返却される() {
        let path = "/technologies/hoge";
        let tech_name = "AWS";

        // テスト用サーバの立ち上げ
        let service = App::new()
                        .service(webapi_mvp::apis::get_technologiy_page);
        let app = test::init_service(service).await;

        // リクエストを生成してレスポンスを取得
        let req = test::TestRequest::get().uri(path).to_request();
        let resp = test::call_service(&app, req).await;

        // ステータスコードが404であることを確認
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    // TODO: モックを用意
    // "tech_page.html"の変数を書き換えればテスト可能
    #[actix_web::test]
    async fn レンダリングに失敗した場合は500InternalServerErrorが返却される() {
        let path = "/technologies/aws";
        let tech_name = "AWS";

        // テスト用サーバの立ち上げ
        let service = App::new()
                        .service(webapi_mvp::apis::get_technologiy_page);
        let app = test::init_service(service).await;

        // リクエストを生成してレスポンスを取得
        let req = test::TestRequest::get().uri(path).to_request();
        let resp = test::call_service(&app, req).await;

        // ステータスコードが500であることを確認
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}