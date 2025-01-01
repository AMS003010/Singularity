use std::fs;
use std::io::Error as IOError;
use std::sync::Arc;
use std::time::Duration;
use actix_files as fs_actix;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_yaml::Result as SerdeResult;
use internals::singularity::Config;
use internals::port::find_available_port;
use internals::render::final_yaml_to_html_render;
use internals::cache::{GenericWidgetCache, convert_cache_ttl_to_seconds};
use feed::header_data::get_system_stats;

//TODO: Implementing COW (Clone On Write) wherever possible

//TODO: Implementing a initial Static HTML render like React / Nextjs and then rehydrate it

//TODO: Add a path for .executable approach like 'singularity -p /test/singularity.yaml'
//      and path of '/singularity.yaml' for container approach with volumes enabled

//TODO: ---FUTURE--- Implementing PGO & LTO

//TODO: ---FUTURE--- Replacing Actix-web and hyper with tokio

//TODO: Try checking if weather widget is already there then dont inject the time updation script

//TODO: Maybe add a position system to make the widget injection faster

// TODO: Going with a simple rendering method, find a better method for faster parse and render

mod widgets {
    pub mod weather;
    pub mod clock;
    pub mod calendar;
    pub mod header;
    pub mod youtube;
}

mod feed {
    pub mod weather_data;
    pub mod clock_data;
    pub mod calendar_data;
    pub mod header_data;
    pub mod youtube_data;
}

mod internals {
    pub mod singularity;
    pub mod render;
    pub mod port;
    pub mod cache;
}

async fn stats_page() -> impl Responder {
    let stats = get_system_stats();
    web::Json(stats)
}

async fn render_page(
    page_name: web::Path<String>,
    config: web::Data<Config>,
    widget_cache: web::Data<Arc<GenericWidgetCache>>,   
) -> impl Responder {
    
    // Dynamically Render Actix endpint handler 
    let page_name = page_name.into_inner();
    if let Some(page) = config.pages.iter().find(|p| p.name == page_name) {
        let final_html = String::new();
        let rendered_html = final_yaml_to_html_render(&config, final_html, &widget_cache, page.name.clone()).await;
        HttpResponse::Ok().content_type("text/html").body(rendered_html)
    } else {
        HttpResponse::NotFound().body(format!("Page '{}' not found", page_name))
    }
}

async fn run_actix_server(port: u16, config: Config) -> std::io::Result<()> {
    let ttl = convert_cache_ttl_to_seconds(config.cache.clone().unwrap_or_default());
    let widget_cache = Arc::new(GenericWidgetCache::new(Duration::from_secs(ttl)));

    // Clone the config for use inside the closure
    let config_clone = config.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config_clone.clone()))
            .app_data(web::Data::new(widget_cache.clone()))
            .route("/stats", web::get().to(stats_page))
            .route("/pages/{page_name}", web::get().to(render_page))
            .service(fs_actix::Files::new("/static", "src/assets/static").show_files_listing())
    })
    .bind(format!("0.0.0.0:{}", port))?
    .workers(4)
    .shutdown_timeout(60)
    .run();

    if let Some(first_page) = config.pages.get(0) {
        println!(
            "🌀 Singularity running at http://127.0.0.1:{}/pages/{}\n",
            port, first_page.name
        );
    } else {
        println!("🟤 Mention at least 1 page in the config !!");
    }

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
        Ok(mut config) => {
            // println!("After parsing -> {:#?}", config);
            
            config.port = Some(port);
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