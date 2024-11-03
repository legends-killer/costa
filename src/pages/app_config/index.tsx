import React, { useEffect, useState } from 'react';
import { Button, Typography, Space, Message, List, Modal } from '@arco-design/web-react';
import { invoke } from '@tauri-apps/api/tauri';
import { getInstalledSimulatorRuntimeList } from '../../utils';
import { RuntimeMap } from './types';
import SimInstaller from './components/sim_installer';
import AppInstaller from './components/app_installer';

const { Title } = Typography;

const AppConfig: React.FC = () => {
  const [simInstalling, setSimInstalling] = useState(false);
  const [runtimeList, setRuntimeList] = useState<RuntimeMap>({});

  useEffect(() => {
    // set interval to get runtime list
    const interval = setInterval(() => {
      getInstalledSimulatorRuntimeList().then((res) => {
        // console.log("res: ", res);
        setRuntimeList(res as RuntimeMap);
      });
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  const handleXcodeInstall = async () => {
    Modal.info({
      title: 'Copy This Command to Terminal to Install Xcode Toolchains',
      content: 'xcode-select --install',
    });
  };

  const handleDownloadSimulator = async () => {
    try {
      await invoke('download_simulator');
      Message.success('Simulator download started');
    } catch (error: any) {
      Message.error(error.toString());
    }
  };

  const handleSimulatorInstall = async () => {
    Modal.info({
      title: 'Install Simulator',
      footer: null,
      style: {
        width: '80%',
        height: '80%',
      },
      content: (
        <>
          <SimInstaller
            installedRuntimes={Object.values(runtimeList)}
          />
        </>
      ),
    })
  };

  const [appList, setAppList] = useState<string[]>([]);

  const handleAppDownload = async (app: string) => {
    try {
      await invoke('download_app', { params: app });
      Message.success('App download started');
    } catch (error: any) {
      Message.error(error.toString());
    }
  }

  const handleSimulatorRuntimeDelete = async (id: string) => {
    await invoke('delete_simulator_runtime', { id });
    Message.success('Simulator runtime deleted');
  }

  useEffect(() => {
    // get app list as interval
    const interval = setInterval(() => {
      invoke('get_app_pkg_list').then((res) => {
        setAppList(res as string[]);
      });
    }, 1000);
    return () => clearInterval(interval);
  }, []);

  return (
    <Space direction="vertical" size="large" style={{ width: '100%' }}>
      <div>
        <Title heading={3}>Install Xcode Toolchains</Title>
        <Button type="primary" onClick={handleXcodeInstall}>
          Install Xcode Toolchains
        </Button>
      </div>

      <div>
        <Title heading={3}>Install Simulator</Title>

        <div style={{ display: 'flex', gap: 10 }}>
          <Button type="primary" onClick={handleDownloadSimulator}>
            Download Simulator
          </Button>

          <Button type="primary" onClick={handleSimulatorInstall} loading={simInstalling}>
            Install Simulator
          </Button>
        </div>
        <div style={{ marginTop: 10 }}>
          <List
            header='Installed Simulator Runtime List'
            dataSource={Object.values(runtimeList)}
            render={(item, index) => {
              return (
                <List.Item key={index}>
                  <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                    <div style={{ display: 'inline-block' }}>{`${item.version} - ${item.build} (${item.state})`}</div>
                    <Button status='danger' type="primary" onClick={async () => {
                      await handleSimulatorRuntimeDelete(item.identifier);
                    }}>
                      Delete
                    </Button>
                  </div>
                </List.Item>
              )
            }}
          />
        </div>
      </div>


      <div>
        <Title heading={3}>Install App to Simulator</Title>
        <div style={{ display: 'flex', gap: 10 }}>
          <Button type="primary" onClick={() => handleAppDownload('app1')}>
            Download App1 Latest
          </Button>
          <Button type="primary" onClick={() => handleAppDownload('app2')} status='warning'>
            Download App2 Latest
          </Button>
        </div>
        <div style={{ marginTop: 10 }}>
          <AppInstaller appList={appList} />
        </div>
      </div>
    </Space>
  );
};

export default AppConfig;
