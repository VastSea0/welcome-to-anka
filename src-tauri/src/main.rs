#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use sysinfo::{System, SystemExt, DiskExt, ProcessorExt};

#[tauri::command]
fn sayhelo(name: &str) -> String {
    format!("helo to {} from AnkaOS!!", name)
}

#[tauri::command]
async fn system_info() -> Result<String, String> {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let mut info = String::new();
    
    // CPU Bilgileri
    info.push_str("CPU Bilgileri:\n");
    for processor in sys.processors() {
        info.push_str(&format!("{}: {} MHz\n", processor.name(), processor.frequency()));
    }
    
    // Bellek Bilgileri
    info.push_str("\nBellek Bilgileri:\n");
    info.push_str(&format!("Toplam Bellek: {} MB\n", sys.total_memory() / 1024));
    info.push_str(&format!("Kullanılan Bellek: {} MB\n", sys.used_memory() / 1024));
    info.push_str(&format!("Boş Bellek: {} MB\n", sys.free_memory() / 1024));
    
    // Disk Bilgileri
    info.push_str("\nDisk Bilgileri:\n");
    for disk in sys.disks() {
        let used_space = disk.total_space() - disk.available_space();
        info.push_str(&format!(
            "Disk: {:?}, Toplam: {} GB, Kullanılan: {} GB, Boş: {} GB\n",
            disk.name(),
            disk.total_space() / 1024 / 1024 / 1024,
            used_space / 1024 / 1024 / 1024,
            disk.available_space() / 1024 / 1024 / 1024
        ));
    }
    
    Ok(info)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sayhelo, system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}