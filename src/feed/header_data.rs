use serde::Serialize;
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworksExt, NetworkExt};

#[derive(Serialize)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub disks: Vec<DiskInfo>,
    pub network: Vec<NetworkInfo>,
    pub system_info: SystemInfo,
    pub no_of_cpus: usize,
}

#[derive(Serialize)]
pub struct DiskInfo {
    pub mount: String,
    pub available_space: u64,
}

#[derive(Serialize)]
pub struct NetworkInfo {
    pub interface_name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}

#[derive(Serialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    kernel_version: String,
    uptime: u64,
    pub host_name: String,
}

pub fn get_system_stats() -> SystemStats {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Collect disk info
    let disks = sys.disks().iter().map(|disk| DiskInfo {
        mount: disk.mount_point().to_string_lossy().to_string(),
        available_space: disk.available_space(),
    }).collect();

    // Collect network info
    let network = sys.networks().iter().map(|(name, data)| NetworkInfo {
        interface_name: name.clone(),
        received_bytes: data.received(),
        transmitted_bytes: data.transmitted(),
    }).collect();

    // Collect system info
    let system_info = SystemInfo {
        os_name: sys.name().unwrap_or_else(|| "Unknown".to_string()),
        os_version: sys.os_version().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: sys.kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        uptime: sys.uptime(),
        host_name: sys.host_name().unwrap_or_else(|| "Unknown".to_string()),
    };

    let stats = SystemStats {
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        disks,
        network,
        system_info,
        no_of_cpus: sys.cpus().len(),
    };

    stats
}