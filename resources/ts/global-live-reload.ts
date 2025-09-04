import { Idiomorph } from "idiomorph";

const DEBOUNCE_MILLIS = 1000;
const DOCTYPE = "<!DOCTYPE html>";

const poetLiveReloadSymbol = Symbol("POET_LIVE_RELOAD");

let currentLiveReloadPath: string = window.location.pathname;
let intendsClose = false;
let liveReloadSocket: null | WebSocket = null;

function onUpdatedHTML(updatedHTML: string) {
  if (updatedHTML.startsWith(DOCTYPE)) {
    updatedHTML = updatedHTML.substring(DOCTYPE.length);
  }

  Idiomorph.morph(document.documentElement, updatedHTML, {
    head: {
      style: "morph",
    },
  });
}

function keepSocketAlive() {
  liveReloadSocket = new WebSocket(
    `/api/v1/live_reload${currentLiveReloadPath}`,
  );

  liveReloadSocket.onclose = function (event) {
    if (!intendsClose) {
      console.warn("[poet] live reload socket closed", event);
    }

    liveReloadSocket = null;

    if (!intendsClose) {
      setTimeout(keepSocketAlive, DEBOUNCE_MILLIS);
    } else {
      setTimeout(keepSocketAlive);
    }

    intendsClose = false;
  };

  liveReloadSocket.onmessage = function (event) {
    let updatedHTML = event.data.trim();

    onUpdatedHTML(updatedHTML);
  };

  liveReloadSocket.onerror = function (event) {
    console.error("[poet] live reload socket failed", event);

    liveReloadSocket?.close();
  };
}

function setupLiveReload() {
  if ((globalThis as unknown as any)[poetLiveReloadSymbol]) {
    return;
  }

  (globalThis as unknown as any)[poetLiveReloadSymbol] = true;

  console.log("[poet] setting up live reload");

  currentLiveReloadPath = window.location.pathname;

  keepSocketAlive();
}

setupLiveReload();
