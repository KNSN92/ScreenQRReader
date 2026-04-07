import ReactDom from "react-dom/client";
import React from "react";

import { getCurrentWindow } from "@tauri-apps/api/window";
import { I18nProvider } from "./hooks/i18n";

function DefaultView() {
  return <div>Unknown window</div>;
}

function View() {
  const [View, setView] = React.useState<React.FC>(() => DefaultView);
  const label = getCurrentWindow().label;
  switch (label) {
    case "qr_maker":
      import("./views/qr_maker").then((module) =>
        setView(() => module.QRMakerView),
      );
      break;
    case "updater":
      import("./views/updater").then((module) =>
        setView(() => module.UpdaterView),
      );
      break;
  }
  return <View />;
}

ReactDom.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <I18nProvider>
      <View />
    </I18nProvider>
  </React.StrictMode>,
);
