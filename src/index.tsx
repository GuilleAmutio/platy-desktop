import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import ConfigurePlatyStore from './store';
import containers from './store/containers';
import MockClient from './client/mockClient';
import './index.css';

const store = ConfigurePlatyStore()
  .withFeature(containers)
  .withDependiencies({
    client: MockClient
  })
  .build();

const root = ReactDOM.createRoot(
  document.getElementById('root') as HTMLElement
);

root.render(
  <React.StrictMode>
    <App store={store}/>
  </React.StrictMode>
);
