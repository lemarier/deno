// Create our new window
const browserWindow = {
  title: "OK",
  url: "https://deno.land",
  width: 800,
  height: 600,
  resizable: false,
  debug: true,
};

// You have full access to tauri api who can interface with Rust Easilly
// By example we'll make an onReady event where you can send command (callback)
await Tauri.run(browserWindow);
