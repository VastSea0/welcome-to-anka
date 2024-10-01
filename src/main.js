const { invoke } = window.__TAURI__.tauri;

// Global değişkenler
let heloInputEl;
let heloMsgEl;

// Selamlama fonksiyonu
async function sayhelo() {
    heloMsgEl.textContent = await invoke("sayhelo", { name: heloInputEl.value });
}



// Sayfa yüklendiğinde çalışacak kodlar
window.addEventListener("DOMContentLoaded", () => {
    // Form elementlerini bul
    heloInputEl = document.querySelector("#greet-input");
    heloMsgEl = document.querySelector("#greet-msg");
    
    // Form submit event listener'ı ekle
    const greetForm = document.querySelector("#greet-form");
    if (greetForm) {
        greetForm.addEventListener("submit", (e) => {
            e.preventDefault();
            sayhelo();
        });
    }
    
    // İlk ekranı göster
    showScreen(currentScreen);
});