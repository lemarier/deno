// You have access to Deno API (Similar to NodeJS but with SandBox)
const url_ =
  "https://gist.githubusercontent.com/lemarier/814d94ff68ff4d525c4d3d3cfeb3b00c/raw/adfda433fb3aacd685cb0af070b448ba114913d3/gistfile1.txt";

// Even fetch is available ;)
const res = await fetch(url_);
const body = new Uint8Array(await res.arrayBuffer());

// Write the output... await is available ass well <3 DENO
await Deno.stdout.write(body);

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
