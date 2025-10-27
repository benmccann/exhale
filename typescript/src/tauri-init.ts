// Tauri initialization - handles window opacity setup
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

async function initializeWindow() {
  const tauriWindow = getCurrentWebviewWindow();

  // Get opacity from localStorage and set it
  const opacity = localStorage.getItem("opacity");
  if (opacity) {
    try {
      await invoke("set_window_opacity", {
        window: tauriWindow,
        opacity: Number(opacity)
      });
    } catch (error) {
      console.error("Failed to set window opacity:", error);
    }
  }
}

// Initialize when the page loads
initializeWindow();
