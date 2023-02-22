const { invoke } = window.__TAURI__.tauri;

let inputEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("greet", { name: inputEl.value });
  inputEl.value = "";
}

window.addEventListener("DOMContentLoaded", () => {
  inputEl = document.getElementById("launcher");
  inputEl.addEventListener(
    "keyup",
    (event) => {
      if (event.key === "Enter") {
        greet();
      }
  });
});
