import ReactDom from "react-dom/client";
import { App } from "./app";
import React from "react";

ReactDom.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
)