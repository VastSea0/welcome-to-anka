const { invoke } = window.__TAURI__.tauri;

let heloInputEl;
let heloMsgEl;

async function sayhelo() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  heloMsgEl.textContent = await invoke("sayhelo", { name: heloInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  heloInputEl = document.querySelector("#greet-input");
  heloMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    sayhelo();
  });
});


