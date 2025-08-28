function startLiveReload() {
  globalThis._IS_POET_LIVE_RELOAD_RUNNING = true;

  const thisScriptElement = document.getElementById("poet-live-reload");

  if (!(thisScriptElement instanceof HTMLScriptElement)) {
    throw new Error("Could not find script element with id 'poet-live-reload'");
  }

  const liveReloadPath = thisScriptElement.dataset["relativePath"];

  if (typeof liveReloadPath !== "string") {
    throw new Error("Could not determine the live reload path");
  }

  const eventSource = new EventSource(`/api/v1/live_reload/${liveReloadPath}`);

  eventSource.onmessage = function (event) {
    document.open();
    document.write(event.data);
    document.close();
  };

  eventSource.onerror = function (event) {
    console.error("Live reload EventSource failed:", event);
  };
}

if (!globalThis._IS_POET_LIVE_RELOAD_RUNNING) {
  startLiveReload();
}
