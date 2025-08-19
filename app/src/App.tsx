import { Routes, Route, Navigate } from 'react-router-dom';
import Dashboard from './pages/Dashboard';
import Duplicates from './pages/Duplicates';
import ColdData from './pages/ColdData';
import Plan from './pages/Plan';
import Reports from './pages/Reports';
import Nav from './components/Nav';
import { useEffect } from 'react';

const App = () => {
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.ctrlKey && e.shiftKey && e.code === 'KeyS') {
        const event = new CustomEvent('toggle-scan');
        window.dispatchEvent(event);
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, []);

  return (
    <div className="flex">
      <Nav />
      <div className="flex-1 p-4">
        <Routes>
          <Route path="/" element={<Navigate to="/dashboard" />} />
          <Route path="/dashboard" element={<Dashboard />} />
          <Route path="/duplicates" element={<Duplicates />} />
          <Route path="/cold-data" element={<ColdData />} />
          <Route path="/plan" element={<Plan />} />
          <Route path="/reports" element={<Reports />} />
        </Routes>
      </div>
    </div>
  );
};

export default App;
