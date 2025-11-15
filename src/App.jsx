import { createSignal, Index, Show } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { dir, setDir, searching, setSearching, updateData, data, setData} from "./funcs";
import { SearchingAnimation } from "./Components/search";
import "./App.css";
import { FileView } from "./Components/view";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal(""); 
  const [debug] = createSignal(false);
  const [dots, setDots] = createSignal("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name: name() }));
  }

  listen('finished-searching', (event) => {
    console.log(
      `downloading ${event.payload.obj}`
    );
    setData(event.payload.obj);
    setSearching(false);
  });
//   async function fetchData() {
//  try {
//    const response = await fetch("https://invalid-url.com");
//    const data = await response.json();
//  } catch (error) {
//    console.error("Error occurred:", error);
//  }
// }
// fetchData();


  async function setInitDir(){
    const root = await invoke("find_root")
    setDir(root);
    updateData(dir());
  }
  setInitDir();
  return (
    <main class="container">
      <SearchingAnimation />
      <FileView data={data()} dir={dir()} setDir={setDir()} setData={setData()} updateData={updateData()}></FileView>
      <Show when={debug()}>
        <h2>Debug</h2>
        <p>{data}</p>
      </Show>
    </main>
  );
}

export default App;
