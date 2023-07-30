import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';

import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [nums, setNums] = useState([]);
  const [paths, setPaths] = useState([]);

  async function openFileDialog() {
    const selected = await open({
      multiple: true,
      directory: true,
      filters: [{
        name: 'Directory',
        extensions: ['dir']
      }]
    });
    if (Array.isArray(selected)) {
      const dir = selected[0]
      console.log(dir)
      await get_files(dir)
      // user selected multiple files
    } else if (selected === null) {
      // user cancelled the selection
    } else {
      // user selected a single file
    }
  }

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  async function grab_nums() {
    setNums(await invoke("list_stuff"));
  }

  async function get_files(dir: string) {
    setPaths(await invoke("get_files", { dir: dir }));
  }

  async function play_song(dir: string) {
    invoke("play_song", { songname: dir });
  }


  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <button onClick={openFileDialog}>Select Folder</button>
      </div>
      <p>{greetMsg}</p>


      <ul>
        {
          nums.map((num) => <li>{num}</li>)
        }
      </ul>

      {
        paths.map((path) => <button onClick={() => {
          play_song(path);
        }}>{path}</button>)
      }

    </div>
  );
}

export default App;
