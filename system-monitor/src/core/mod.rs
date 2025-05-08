
use serde::Serialize;
use sysinfo::System;


#[derive(Serialize)]
pub struct SystemOSData{
    name: String,
    kernel_version: String,
    os: String,
    host: String
}

#[derive(Serialize)]
pub struct SystemLoad {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    process_running: u64,

}


#[derive(Serialize)]
pub struct ProcessData {
    pid: u32,
    cpu_usage: f32,
    memory_usage: u64,
}

#[derive(Serialize)]
pub struct SystemMonitor {
    process: Vec<ProcessData>,
}


pub fn get_system_monitor(system: &mut System) -> SystemMonitor {
    system.refresh_cpu_usage();
    let mut result = Vec::new();
    for (pid, process) in system.processes(){
        result.push(ProcessData{
            pid: pid.as_u32(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.disk_usage().total_written_bytes,
        });
    }
    SystemMonitor { process: result }
}

pub fn get_os_data() -> SystemOSData {
    SystemOSData {
        name: System::name().unwrap_or_else(|| "Unknown".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
        os: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
        host: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
    }
}
