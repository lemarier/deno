// Copyright 2018-2020 the Deno authors. All rights reserved. MIT license.

/// <reference no-default-lib="true" />
/// <reference lib="esnext" />

declare interface BrowserOptions {
  title: String;
  url: String;
  width: Number;
  height: Number;
  resizable: boolean;
  debug: boolean;
}

declare namespace Tauri {
  export function run(option: BrowserOptions): Promise<Number>;
}
