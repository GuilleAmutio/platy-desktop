import React from 'react';
import logo from './logo.svg';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri'

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <button type="button" onClick={()=>StartContainer()}> Start Container </button>
        <button type="button" onClick={()=>StopContainer()}> Stop Container </button>
        <button type="button" onClick={()=>RestartContainer()}> Restart Container </button>
        <button type="button" onClick={()=>RemoveContainer()}> Remove Container </button>
        <button type="button" onClick={()=>OpenInBrowser()}> Open in browser </button>
        <button type="button" onClick={()=>OpenTerminal()}> Open terminal </button> 
      </header>
    </div>
  );
}

function StartContainer() {
  invoke('start_container');
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
