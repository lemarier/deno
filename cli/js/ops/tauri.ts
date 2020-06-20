import { sendAsync } from "./dispatch_json.ts";

export interface BrowserOptions {
  title: String;
  url: String;
  width: Number;
  height: Number;
  resizable: boolean;
  debug: boolean;
}

export function close(path: Number): void {
  //sendAsync("op_mkdir");
}

export async function openWebview(
  options: BrowserOptions,
): Promise<Number> {
  console.log("DONE HERE", options);
  return await sendAsync("op_webview_start", options);
}
