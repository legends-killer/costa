import React, { useEffect, useState } from 'react';
import { List, Button } from '@arco-design/web-react';
import { Runtime } from '../types';
import { getAvailableSimulatorRuntimeList } from '../../../utils';
import { invoke } from '@tauri-apps/api/tauri';

interface SimInstallerProps {
  installedRuntimes: Runtime[];
}

const SimInstaller: React.FC<SimInstallerProps> = ({ installedRuntimes }) => {
  const [availableRuntimes, setAvailableRuntimes] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);

  const onInstall = async (runtime: string) => {
    setLoading(true);
    await invoke('install_simulator', { path: runtime });
    setLoading(false);
  }

  useEffect(() => {
    // set interval to get available runtimes
    const interval = setInterval(() => {
      getAvailableSimulatorRuntimeList().then((res) => {
        // console.log('res: ', res);
        setAvailableRuntimes(res as string[]);
      });
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const isInstalled = (runtime: string) => {
    return installedRuntimes.some(
      (installed) => runtime.includes(installed.version)
    );
  };

  return (
    <div style={{ height: '100%' }}>
      <List
        header="Available Simulator Runtimes"
        dataSource={availableRuntimes}
        render={(item, index) => (
          <List.Item
            key={index}
            actions={[
              <Button
                type="primary"
                disabled={isInstalled(item)}
                onClick={async () => await onInstall(item)}
                loading={loading}
              >
                {isInstalled(item) ? 'Installed' : 'Install'}
              </Button>
            ]}
          >
            {item}
          </List.Item>
        )}
      />
    </div>
  );
};

export default SimInstaller;
