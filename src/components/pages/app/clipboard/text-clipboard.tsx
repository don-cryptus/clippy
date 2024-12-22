import Tooltip from "@corvu/tooltip";
import dayjs from "dayjs";
import { FiCode, FiFileText, FiLink } from "solid-icons/fi";
import { VsDebugRestartFrame } from "solid-icons/vs";
import { Component } from "solid-js";
import { SettingsStore } from "../../../../store/settings-store";
import { ClipboardWithRelations } from "../../../../types";
import { ClipboardTextType, ClipboardType } from "../../../../types/enums";
import { InvokeCommand } from "../../../../types/tauri-invoke";
import { rgbCompatible } from "../../../../utils/colors";
import { invokeCommand } from "../../../../utils/tauri";
import { ClipboardHeader } from "../../../utils/clipboard/clipboard-header";

interface TextClipboardProps {
  data: ClipboardWithRelations;
  index: number;
}

export const TextClipboard: Component<TextClipboardProps> = (props) => {
  let type = ClipboardType.Text;
  let data = props.data.text?.data;
  let textType = props.data.text?.type as ClipboardTextType;

  if (!props.data.text?.data && props.data.html?.data) {
    type = ClipboardType.Html;
    data = props.data.html.data;
  }
  if (props.data.text?.data && props.data.rtf?.data && props.data.html?.data) {
    type = ClipboardType.Rtf;
    data = props.data.rtf.data;
  }

  const getIcon = () => {
    if (type === ClipboardType.Html) {
      return FiCode;
    }
    if (type === ClipboardType.Rtf) {
      return VsDebugRestartFrame;
    }

    switch (textType) {
      case ClipboardTextType.Link:
        return FiLink;
      case ClipboardTextType.Hex:
        return () => (
          <div
            class="h-5 w-5 rounded-md border border-solid border-zinc-400 dark:border-black"
            style={{ "background-color": data?.includes("#") ? data : `#${data}` }}
          />
        );
      case ClipboardTextType.Rgb:
        return () => (
          <div
            class="h-5 w-5 rounded-md border border-solid border-zinc-400 dark:border-black"
            style={{ "background-color": rgbCompatible(data || "")! }}
          />
        );
      default:
        return FiFileText;
    }
  };

  const handleClick = async (e: MouseEvent) => {
    e.stopPropagation();
    await invokeCommand(InvokeCommand.CopyClipboard, {
      id: props.data.clipboard.id,
      type,
    });
  };

  return (
    <Tooltip openDelay={1000}>
      <Tooltip.Trigger>
        <button type="button" onClick={handleClick} class="clipboard relative">
          <ClipboardHeader {...props} Icon={getIcon()} />

          <div class="min-w-0 flex-1">
            <p class="w-[calc(100vw-6.5rem)] truncate text-left text-sm">{data}</p>
            <div
              class="text-left text-xs font-thin text-zinc-700 dark:text-zinc-300"
              title={new Date(props.data.clipboard.created_date).toLocaleString()}
            >
              {dayjs.utc(props.data.clipboard.created_date).fromNow()}
            </div>
          </div>
        </button>
      </Tooltip.Trigger>

      {SettingsStore.settings()?.tooltip && props.data.html?.data && (
        <Tooltip.Portal>
          <Tooltip.Content
            style={{ "max-width": "300px", "max-height": "300px" }}
            class="truncate rounded-md bg-white p-1 shadow-lg dark:bg-zinc-800"
          >
            <div innerHTML={props.data.html.data} />
          </Tooltip.Content>
        </Tooltip.Portal>
      )}
    </Tooltip>
  );
};
