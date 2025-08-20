import { useEffect, useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { appDataDir, join } from '@tauri-apps/api/path';
import { readTextFile, writeFile, createDir } from '@tauri-apps/plugin-fs';
import { scanStart, scanStatus, scanCancel, scanResults, FileInfo, ScanStatus } from '../ipc/commands';
import Progress from '../components/Progress';
import { Treemap } from 'recharts';

function summarize(files: FileInfo[]): { name: string; size: number }[] {
  const map = new Map<string, number>();
  files.forEach(f => {
    const parts = f.path.replace(/\\/g, '/').split('/');
    parts.pop();
    const folder = parts.join('/');
    map.set(folder, (map.get(folder) || 0) + f.size);
  });
  return Array.from(map.entries())
    .map(([name, size]) => ({ name, size }))
    .sort((a, b) => b.size - a.size);
}

const Dashboard = () => {
  const [roots, setRoots] = useState<string[]>([]);
  const [status, setStatus] = useState<ScanStatus>({ running: false, scanned_bytes: 0, scanned_files: 0, current_path: null });
  const [files, setFiles] = useState<FileInfo[]>([]);

  useEffect(() => {
    loadRoots();
    const interval = setInterval(async () => {
      const st = await scanStatus();
      setStatus(st);
      if (!st.running) {
        const res = await scanResults();
        setFiles(res);
      }
    }, 1000);
    window.addEventListener('toggle-scan', toggleScan);
    return () => {
      clearInterval(interval);
      window.removeEventListener('toggle-scan', toggleScan);
    };
  }, [roots]);

  const settingsFile = async () => {
    const dir = await appDataDir();
    await createDir(dir, { recursive: true });
    return await join(dir, 'settings.json');
  };

  const loadRoots = async () => {
    try {
      const file = await settingsFile();
      const txt = await readTextFile(file);
      const data = JSON.parse(txt);
      setRoots(data.roots || []);
    } catch {}
  };

  const saveRoots = async (r: string[]) => {
    try {
      const file = await settingsFile();
      await writeFile({ path: file, contents: JSON.stringify({ roots: r }) });
    } catch {}
  };

  const selectRoots = async () => {
    const selected = await open({ directory: true, multiple: true });
    if (Array.isArray(selected)) {
      setRoots(selected as string[]);
      saveRoots(selected as string[]);
    }
  };

  const toggleScan = async () => {
    if (status.running) {
      await scanCancel();
    } else {
      await scanStart(roots);
    }
  };

  const summary = summarize(files).slice(0, 20);

  return (
    <div>
      <h1 className="text-xl mb-2">Dashboard</h1>
      <button onClick={selectRoots} className="border px-2 py-1 mr-2">Select Roots</button>
      <button onClick={toggleScan} className="border px-2 py-1">{status.running ? 'Cancel' : 'Scan'}</button>
      <ul className="mt-2">
        {roots.map(r => <li key={r}>{r}</li>)}
      </ul>
      <Progress status={status} />
      <div className="mt-4" style={{ height: 200 }}>
        <Treemap width={400} height={200} data={summary} dataKey="size" stroke="#fff" fill="#8884d8" />
      </div>
      <h2 className="text-lg mt-4">Top Folders</h2>
      <table className="w-full text-left">
        <thead><tr><th>Folder</th><th>Size</th></tr></thead>
        <tbody>
          {summary.map(s => (
            <tr key={s.name}><td>{s.name}</td><td>{(s.size/1024/1024).toFixed(2)} MB</td></tr>
          ))}
        </tbody>
      </table>
    </div>
  );
};

export default Dashboard;
