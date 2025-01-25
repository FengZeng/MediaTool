import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";

import "/src/style.css";

const loadingOverlay = document.getElementById("loading-overlay");

listen("MediaInfo", (event) => {
  loadingOverlay.classList.add("hidden");
  const messageElement = document.getElementById("MediaInfo");
  messageElement.textContent = event.payload;
  messageElement.style.display = "flex";
});

async function openFilePicker() {
  try {
    const selectedFile = await open({
      multiple: false, // Set to true if you want to allow multiple file selection
      filters: [
        { name: "Text Files", extensions: ["txt", "mp4", "mkv", "mov", "flv"] },
        { name: "All Files", extensions: ["*"] },
      ],
    });

    if (selectedFile) {
      const messageElement = document.getElementById("MediaInfo");
      messageElement.textContent = "";
      loadingOverlay.classList.remove("hidden");
      document.getElementById("MediaPath").textContent = selectedFile;
      document.getElementById("inputOverlay").style.display = "none";
      document.getElementById("openFileButton").style.display = "none";
      document.getElementById("openFileUrlButton").style.display = "flex";
      console.log("Selected file:", selectedFile);
      // You can now read the file, pass it to a backend, etc.
      await invoke("get_media_info", { url: selectedFile });
    } else {
      console.log("No file selected");
    }
  } catch (error) {
    console.error("Error opening file picker:", error);
  }
}

document
  .getElementById("openFileButton")
  .addEventListener("click", openFilePicker);

const plusSpan = document.getElementById("openFileSpan");
const dropdownSpan = document.getElementById("openUrlSpan");

plusSpan.addEventListener("click", openFilePicker);

function handleClickOutside(event) {
  const button = document.getElementById("urlInputDiv");
  if (button && !button.contains(event.target)) {
    document.getElementById("inputOverlay").style.display = "none";
  }
}

function showInputUrl() {
  document.getElementById("urlInputDiv").style.top = "20px";
  document.getElementById("urlInputDiv").style.backgroundColor = "#2f2f2f";
  document.getElementById("inputOverlay").style.display = "flex";
  document.addEventListener("click", handleClickOutside);
}

function hideInputUrl() {
  document.getElementById("inputOverlay").style.display = "none";
  document.removeEventListener("click", handleClickOutside);
}

dropdownSpan.addEventListener("click", function (event) {
  event.stopPropagation();
  showInputUrl();
});

const urlInput = document.getElementById("url-input");
const urlSend = document.getElementById("url-send");

urlSend.addEventListener("click", async () => {
  const query = urlInput.value; // Use urlInput instead of copilotInput
  if (query.trim() !== "") {
    // Clear input field after sending the query
    urlInput.value = "";

    // Ensure that invoke() is properly awaited
    try {
      document.getElementById("inputOverlay").style.display = "none";
      document.getElementById("openFileButton").style.display = "none";
      document.getElementById("openFileUrlButton").style.display = "flex";
      await invoke("get_media_info", { url: query });
    } catch (error) {
      console.error("Error invoking backend:", error);
    }
  }
});

urlInput.addEventListener("keydown", (event) => {
  if (event.key === "Enter") {
    urlSend.click(); // Trigger click on the send button when Enter is pressed
  }
});

document.addEventListener("keydown", (event) => {
  if (event.code === "Space") {
    event.preventDefault();
    const mediaInfoElement = document.getElementById("openFileButton");

    if (mediaInfoElement.style.display === "none") {
      const div = document.getElementById("inputOverlay");
      if (div.style.display === "none") {
        showInputUrl();
      } else {
        hideInputUrl();
      }
    }
  }
});

window.addEventListener("DOMContentLoaded", () => {
  // Hide the loading overlay when the initial content is loaded.
  loadingOverlay.classList.add("hidden");
});
