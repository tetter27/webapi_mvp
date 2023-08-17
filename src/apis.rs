use actix_web::{get, web, HttpResponse, Responder, HttpServer, App};
use crate::db_connector;
use tera::{Tera, Context};
use crate::models::{Technology, Project};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/technologies/{tech_name}")]
pub async fn get_technologiy_page(
        tech_name: web::Path<String>) -> impl Responder {
    
    // DBへアクセスして技術ページに表示する情報を取得する
    let conn = db_connector::create_connection();
    let path = tech_name.to_string();
    let (tech, projs) = 
    match db_connector::get_technology_page_by_url_name(&conn, &path) {
        Ok(result) => result,
        Err(_) => return HttpResponse::NotFound().finish()
    };

    // HTML形式のレスポンスボディを生成する
    match render_technology_page(tech, projs) {
        Ok(contents) => HttpResponse::Ok().content_type("text/html").body(contents),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub fn render_technology_page(
        tech: Technology, 
        projs: Vec<Project>) -> Result<String, tera::Error> {
        
    let tmpl = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.insert("tech", &tech);
    ctx.insert("projs", &projs);

    tmpl.render("tech_page.html", &ctx) 
}

pub async fn create_app(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_technologiy_page)
    })
    .bind((addr, port))?
    .run()
    .await
}

#[cfg(test)]
#[allow(non_snake_case)]
mod unit_APIテスト {

    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn GET_rootでAPIサーバに接続できる() {

        let path = "/";

        let service = App::new().service(hello);
        let app = test::init_service(service).await;
        let req = test::TestRequest::get().uri(path).to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod unit_contentsテスト {

    use super::*;

    #[test]
    fn render_technology_page関数はtech_pageのコンテンツを生成する() {

        let tech_name = "AWS";
        let proj1_name = "ProjectA";
        let proj2_name = "ProjectB";
        let tech = Technology{
            id: 1,
            name: String::from(tech_name),
            url_name: String::from("aws"),
            image_url: String::from("http://example.com"),
        };

        let projs = vec![
            Project{
                id: 1,
                name: String::from(proj1_name),
                url_name: String::from("project-a"),
            },
            Project{
                id: 2,
                name: String::from(proj2_name),
                url_name: String::from("project-b"),
            },
        ];

        let contents = render_technology_page(tech, projs).unwrap();
        assert!(contents.contains(tech_name));
        assert!(contents.contains(proj1_name));
        assert!(contents.contains(proj2_name));
    }
}
