use sysinfo::{System, SystemExt, DiskExt, ProcessorExt};

fn main() {
    // Sistem bilgilerini almak için System nesnesini oluşturuyoruz
    let mut sys = System::new_all();

    // CPU bilgilerini yazdırıyoruz
    println!("CPU Bilgileri:");
    for processor in sys.processors() {
        println!("{}: {} MHz", processor.name(), processor.frequency());
    }

    // Bellek bilgilerini yazdırıyoruz
    println!("\nBellek Bilgileri:");
    println!("Toplam Bellek: {} MB", sys.total_memory() / 1024);
    println!("Kullanılan Bellek: {} MB", sys.used_memory() / 1024);
    println!("Boş Bellek: {} MB", sys.free_memory() / 1024);

    // Disk bilgilerini yazdırıyoruz
    println!("\nDisk Bilgileri:");
    for disk in sys.disks() {
        let used_space = disk.total_space() - disk.available_space();
        println!("Disk: {:?}, Toplam: {} MB, Kullanılan: {} MB, Boş: {} MB",
            disk.name(),
            disk.total_space() / 1024 / 1024,
            used_space / 1024 / 1024,
            disk.available_space() / 1024 / 1024
        );
    }
}