import { FluentProvider, webDarkTheme } from "@fluentui/react-components";
import React from "react";
import ReactDOM from "react-dom/client";
import AppContainer from "./AppContainer";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <FluentProvider theme={webDarkTheme}>
      <AppContainer />
    </FluentProvider>
  </React.StrictMode>
);
