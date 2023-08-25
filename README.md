# webapi_mvp

## 解説
https://zenn.dev/tetter/books/webapi-mvp-book

## 使用方法
1. Docker for Desktop (Windows/Mac) もしくは Docker Engine (Linux) をインストールする
2. `.env` および `docker-compose.yml` の DB 環境を任意の値に設定する
3. DB へ値を挿入する
    - 挿入方法
        - CLI コマンドを使用する
            (使用方法は [こちら](https://zenn.dev/tetter/books/webapi-mvp-book/viewer/11_cli-command) の一番下を確認)
        - adminer を使用する
    - 挿入する値
        - technologies テーブル
            - `id = 1`
                - `name = AWS`
                - `url_name = aws`
                - `image_url_name = [AWSのロゴ画像のURL]`
        - projects テーブル
            - `id = 1`
                - `name = ProjectA` 
                - `url_name = project-a`
            - `id = 2`
                - `name = ProjectB` 
                - `url_name = project-b`
        - adopts テーブル
            - `id = 1`
                - `technologies_id = 1`
                - `projects_id = 1`
                - `created_at = 1970-01-01 00:00:01.000000` 
            - `id = 2`
                - `technologies_id = 1`
                - `projects_id = 2`
                - `created_at = 1970-01-01 00:00:01.000000` 
4. `docker compose up` で起動する
5. ブラウザから `localhost:8080/technoligies/aws` へアクセスする