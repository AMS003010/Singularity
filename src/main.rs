use std::fs;
use serde_yaml::Result;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::fmt::Write;

mod widgets {
    pub mod weather;
}

mod internals {
    pub mod singularity;
}

use internals::singularity::Config;
use widgets::weather::weather_get;

struct Home {
    template_content: String,
}

impl Home {
    fn render(&self) -> String {
        let mut buf = String::new();
        write!(buf, "{}", self.template_content).unwrap();
        buf
    }
}

async fn homepage() -> impl Responder {
    let template_content = match fs::read_to_string("./src/assets/templates/home.stpl") {
        Ok(content) => content,
        Err(_) => {
            println!("Couldn't find Home template at './src/assets/templates/home.stpl' ❌");
            println!("Trying '../app/src/assets/templates/home.stpl' 🕜\n");
            fs::read_to_string("../app/src/assets/templates/home.stpl").expect("Unable to read Home template file")
        }
    };

    let home = Home { template_content };
    HttpResponse::Ok().body(home.render())
}

async fn run_actix_server(port: u16) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(homepage))
    })
    .bind(addr.clone())
    .unwrap()
    .run();

    println!("Singularity🌌 running at http://127.0.0.1:{}\n", port);
    server.await.unwrap();
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<()> {
    weather_get();
    let yaml_data = match fs::read_to_string("singularity.yaml") {
        Ok(data) => data,
        Err(_) => {
            println!("Couldn't find singularity.yaml under '/' ❌");
            println!("Trying under '../app/singularity.yaml' 🕜\n");
            fs::read_to_string("../app/singularity.yaml").expect("Unable to read file")
        }
    };
    let singularity: Result<Config> = serde_yaml::from_str(&yaml_data);
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