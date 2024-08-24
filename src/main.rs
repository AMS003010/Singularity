use std::fs;
use std::io::Error as IOError;
use actix_files as fs_actix;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_yaml::Result as SerdeResult;
use internals::singularity::Config;
use internals::port::find_available_port;
// use widgets::weather::weather_widget_handler;
use widgets::clock::clock_widget_handler;

//TODO: Adding a System config Page 

//TODO: Adding a cache approach

//TODO: Remove the warnings

//TODO: ---FUTURE--- Implementing PGO & LTO

//TODO: ---FUTURE--- Replacing Actix-web and hyper with tokio

mod widgets {
    pub mod weather;
    pub mod clock;
}

mod feed {
    pub mod weather_data;
    pub mod clock_data;
}

mod internals {
    pub mod singularity;
    pub mod render;
    pub mod port;
}

async fn landerpage(_config: web::Data<Config>) -> impl Responder {
    // let final_html: String = String::new();
    // final_yaml_to_html_render(&config);
    // clock_widget_handler().await;
    // println!("Beginning of render -> {:?}", config);
    // match weather_widget_handler("Bengaluru".to_string()).await {
    //     Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
    //     Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    // }
    match clock_widget_handler().await {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn run_actix_server(port: u16, config: Config) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/", web::get().to(landerpage))
            .service(fs_actix::Files::new("/static", "src/assets/static").show_files_listing())
    })
    .bind(addr.clone())
    .unwrap()
    .run();

    println!("Singularity🌌 running at http://127.0.0.1:{}\n", port);
    server.await.unwrap();
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), IOError> {

    let yaml_data = fs::read_to_string("singularity.yaml").expect("Couldn't find singularity.yaml under '/' ❌");
    let singularity: SerdeResult<Config> = serde_yaml::from_str(&yaml_data);
    let port = find_available_port(8080);
    match singularity {
        Ok(config) => {
            // println!("After parsing -> {:?}", config);
            
            println!("\nParsed yaml file Successfully!!!");
            if let Err(e) = run_actix_server(port, config).await {
                eprintln!("Failed to run Actix server: {}", e);
            }
        },
        Err(err) => println!("Error in parsing singularity.yaml {}", err),
    }
    Ok(())
}
