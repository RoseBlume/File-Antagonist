
import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";

export const [dir, setDir] = createSignal("C:\\Users\\James");
export const [searching, setSearching] = createSignal(false);
export const [data, setData] = createSignal();
export async function updateData(path) {
    await invoke("log", {message: "Button clicked"})
    try {
      setSearching(true);
      setDir(path);
      setData();
      await invoke("collect_dir_info", { perfectPath: path });
    } 
    catch (error) {
      console.error("Error occurred:", error);
      await invoke("log", { message: error });

      const msg = "This is the path: " + path;
      await invoke("log", { message: msg });
    }
  }
export async function goBack(dir) {
    let directory = await invoke("get_parent", {path: dir});
    updateData(directory);
}