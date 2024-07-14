use std::fs;
use serde_yaml::Result;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sailfish::TemplateOnce;

mod widgets {
    pub mod weather;
}

mod internals {
    pub mod singularity;
}

use internals::singularity::Config;
use widgets::weather::weather_get;

#[derive(TemplateOnce)]
#[template(path = "../src/assets/templates/home.stpl")]
struct Home {}

async fn homepage() -> impl Responder {
    HttpResponse::Ok().body(Home {}.render_once().unwrap())
}

async fn run_actix_server(port: u16) -> std::io::Result<()> {
    let addr = format!("localhost:{}",port);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(homepage))
    })
    .bind(addr.clone())
    .unwrap()
    .run();

    println!("Singularity🌌 running at at http://{}", addr);
    server.await.unwrap();
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<()> {
    weather_get();
    let yaml_data = fs::read_to_string("singularity.yaml").expect("Unable to read file");
    let singularity: Result<Config> = serde_yaml::from_str(&yaml_data);
    let port = 8080;
    match singularity {
        Ok(value) => {
            println!("{:?}",value);
            println!("\nParsed yaml file Successfully!!!");
            if let Err(e) = run_actix_server(port).await{
                eprintln!("Failed to run Actix server: {}", e);
            }
        },
        Err(err) => println!("Error in parsing singularity.yaml {}", err),
    }
    Ok(())
}