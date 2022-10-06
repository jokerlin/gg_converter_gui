import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { InfoCircleOutlined } from '@ant-design/icons';
import { Button, Input, Tooltip, } from 'antd';
import "./App.css";
import 'antd/dist/antd.css'

function App() {
  const [info, setInfo] = useState<String>("");
  const [directory, setDirectory] = useState("");

  async function convert() {
    const result = await invoke("convert", { name: directory }) as String
    setInfo(result);
  }

  const handleInputChange = (val: React.ChangeEvent<HTMLInputElement>) => {
    setDirectory(val.target.value);
  }

  return (
    <div className="container">
      <h1>Welcome to GG Converter</h1>

      <p>Convert GGPoker's hand history into Hand2Note supportable format.</p>

      <div className="row">

        <a href="https://www.natural8.com/" target="_blank">
          <img src="/n8.png" className="logo" alt="Natural8 Logo" />
        </a>
        <a href="https://hand2note.com/" target="_blank">
          <img src="/h2n.png" className="logo" alt="Hand2Note Logo" />
        </a>
      </div>

      <div className="row">
        <Input
          className="customInput"
          placeholder="Your Local Directory with GGPoker's Hands"
          value={directory}
          onChange={handleInputChange}
          suffix={
            <Tooltip title="Type Full Path">
              <InfoCircleOutlined style={{ color: 'rgba(0,0,0,.45)' }} />
            </Tooltip>
          }
        />
      </div>

      <div className="row">
          <Button type="primary" onClick={() => convert()}>
            Convert
          </Button>
      </div>

      <div style={{marginTop : '3em'}}>
        {info === 'success' ? <div className="resultSuccess">Hands Converted Successfully!</div> : <div className="resultError">{info}</div>}
      </div>
    </div>
  );
}

export default App;
