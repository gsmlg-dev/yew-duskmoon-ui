use std::cell::RefCell;
use std::rc::Rc;

use actix_web::{get, App as ActixApp, Error, HttpResponse, HttpServer};
use tokio::task::LocalSet;
use tokio::task::{spawn_blocking, spawn_local};

use serde::{Deserialize, Serialize};

use yew_app_sample::app::App;

#[get("/")]
async fn render() -> Result<HttpResponse, Error> {
    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            let renderer = yew::ServerRenderer::<App>::new();

            renderer.render().await
        })
    })
    .await
    .expect("the thread has failed.");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            r#"<!DOCTYPE HTML>
                <html>
                    <head>
                        <title>yew-ssr with actix-web example</title>
                    </head>
                    <body>
                        <h1>yew-ssr with actix-web example</h1>
                        {}
                    </body>
                </html>
            "#,
            content
        )))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| ActixApp::new().service(render));
    println!("You can view the website at: http://localhost:18080/");
    server.bind(("0.0.0.0", 18080))?.run().await
}
