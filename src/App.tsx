import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [imageMsg, setImageMsg] = createSignal("");
  const [text, setText] = createSignal("");

  async function getQrCode() {
    const imageResponse: any = await invoke("generate_qr_image", { text: text() });
    const base64Data = btoa(String.fromCharCode(...new Uint8Array(imageResponse)));
    setImageMsg(`data:image/png;base64,${base64Data}`);
  }

  return (
    <div class="container">
      <h2>QRCode生成</h2>
      <div class="row">
        <div>
          <input
            id="url-input"
            onChange={(e) => setText(e.currentTarget.value)}
            placeholder="Enter a URL..."
          />
          <button type="button" onClick={() => getQrCode()}>
            生成
          </button>
        </div>
      </div>
      <div class="row">
        <img src={imageMsg()} alt="" style="width: 200px;" id="image"></img>
      </div>
    </div>
  );
}

export default App;
