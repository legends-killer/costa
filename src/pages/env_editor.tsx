import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Editor } from '@monaco-editor/react';
import { listen } from '@tauri-apps/api/event';
import { Button, Card, Divider, Empty, Input, List, Message, Switch } from '@arco-design/web-react';
import { message } from '@tauri-apps/api/dialog';
import { useLocalStorageState } from 'ahooks';

interface EnvConfig {
  name: string;
  desc: string;
  modifiedAt: Date;
}

const EnvEditor: React.FC = () => {
  const [editorValue, setEditorValue] = useState<EnvConfig>({ name: '', desc: '', modifiedAt: new Date() });
  const [history, setHistory] = useLocalStorageState<EnvConfig[]>('historyEnv', {
    defaultValue: [],
  });
  const [isDarkMode, setIsDarkMode] = useState(false);
  const [favorateEnv, setfavorateEnv] = useLocalStorageState<EnvConfig[]>('favorateEnv', {
    defaultValue: [],
  });
  const [geckoOnline, setGeckoOnline] = useState(true);

  // if the system is in dark mode by prefers-color-scheme: dark
  // listen to the system's dark mode change
  useEffect(() => {
    const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    const handleChange = (e: MediaQueryListEvent) => {
      setIsDarkMode(e.matches);
    };
    mediaQuery.addEventListener('change', handleChange);
    return () => {
      mediaQuery.removeEventListener('change', handleChange);
    };
  }, []);

  useEffect(() => {
    // Get clipboard value on component mount
    getClipboardValue();
  }, []);

  const getClipboardValue = async () => {
    try {
      const clipboardValue = await invoke('get_clipborad_value');
      setEditorValue({ name: clipboardValue as string, desc: 'New name', modifiedAt: new Date() });
    } catch (error) {
      console.error('Error getting clipboard value:', error);
    }
  };

  const handleSetEnv = async (env: 'boe' | 'ppe', isOn: boolean) => {
    try {
      console.log(`set_${env}`, { params: { name: editorValue.name.trim(), envType: env, isOn: isOn, geckoOnline: geckoOnline } });
      await invoke(`set_${env}`, { params: { name: editorValue.name.trim(), envType: env, isOn: isOn, geckoOnline: geckoOnline } });
      // Add current value to history
      setHistory([{ name: editorValue.name.trim(), desc: editorValue.desc, modifiedAt: new Date() }, ...(history || [])]);
      // toast
      Message.success(`Set ${env} to ${isOn ? editorValue.name.trim() : 'Off'} with Gecko ${geckoOnline ? 'Online' : 'Offline'}. Your app may restart.`);
    } catch (error) {
      console.error('Error navigating to name:', error);
    }
  };

  const handleClearHistory = () => {
    setHistory([]);
  };
  // add a listener to the window to respond to the refresh event
  useEffect(() => {
    listen('refresh', (event) => {
      console.log('refresh event', event);
      getClipboardValue();
    });
  }, []);

  const handleDeleteHistory = (item: EnvConfig) => {
    setHistory(history?.filter(historyItem => historyItem.name !== item.name) || []);
  };

  const handleSaveToFavorate = (item: EnvConfig) => {
    let newFavorateEnv = favorateEnv || [];
    // check if the item is already in the favorateEnv
    if (favorateEnv?.some(favoratename => favoratename.name === item.name)) {
      newFavorateEnv = favorateEnv.map((favorate) =>
        favorate === item ? { ...favorate, desc: item.desc } : favorate
      );
    } else {
      newFavorateEnv = [item, ...(favorateEnv || [])];
    };
    // update the favorateEnv
    setfavorateEnv(newFavorateEnv);
  }

  const handleDeleteFavorate = (item: EnvConfig) => {
    setfavorateEnv(favorateEnv?.filter(favoratename => favoratename.name !== item.name) || []);
  };

  return (
    <div style={{ display: 'flex', height: '100vh', padding: 0, margin: 0, overflow: 'hidden' }}>
      <div style={{ flex: 1, marginRight: '16px', display: 'flex', flexDirection: 'column', justifyContent: 'flex-start', height: '100%' }}>
        <Card title='Env Editor' extra={
          <div style={{ display: 'flex', gap: '8px' }}>
            <Switch checked={geckoOnline} onChange={(value) => setGeckoOnline(value)} checkedText='Gecko Online' uncheckedText='Gecko Offline' />
            <Button type='primary' onClick={async () => await handleSetEnv('boe', true)}>Set as Boe</Button>
            <Button type='primary' onClick={async () => await handleSetEnv('ppe', true)}>Set as PPE</Button>
            <Button type='primary' onClick={async () => await handleSetEnv('boe', false)}>Set as Boe Off</Button>
            <Button type='primary' onClick={async () => await handleSetEnv('ppe', false)}>Set as PPE Off</Button>
          </div>
        }
        >
          <Editor
            width="70vw"
            height="50vh"
            defaultLanguage="plaintext"
            theme={isDarkMode ? 'vs-dark' : 'light'}
            value={editorValue.name}
            onChange={(value) => setEditorValue({ name: value || '', desc: editorValue.desc, modifiedAt: new Date() })}
          />
        </Card>
        <div style={{ height: '50vh' }}>
          <h3>Favorate Env</h3>
          <div style={{ display: 'flex', overflowX: 'scroll', whiteSpace: 'nowrap', padding: '4px 0', width: '65vw' }}>
            {favorateEnv?.map((item, index) => (
              <Card
                size='small'
                key={index}
                style={{
                  cursor: 'pointer',
                  marginRight: '16px',
                  padding: '8px',
                  border: '1px solid #ccc',
                  borderRadius: '4px',
                  minWidth: '200px',
                  display: 'inline-block'
                }}
                extra={
                  <Input
                    type="text"
                    value={item.desc}
                    onChange={(e) => {
                      const updatedFavorate = favorateEnv?.map((favorate) =>
                        favorate === item ? { ...favorate, desc: e } : favorate
                      );
                      setfavorateEnv(updatedFavorate);
                    }}
                    style={{ fontWeight: 'bold', border: 'none', background: 'transparent', padding: '3' }}
                  />
                }
              >
                <p style={{ margin: '4px 0 0', fontSize: '0.9em', color: '#666', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                  {item.name.substring(0, 50)}...
                </p>
                <div style={{ marginTop: '8px', display: 'flex', justifyContent: 'space-between' }}>
                  <Button onClick={(e) => {
                    handleDeleteFavorate(item)
                  }} >Delete</Button>
                  <Button type='primary' onClick={(e) => {
                    setEditorValue(item);
                  }}>Apply</Button>
                </div>
              </Card>
            ))}
            {
              favorateEnv?.length === 0 &&
              <Empty />
            }
          </div>
        </div>

      </div>
      <div style={{ width: '35%', height: '100vh', overflowY: 'scroll' }}>

        <h3>History</h3>
        {history?.map((item, index) => (
          <Card
            key={index}
            title={
              <Input
                type="text"
                value={item.desc}
                onChange={(e) => {
                  // update history item by index
                  const updatedHistory = history?.map((historyItem, idx) =>
                    idx === index ? { ...historyItem, desc: e } : historyItem
                  );
                  setHistory(updatedHistory);
                }}
                style={{ fontWeight: 'bold', border: 'none', background: 'transparent' }}
              />
            }
            extra={
              <Button status='danger' onClick={() => handleDeleteHistory(item)}>Delete</Button>
            }
          >
            <p style={{ margin: '4px 0 0', fontSize: '0.9em', color: '#666' }}>
              {item.name}
              <div style={{ marginTop: '8px', display: 'flex', justifyContent: 'space-between' }}>
                <Button type='primary' onClick={() => setEditorValue(item)}>Apply</Button>
                <Button onClick={() => handleSaveToFavorate(item)}>Add to Favorites</Button>
              </div>
            </p>
          </Card>
        ))}
      </div>
    </div>
  );
};

export default EnvEditor;
