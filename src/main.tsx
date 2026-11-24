import React from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import DesktopRequired from './DesktopRequired';
import {isNative} from './api';
import './styles.css';

// React is the interface inside Tauri's native WebView, not a standalone website.
createRoot(document.getElementById('root')!).render(<React.StrictMode>{isNative?<App/>:<DesktopRequired/>}</React.StrictMode>);
