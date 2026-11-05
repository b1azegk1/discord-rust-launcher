import { invoke } from "@tauri-apps/api/core";
import "./style.css";

const channels = ["stable", "ptb", "canary"];
const names = { stable: "Discord Stable", ptb: "Discord PTB", canary: "Discord Canary" };
const $ = (selector) => document.querySelector(selector);
let installations = {};

function toast(message, error = false) {
  const el = $("#toast");
  el.textContent = message;
  el.classList.toggle("error", error);
  el.classList.add("visible");
  window.clearTimeout(toast.timer);
  toast.timer = window.setTimeout(() => el.classList.remove("visible"), 3600);
}

function setGlobalStatus(message) {
  $("#global-status").textContent = message;
}

function renderInstallations(data) {
  installations = data;
  for (const channel of channels) {
    const item = data[channel];
    const state = $(`#state-${channel}`);
    state.classList.toggle("installed", item.installed);
    state.classList.toggle("missing", !item.installed);
    state.querySelector("span").textContent = item.installed
      ? `Найдено${item.version ? ` · ${item.version}` : ""}`
      : "Не установлено";
    const button = document.querySelector(`[data-launch="${channel}"]`);
    button.disabled = !item.installed;
    button.title = item.installed ? `Запустить ${names[channel]}` : "Эта версия не найдена";
  }
}

async function refresh() {
  setGlobalStatus("Поиск установленных версий...");
  try {
    const result = await invoke("detect_discord");
    renderInstallations(result);
    const count = channels.filter((c) => result[c].installed).length;
    setGlobalStatus(`Обнаружено версий: ${count} из 3`);
  } catch (error) {
    setGlobalStatus("Не удалось выполнить обнаружение");
    toast(String(error), true);
  }
}

async function launch(channel) {
  const item = installations[channel];
  if (!item?.installed) {
    toast(`${names[channel]} не установлена`, true);
    return;
  }
  const args = $("#launch-args").value.trim();
  const button = document.querySelector(`[data-launch="${channel}"]`);
  button.classList.add("busy");
  button.disabled = true;
  try {
    const message = await invoke("launch_discord", { channel, args });
    toast(message);
    setGlobalStatus(`${names[channel]} запущена`);
  } catch (error) {
    toast(String(error), true);
  } finally {
    button.classList.remove("busy");
    button.disabled = !installations[channel]?.installed;
  }
}

async function checkUpdates() {
  const installed = channels.filter((c) => installations[c]?.installed);
  if (!installed.length) {
    toast("Сначала установи Discord Stable, PTB или Canary", true);
    return;
  }
  $("#update-button").disabled = true;
  setGlobalStatus("Проверка обновлений...");
  try {
    const results = await invoke("check_updates", { channels: installed });
    toast(results.join(" · "));
    setGlobalStatus("Запросы проверки обновлений отправлены");
  } catch (error) {
    toast(String(error), true);
    setGlobalStatus("Ошибка проверки обновлений");
  } finally {
    $("#update-button").disabled = false;
  }
}

document.querySelectorAll(".channel-card").forEach((card) => {
  card.addEventListener("click", (event) => {
    if (event.target.closest("button")) return;
    document.querySelectorAll(".channel-card").forEach((el) => el.classList.remove("selected"));
    card.classList.add("selected");
  });
});

document.querySelectorAll("[data-launch]").forEach((button) => {
  button.addEventListener("click", () => launch(button.dataset.launch));
});
$("#refresh-button").addEventListener("click", refresh);
$("#update-button").addEventListener("click", checkUpdates);

refresh();
