use std::fs;
use std::io::Error as IOError;
use actix_files as fs_actix;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_yaml::Result as SerdeResult;
use internals::singularity::Config;
use widgets::weather::weather_widget_handler;

//TODO: Adding a System config Page

//TODO: Adding a cache approach

//TODO: Remove the warnings

mod widgets {
    pub mod weather;
}

mod feed {
    pub mod weather_data;
}

mod internals {
    pub mod singularity;
    pub mod render;
}

async fn landerpage(config: web::Data<Config>) -> impl Responder {
    println!("Beginning of render -> {:?}", config);
    match weather_widget_handler("Bengaluru".to_string()).await {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

async fn run_actix_server(port: u16, config: Config) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone())) // Clone and share the Config object
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

    //TODO: Remove alternate path parse for singularity.yaml

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
        Ok(config) => {
            println!("After parsing -> {:?}", config);
            println!("\nParsed yaml file Successfully!!!");
            if let Err(e) = run_actix_server(port, config).await {
                eprintln!("Failed to run Actix server: {}", e);
            }
        },
        Err(err) => println!("Error in parsing singularity.yaml {}", err),
    }
    Ok(())
}
