use webapi_mvp::db_connector;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 'n', long = "project-name", help = "プロジェクト名")]
    proj_name: String,

    #[arg(short = 'u', long = "project-url", help = "プロジェクトのURL文字列")]
    proj_url_name: String,

    #[arg(short = 'N', long = "technology-name", help = "技術名")]
    tech_name: String,

    #[arg(short = 'U', long = "technology-url", help = "技術のURL文字列")]
    tech_url_name: String,

    #[arg(short = 'I', long = "technology-image", help = "技術ロゴのURL")]
    tech_image_url: String
}

fn main() {
    let args = Args::parse();

    let conn = db_connector::create_connection();
    let _ = db_connector::input_command_data(&conn,
                &args.proj_name, 
                &args.proj_url_name, 
                &args.tech_name, 
                &args.tech_url_name, 
                &args.tech_image_url);
}


#[cfg(test)]
#[allow(non_snake_case)]
mod entriesテスト {

    use super::*;

    const PROJ_NAME: &str = "CLIコマンド";
    const PROJ_URL_NAME: &str = "cli_command";
    const TECH_NAME: &str = "Clap";
    const TECH_URL_NAME: &str = "clap";
    const TECH_IMAGE_URL: &str = "http://example.com";

    #[test]
    fn コマンド自体が動作する() {

        let args: Args = Args::try_parse_from([
            "",  // コマンド自体を指しているため空文字で良い
            "-n", PROJ_NAME, 
            "-u", PROJ_URL_NAME, 
            "-N", TECH_NAME, 
            "-U", TECH_URL_NAME, 
            "-I", TECH_IMAGE_URL
        ]).unwrap();
        
        assert_eq!(args.proj_name, PROJ_NAME);
    }

    #[test]
    fn コマンドで指定したデータをDBへ入力できる() {

        let args: Args = Args::try_parse_from([
            "",  // コマンド自体を指しているため空文字で良い
            "--project-name", PROJ_NAME, 
            "--project-url", PROJ_URL_NAME, 
            "--technology-name", TECH_NAME, 
            "--technology-url", TECH_URL_NAME, 
            "--technology-image", TECH_IMAGE_URL
        ]).unwrap();

        db_connector::test_transaction(|conn| {

            let _ = db_connector::input_command_data(&conn,
                                                    &args.proj_name, 
                                                    &args.proj_url_name, 
                                                    &args.tech_name, 
                                                    &args.tech_url_name, 
                                                    &args.tech_image_url);
            
            // 関数のテスト実行
            let (tech, projs) = db_connector::get_technology_page_by_url_name(&conn, TECH_URL_NAME)?;

            let proj = &projs[0];
            assert_eq!(tech.name, TECH_NAME);
            assert_eq!(tech.image_url, TECH_IMAGE_URL);
            assert_eq!(proj.name, PROJ_NAME);
            assert_eq!(proj.url_name, PROJ_URL_NAME);

            Ok(())
        }).unwrap();
    }
}