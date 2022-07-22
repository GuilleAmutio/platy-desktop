import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri'

function App() {
  const [fieldValue, setFieldValue] = useState('');
  const [portValue, setPortValue] = useState('');
  return (
    <div className="App">
      <header className="App-header">
        <input value={fieldValue} onChange={(event) => setFieldValue(event.target.value)}></input>
        <input value={portValue} onChange={(event) => setPortValue(event.target.value)}></input>
        <button type="button" onClick={()=>ListContainers()}> List Containers </button>
        <button type="button" onClick={()=>StartContainer(fieldValue)}> Start Container </button>
        <button type="button" onClick={()=>StopContainer()}> Stop Container </button>
        <button type="button" onClick={()=>RestartContainer()}> Restart Container </button>
        <button type="button" onClick={()=>RemoveContainer()}> Remove Container </button>
        <button type="button" onClick={()=>OpenInBrowser()}> Open in browser </button>
        <button type="button" onClick={()=>OpenTerminal()}> Open terminal </button> 
      </header>
    </div>
  );
}

function ListContainers() {
  invoke('list_containers');
}
function StartContainer(name: string) {
  invoke('start_container', { name });
}

function StopContainer() {
  invoke('stop_container');
}

function RestartContainer() {
  invoke('restart_container');
}

function RemoveContainer() {
  invoke('remove_container');
}

function OpenInBrowser() {
  invoke('open_in_browser');
}

function OpenTerminal() {
  invoke('open_terminal');
}

export default App;
