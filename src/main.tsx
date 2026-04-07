import ReactDom from "react-dom/client";
import React from "react";

import { getCurrentWindow } from "@tauri-apps/api/window";

function DefaultView() {
  return <div>Unknown window</div>;
}

function View() {
  const [View, setView] = React.useState<React.FC>(DefaultView);
  const label = getCurrentWindow().label;
  switch (label) {
    case "qr_maker":
      import("./view/qr_maker").then((module) =>
        setView(() => module.QRMakerView),
      );
      break;
    case "updater":
      import("./view/updater").then((module) =>
        setView(() => module.UpdaterView),
      );
      break;
  }
  return <View />;
}

ReactDom.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <View />
  </React.StrictMode>,
);
