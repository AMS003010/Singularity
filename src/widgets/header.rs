use crate::feed::header_data::{SystemStats, get_system_stats};
use crate::internals::singularity::WidgetError;
use crate::internals::render::{read_html_file, render_final_template, TempData};
use crate::internals::cache::GenericWidgetCache;
use std::collections::HashMap;
use std::sync::Arc;
use actix_web::web;

fn truncate_decimal(input: f32, decimal_places: usize) -> String {
    let factor = 10f32.powi(decimal_places as i32);
    let truncated = (input * factor).trunc() / factor;
    format!("{:.1$}", truncated, decimal_places)
}

pub async fn header_widget_handler(
    theme: String,
    _widget_cache: web::Data<Arc<GenericWidgetCache>>
) -> Result<String, WidgetError> {

    const WIDGET_NAME: &str = "header_widget";

    match _widget_cache.get(WIDGET_NAME).await {
        Ok(Some(cached_html)) => {
            // Cache HIT
            return Ok(cached_html);
        }
        Ok(None) => {
            // Cache MISS
        }
        Err(e) => {
            eprintln!("Cache retrieval error: {}", e);
        }
    }

    let mut template_data: HashMap<String, TempData> = HashMap::new();
    let stats: SystemStats = get_system_stats();
    let mut count: i32 = 0;
    let mut connectivity: i32 = 0;

    // Set the network status
    let wifi_active = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" class=\"lucide lucide-wifi size-5\"><path d=\"M12 20h.01\"/><path d=\"M2 8.82a15 15 0 0 1 20 0\"/><path d=\"M5 12.859a10 10 0 0 1 14 0\"/><path d=\"M8.5 16.429a5 5 0 0 1 7 0\"/></svg>";
    let wifi_inactive = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"24\" height=\"24\" viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" class=\"lucide lucide-wifi-off size-5\"><path d=\"M12 20h.01\"/><path d=\"M8.5 16.429a5 5 0 0 1 7 0\"/><path d=\"M5 12.859a10 10 0 0 1 5.17-2.69\"/><path d=\"M19 12.859a10 10 0 0 0-2.007-1.523\"/><path d=\"M2 8.82a15 15 0 0 1 4.177-2.643\"/><path d=\"M22 8.82a15 15 0 0 0-11.288-3.764\"/><path d=\"m2 2 20 20\"/></svg>";

    for network in stats.network.iter() {
        if network.interface_name == "Wi-Fi" {
            connectivity = 1;
        }
    }

    for disk in stats.disks.iter() {
        count += 1;
        let mount_key = format!("mount{}", count);
        let avai_space_key = format!("available_space{}", count);

        // Convert available space to GB and truncate
        let avai_space_gb = disk.available_space as f32 / 1e9; // Use f32 for compatibility
        let avai_space_str = truncate_decimal(avai_space_gb, 2);
        let avai_space_formatted = format!("{} GB", avai_space_str);

        template_data.insert(mount_key, TempData::Text(disk.mount.clone()));
        template_data.insert(avai_space_key, TempData::Text(avai_space_formatted));
    }

    for i in count..8 {
        let mount_key = format!("mount{}", i + 1);
        let avai_space_key = format!("available_space{}", i + 1);
        template_data.insert(mount_key, TempData::Text("--".to_string()));
        template_data.insert(avai_space_key, TempData::Text("--".to_string()));
    }

    if connectivity == 1 {
        template_data.insert("wifi_status".to_string(), TempData::Text(wifi_active.to_string()));
    } else {
        template_data.insert("wifi_status".to_string(), TempData::Text(wifi_inactive.to_string()));   
    }

    let truncated_cpu_usage = truncate_decimal(stats.cpu_usage, 1);
    let formatted_os_info = format!("{} {}", stats.system_info.os_name, stats.system_info.os_version);

    template_data.insert("widget_theme".to_string(), TempData::Text(theme));
    template_data.insert("cpu_usage".to_string(), TempData::Text(truncated_cpu_usage));
    template_data.insert("os_info".to_string(), TempData::Text(formatted_os_info));
    template_data.insert("system_info_hostname".to_string(), TempData::Text(stats.system_info.host_name));
    template_data.insert("cpu_numbers".to_string(), TempData::Text(stats.no_of_cpus.to_string()));

    match read_html_file("src/assets/templates/header.html") {
        Ok(inner_html) => {
            let rendered_html = render_final_template(inner_html, template_data);
            match _widget_cache.insert(WIDGET_NAME.to_string(), rendered_html.clone()).await {
                Ok(_) => {
                    // Inserted to Cache
                }
                Err(e) => {
                    eprintln!("Failed to insert widget into cache: {}", e);
                }
            }
            Ok(rendered_html)
        }
        Err(e) => {
            eprintln!("Error in reading header HTML file: {}", e);
            Err(WidgetError::NoHtmlToString)
        }
    }
}
