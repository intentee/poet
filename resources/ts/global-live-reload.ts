import { Idiomorph } from "idiomorph";

const DEBOUNCE_MILLIS = 1000;
const DOCTYPE = "<!DOCTYPE html>";
const LIVE_RELOAD_API_URL = `/api/v1/live_reload${window.location.pathname}`;

function keepSocketAlive() {
  const liveReloadSocket = new WebSocket(LIVE_RELOAD_API_URL);

  liveReloadSocket.onclose = function (event) {
    console.warn("[poet] live reload socket closed", event);

    setTimeout(keepSocketAlive, DEBOUNCE_MILLIS);
  };

  liveReloadSocket.onmessage = function (event) {
    let updatedHTML = event.data.trim();

    if (updatedHTML.startsWith(DOCTYPE)) {
      updatedHTML = updatedHTML.substring(DOCTYPE.length);
    }

    Idiomorph.morph(document.documentElement, updatedHTML, {
      head: {
        style: "morph",
      },
    });
  };

  liveReloadSocket.onerror = function (event) {
    console.error("[poet] live reload socket failed", event);

    liveReloadSocket.close();
  };

  liveReloadSocket.onopen = function (event) {
    console.log("[poet] live reload socket connected", event);
  };
}

function setupLiveReload() {
  if ((globalThis as unknown as any).isLiveReloadSetup) {
    return;
  }

  (globalThis as unknown as any).isLiveReloadSetup = true;

  console.log(`[poet] setting up live reload for ${window.location.pathname}`);

  keepSocketAlive();
}

setupLiveReload();
