import { invoke } from "@tauri-apps/api";
import { IconTypes } from "solid-icons";
import { FaRegularKeyboard, FaRegularUser } from "solid-icons/fa";
import { IoCogSharp } from "solid-icons/io";
import { VsHistory } from "solid-icons/vs";
import { createRoot, createSignal } from "solid-js";
import { Hotkey, Settings } from "../@types";
import { parseShortcut, registerHotkeys } from "../utils/hotkeyRegister";

type SettingsTabName = "General" | "Account" | "History" | "Hotkeys";

type SettingsTab = {
  name: SettingsTabName;
  Icon: IconTypes;
  current: boolean;
};

function createSettingsStore() {
  const [isProduction, setIsProduction] = createSignal<boolean>(false); // process.env.NODE_ENV === "production" ? true : false
  const [globalHotkeyEvent, setGlobalHotkeyEvent] = createSignal<boolean>(true);
  const [hotkeys, setHotkeys] = createSignal<Hotkey[]>([]);
  const [settings, setSettings] = createSignal<Settings>();
  const [tabs, setTabs] = createSignal<SettingsTab[]>([
    { name: "General", Icon: IoCogSharp, current: true },
    { name: "Account", Icon: FaRegularUser, current: false },
    {
      name: "History",
      Icon: VsHistory,
      current: false,
    },
    {
      name: "Hotkeys",
      Icon: FaRegularKeyboard,
      current: false,
    },
  ]);

  const setCurrentTab = (tabName: SettingsTabName) =>
    setTabs((prev) =>
      prev.map((tab) => ({ ...tab, current: tab.name === tabName }))
    );

  const updateSettings = async (
    settings: Settings,
    upload: boolean | undefined = true
  ) => {
    if (upload) await invoke("updateSettings", settings);
    setSettings(settings);
  };

  const updateHotkey = async (
    hotkey: Hotkey,
    upload: boolean | undefined = true
  ) => {
    if (upload) await invoke("updateHotkey", hotkey as Hotkey);
    setHotkeys((prev) =>
      prev.map((h) => (h.id === hotkey.id ? { ...h, ...hotkey } : h))
    );
  };

  const initSettings = async () => {
    const isProduction = await invoke<boolean>("is_production");
    console.log("isProduction", isProduction);
    const settings = await invoke<Settings>("get_settings");
    setSettings(settings);
    setIsProduction(isProduction);
    await initHotkeys();
  };

  const initHotkeys = async () => {
    const hotkeys = (await invoke<Hotkey[]>("get_hotkeys")).map((h) => ({
      ...h,
      shortcut: parseShortcut(h),
    }));

    setHotkeys(hotkeys);
    registerHotkeys(hotkeys);
  };

  return {
    globalHotkeyEvent,
    setGlobalHotkeyEvent,
    hotkeys,
    setHotkeys,
    settings,
    setSettings,
    tabs,
    setTabs,
    setCurrentTab,
    updateSettings,
    updateHotkey,
    initSettings,
  };
}

export default createRoot(createSettingsStore);
