import { open_webview } from "./ops/tauri.ts";

interface BrowserOptions {
  title: String;
  url: String;
  width: Number;
  height: Number;
  resizable: boolean;
  debug: boolean;
}

export function run(option: BrowserOptions): Promise<Number>;

export function run(option: BrowserOptions): Promise<Number> {
  console.log("running'!!");
  console.log(option);
  return open_webview(option);
}
