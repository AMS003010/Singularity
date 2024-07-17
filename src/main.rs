use std::fs;
use std::io::Error as IOError;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_yaml::Result as SerdeResult;

mod widgets {
    pub mod weather;
}

mod feed {
    pub mod weather_data;
}

mod internals {
    pub mod singularity;
}

use internals::singularity::Config;
use widgets::weather::weather_widget_handler;

async fn landerpage() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn run_actix_server(port: u16) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(landerpage))
    })
    .bind(addr.clone())
    .unwrap()
    .run();

    println!("Singularity🌌 running at http://127.0.0.1:{}\n", port);
    server.await.unwrap();
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(),IOError> {
    weather_widget_handler("Bengaluru".to_string()).await;
    let yaml_data = match fs::read_to_string("singularity.yaml") {
        Ok(data) => data,
        Err(_) => {
            println!("Couldn't find singularity.yaml under '/' ❌");
            println!("Trying under '../app/singularity.yaml' 🕜\n");
            fs::read_to_string("../app/singularity.yaml").expect("Unable to read file")
        }
    };
    let singularity: SerdeResult<Config> = serde_yaml::from_str(&yaml_data);
    let port = 8080;
    match singularity {
        Ok(value) => {
            println!("{:?}", value);
            println!("\nParsed yaml file Successfully!!!");
            if let Err(e) = run_actix_server(port).await {
                eprintln!("Failed to run Actix server: {}", e);
            }
        },
        Err(err) => println!("Error in parsing singularity.yaml {}", err),
    }
    Ok(())
}