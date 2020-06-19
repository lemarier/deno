import { open_webview, BrowserOptions } from "./ops/tauri.ts";

export function run(option: BrowserOptions): Promise<Number> {
  console.log("running'!!");
  console.log(option);
  return open_webview(option);
}
