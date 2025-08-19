import { ScanStatus } from '../ipc/commands';

const Progress = ({ status }: { status: ScanStatus }) => {
  if (!status.running && status.scanned_files === 0) return null;
  const percent = status.scanned_bytes / (status.scanned_bytes + 1);
  return (
    <div className="mt-4">
      <div className="h-4 bg-gray-200">
        <div className="h-full bg-green-500" style={{ width: `${percent * 100}%` }}></div>
      </div>
      <p className="text-sm mt-1">{status.scanned_files} files - {status.current_path || ''}</p>
    </div>
  );
};

export default Progress;
