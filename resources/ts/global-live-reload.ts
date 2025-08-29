import { Idiomorph } from "idiomorph";

const DOCTYPE = "<!DOCTYPE html>";

function setupLiveReload() {
  if ((globalThis as unknown as any).isLiveReloadSetup) {
    return;
  }

  (globalThis as unknown as any).isLiveReloadSetup = true;

  console.log(`Seting up live reload for ${window.location.pathname}`);

  const liveReloadSocket = new WebSocket(
    `/api/v1/live_reload${window.location.pathname}`,
  );

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
    console.error("Live reload socket failed:", event);
  };

  liveReloadSocket.onopen = function (event) {
    console.log("Live reload socket connected:", event);
  };
}

setupLiveReload();
