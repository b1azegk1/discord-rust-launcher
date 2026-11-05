import { Webview } from "@tauri-apps/api/webview";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LogicalPosition, LogicalSize } from "@tauri-apps/api/dpi";
import "./style.css";

const appWindow = getCurrentWindow();
const $ = (selector) => document.querySelector(selector);
let discordView;

function toast(message) {
  const el = $("#toast");
  el.textContent = message;
  el.classList.add("visible");
  clearTimeout(toast.timer);
  toast.timer = setTimeout(() => el.classList.remove("visible"), 4000);
}

async function createDiscordView() {
  const scale = await appWindow.scaleFactor();
  const size = await appWindow.innerSize();
  discordView = new Webview(appWindow, "discord-webview", {
    url: "https://discord.com/app",
    x: 252,
    y: 58,
    width: Math.max(320, Math.floor(size.width / scale) - 252),
    height: Math.max(300, Math.floor(size.height / scale) - 58),
    focus: true,
    backgroundColor: "#313338"
  });
  discordView.once("tauri://created", () => {
    $("#loading").classList.add("hidden");
    toast("Discord Web открыт внутри лаунчера");
  });
  discordView.once("tauri://error", (event) => {
    $("#loading").classList.remove("hidden");
    $("#loading h1").textContent = "Не удалось открыть Discord";
    $("#loading p").textContent = String(event.payload || "Проверь интернет-соединение.");
    toast("Ошибка загрузки Discord Web");
  });
}

async function resizeDiscordView() {
  if (!discordView) return;
  const scale = await appWindow.scaleFactor();
  const size = await appWindow.innerSize();
  await discordView.setPosition(new LogicalPosition(252, 58));
  await discordView.setSize(new LogicalSize(Math.max(320, Math.floor(size.width / scale) - 252), Math.max(300, Math.floor(size.height / scale) - 58)));
}

$("#reload-button").addEventListener("click", async () => {
  if (discordView) {
    try { await discordView.close(); } catch {}
    discordView = undefined;
  }
  $("#loading").classList.remove("hidden");
  $("#loading h1").textContent = "Переподключаем Discord";
  $("#loading p").textContent = "Загружаем официальный Web-клиент…";
  await createDiscordView();
});

appWindow.onResized(() => resizeDiscordView().catch(() => {}));
createDiscordView().catch((error) => {
  $("#loading h1").textContent = "Не удалось запустить WebView";
  $("#loading p").textContent = String(error);
  toast("Ошибка создания встроенного окна Discord");
});
