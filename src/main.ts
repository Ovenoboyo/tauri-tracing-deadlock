import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";

async function greet() {
	console.log("calling greet");
	const resp = await invoke("greet");
	console.log(resp);
	throw new Error("hello");
}

setInterval(() => {
	greet();
}, 50);

listen("hello", () => {
	emit("bye");
});

window.onunhandledrejection = (ev) => {
	console.error("Uncaught in promise", ev);
};

// greet();
