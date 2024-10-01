use std::process::Command;

fn main() {
    let output = Command::new("ls")
        .arg("-la")
        .output()
        .expect("Komut çalıştırılamadı.");

    // Komutun çıktısını ekrana bas
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Komut Çıktısı:\n{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Hata:\n{}", stderr);
    }
}
