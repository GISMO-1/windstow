import { NavLink } from 'react-router-dom';

const Nav = () => (
  <nav className="w-48 h-screen border-r p-4">
    <ul className="space-y-2">
      <li><NavLink to="/dashboard" className={({ isActive }) => isActive ? 'font-bold' : ''}>Dashboard</NavLink></li>
      <li><NavLink to="/duplicates" className={({ isActive }) => isActive ? 'font-bold' : ''}>Duplicates</NavLink></li>
      <li><NavLink to="/cold-data" className={({ isActive }) => isActive ? 'font-bold' : ''}>Cold Data</NavLink></li>
      <li><NavLink to="/plan" className={({ isActive }) => isActive ? 'font-bold' : ''}>Plan</NavLink></li>
      <li><NavLink to="/reports" className={({ isActive }) => isActive ? 'font-bold' : ''}>Reports</NavLink></li>
    </ul>
  </nav>
);

export default Nav;
