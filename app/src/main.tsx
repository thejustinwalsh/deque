import React, { useEffect, useState } from "react";
import { ReactQueryDevtoolsPanel } from "@tanstack/react-query-devtools";
import { listen } from "@tauri-apps/api/event";
import { RSPCProvider, client, queryClient } from "hooks/rspc";
import ReactDOM from "react-dom/client";
import { IntlProvider } from "react-intl";
import { Portal } from "react-portal";
import App from "./App";
import "styles/main.css";

const handleTranslationError = (e: any) => {
  if (e?.code !== "MISSING_TRANSLATION") console.error(e);
};

const DevTools = () => {
  const [showDevTools, setShowDevTools] = useState(false);

  const handleDragStart = (startEvent: React.MouseEvent<HTMLDivElement, MouseEvent>) => {
    const panelElement = startEvent.currentTarget.parentElement;
    if (!panelElement) return;
    if (startEvent.button !== 0) return;

    const { height } = panelElement.getBoundingClientRect();
    const startY = startEvent.clientY;
    let newSize = 0;

    const run = (moveEvent: MouseEvent) => {
      // prevent mouse selecting stuff with mouse drag
      moveEvent.preventDefault();
      newSize = height + startY - moveEvent.clientY;
      panelElement.style.height = `${newSize}px`;
    };

    const remove = () => {
      document.removeEventListener("mousemove", run, false);
      document.removeEventListener("mouseUp", remove, false);
    };

    document.addEventListener("mousemove", run, false);
    document.addEventListener("mouseup", remove, false);
  };

  useEffect(() => {
    const keyBindListener = listen("keybind", (event) => {
      switch (event.payload as string) {
        case "reload_app":
          window.location.reload();
          break;
        case "toggle_rq_devtools":
          setShowDevTools((s) => !s);
          break;
      }
    });

    return () => void keyBindListener.then((remove) => remove());
  }, []);

  if (!showDevTools) return null;

  return (
    <Portal>
      <ReactQueryDevtoolsPanel
        isOpen={true}
        setIsOpen={() => {}}
        onDragStart={handleDragStart}
        showCloseButton={false}
        style={{
          position: "absolute",
          bottom: 0,
          left: 0,
          right: 0,
          height: "300px",
        }}
      />
    </Portal>
  );
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <IntlProvider
      locale={navigator.language}
      key={navigator.language}
      onError={handleTranslationError}
    >
      <RSPCProvider client={client} queryClient={queryClient}>
        <>
          <App />
          {import.meta.env.DEV && <DevTools />}
        </>
      </RSPCProvider>
    </IntlProvider>
  </React.StrictMode>
);
