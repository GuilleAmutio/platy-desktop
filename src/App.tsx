import { Provider } from 'react-redux'
import { Store } from '@reduxjs/toolkit';
import ContainersScreen from 'screens/containers';
import './App.css';

function App({store}: {store: Store}) {
  return (
    <Provider store={store}>
      <ContainersScreen/>
    </Provider>
  );
}

export default App;
