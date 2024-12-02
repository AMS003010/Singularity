use std::fs;
use std::io::Error as IOError;
use actix_files as fs_actix;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_yaml::Result as SerdeResult;
use internals::singularity::Config;
use internals::port::find_available_port;
use internals::render::final_yaml_to_html_render;

// use widgets::weather::weather_widget_handler;
// use widgets::clock::clock_widget_handler;
// use widgets::calendar::calendar_widget_handler;

//TODO: Adding a System config Page

//TODO: Remove the warnings

//TODO: Implementing COW (Clone On Write) wherever possible

//TODO: Implementing a initial Static HTML render like React / Nextjs and then rehydrate it

//TODO: Add a path for .executable approach like 'singularity -p /test/singularity.yaml'
//      and path of '/singularity.yaml' for container approach with volumes enabled

//TODO: ---FUTURE--- Implementing PGO & LTO

//TODO: ---FUTURE--- Replacing Actix-web and hyper with tokio

//TODO: Maybe add a position system to make the widget injection faster

mod widgets {
    pub mod weather;
    pub mod clock;
    pub mod calendar;
}

mod feed {
    pub mod weather_data;
    pub mod clock_data;
    pub mod calendar_data;
}

mod internals {
    pub mod singularity;
    pub mod render;
    pub mod port;
}

async fn landerpage(_config: web::Data<Config>) -> impl Responder {
    let final_html: String = String::new();
    let rendered_html = final_yaml_to_html_render(&_config, final_html).await;
    HttpResponse::Ok().content_type("text/html").body(rendered_html)
}

async fn run_actix_server(port: u16, config: Config) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .route("/", web::get().to(landerpage))
            .route("/home", web::get().to(landerpage))
            .service(fs_actix::Files::new("/static", "src/assets/static").show_files_listing())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .workers(4)
    .shutdown_timeout(60)
    .run();

    println!("🌀 Singularity running at http://127.0.0.1:{}\n", port);
    server.await.unwrap();
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<(), IOError> {
    let yaml_data = fs::read_to_string("singularity.yaml")
        .expect("Couldn't find singularity.yaml ⚠️");
    
    let singularity: SerdeResult<Config> = serde_yaml::from_str(&yaml_data);

    println!("

  ███████╗██╗███╗   ██╗ ██████╗ ██╗   ██╗██╗      █████╗ ██████╗ ██╗████████╗██╗   ██╗
  ██╔════╝██║████╗  ██║██╔════╝ ██║   ██║██║     ██╔══██╗██╔══██╗██║╚══██╔══╝╚██╗ ██╔╝
  ███████╗██║██╔██╗ ██║██║  ███╗██║   ██║██║     ███████║██████╔╝██║   ██║    ╚████╔╝ 
  ╚════██║██║██║╚██╗██║██║   ██║██║   ██║██║     ██╔══██║██╔══██╗██║   ██║     ╚██╔╝  
  ███████║██║██║ ╚████║╚██████╔╝╚██████╔╝███████╗██║  ██║██║  ██║██║   ██║      ██║   
  ╚══════╝╚═╝╚═╝  ╚═══╝ ╚═════╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝   ╚═╝      ╚═╝
");

    let port = find_available_port(8080);
    match singularity {
        Ok(config) => {
            // println!("After parsing -> {:?}", config);
            
            println!("🟡 Config file parsed successfully ⚙️");
            println!("\n🌀 Theme: {}", config.theme);
            println!("🌀 Background color: {}", config.theme_background_color);
            if let Err(e) = run_actix_server(port, config).await {
                eprintln!("Failed to run Actix server: {}", e);
            }
        },
        Err(err) => println!("Error in parsing singularity.yaml {}", err),
    }
    
    println!("\n🟡 Gracefully stopping...");
    println!("🟡 The Infinity is coming to an end...");
    println!("🟡 Singularity has been successfully stopped.");
    Ok(())
}