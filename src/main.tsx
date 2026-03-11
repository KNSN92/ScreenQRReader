import ReactDom from "react-dom/client";
import React from "react";

import { getCurrentWindow } from "@tauri-apps/api/window";

let App;
const label = await getCurrentWindow().label;
switch (label) {
  case "qr_maker":
    App = (await import("./window/qr_maker")).App;
    break;
  case "updater":
    App = (await import("./window/updater")).App;
    break;
  default:
    App = () => <div>Unknown window</div>;
    break;
}

ReactDom.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
