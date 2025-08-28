import { Idiomorph } from "idiomorph";

const DOCTYPE = "<!DOCTYPE html>";

setTimeout(function () {
  if ((globalThis as unknown as any).isLiveReloadSetup) {
    return;
  }

  (globalThis as unknown as any).isLiveReloadSetup = true;

  console.log(`Seting up live reload for ${window.location.pathname}`);

  const eventSource = new EventSource(
    `/api/v1/live_reload${window.location.pathname}`,
  );

  eventSource.onmessage = function (event) {
    let updatedHTML = event.data;

    if (updatedHTML.startsWith(DOCTYPE)) {
      updatedHTML = updatedHTML.substring(DOCTYPE.length);
    }

    Idiomorph.morph(document.documentElement, updatedHTML, {
      head: {
        style: "morph",
      },
    });
  };

  eventSource.onerror = function (event) {
    console.error("Live reload EventSource failed:", event);
  };

  eventSource.onopen = function (event) {
    console.log("Live reload EventSource connected:", event);
  };
});
