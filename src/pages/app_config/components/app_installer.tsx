import { Button, List, Message, Tag } from "@arco-design/web-react";
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

interface AppInstallerProps {
  appList: string[];
}

const AppInstaller: React.FC<AppInstallerProps> = ({ appList }) => {
  const [loading, setLoading] = useState(false);
  const [appInstalling, setAppInstalling] = useState(false);

  const handleAppInstall = async (path: string) => {
    setAppInstalling(true);
    try {
      await invoke('install_app', { params: path });
      Message.success('App installation success');
    } catch (error: any) {
      Message.error(error.toString());
    } finally {
      setAppInstalling(false);
    }
  };

  return (<><List
    dataSource={appList}
    header='Downloaded App List'
    render={(item, index) => {
      return (
        <List.Item key={index}>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <div style={{ display: 'inline-block' }}>{item}
            </div>
            <Button type="primary" onClick={() => handleAppInstall(item)} loading={appInstalling}>
              Install
            </Button>
          </div>
        </List.Item>
      )
    }}
  />
  </>)
};

export default AppInstaller;
