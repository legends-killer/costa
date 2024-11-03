import React from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import Home from './App';
import SchemaEditor from './pages/schema_editor';
import EnvEditor from './pages/env_editor';
import './index.less';
import AppConfig from './pages/app_config';

const Router: React.FC = () => {
  const darkThemeMq = window.matchMedia("(prefers-color-scheme: dark)");

  darkThemeMq.addListener(e => {
    if (e.matches) {
      document.body.setAttribute('arco-theme', 'dark');
    } else {
      document.body.removeAttribute('arco-theme');
    }
  });
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/home" element={<Home />} />
        <Route path="/schema_editor" element={<SchemaEditor />} />
        <Route path="/env_edit" element={<EnvEditor />} />
        <Route path="/" element={<Navigate replace to="/home" />} />
        <Route path="*" element={<Navigate replace to="/home" />} />
        <Route path="/app_config" element={<AppConfig />} />
      </Routes>
    </BrowserRouter>
  );
};

export default Router;

