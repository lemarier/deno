export { BrowserOptions } from "./ops/tauri.ts";
import { openWebview, BrowserOptions } from "./ops/tauri.ts";

export function run(option: BrowserOptions): Promise<Number> {
  return openWebview(option);
}
