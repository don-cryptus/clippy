import { listen } from "@tauri-apps/api/event";
import { sendNotification } from "@tauri-apps/api/notification";
import { appWindow } from "@tauri-apps/api/window";
import { createResource, onMount } from "solid-js";
import { render } from "solid-js/web";
import { Clips } from "./@types";
import App from "./components/pages/app/App";
import ClipboardStore from "./store/ClipboardStore";
import HotkeyStore from "./store/HotkeyStore";
import SettingsStore from "./store/SettingsStore";
import "./styles.css";
import { removeAllHotkeyListeners } from "./utils/hotkeyRegister";

const Index = () => {
  const { initHotkeys } = HotkeyStore;
  const { setClipboards } = ClipboardStore;
  const { init, settings } = SettingsStore;

  createResource(init);

  onMount(async () => {
    appWindow.onFocusChanged(async ({ payload }) => {
      if (!payload) {
        await appWindow.hide();
        removeAllHotkeyListeners();
      }
    });

    listen<Clips>("clipboard_listener", ({ payload }) => {
      settings()?.notification &&
        sendNotification({
          title: `New ${payload.type}`,
          body: "Copied to clipboard",
        });
      setClipboards((prev) => [payload, ...prev]);
    });

    listen("init_listener", init);

    listen("init_hotkeys_listener", () => initHotkeys(true));
  });

  return <App />;
};

render(() => <Index />, document.getElementById("root") as HTMLElement);
