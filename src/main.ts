import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

async function greet() {
	await invoke("greet");
}

listen("hello", () => {
	console.log("received event")
	greet()
});

window.onunhandledrejection = (ev) => {
	console.error("Uncaught in promise", ev);
};
