import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Editor } from '@monaco-editor/react';
import { listen } from '@tauri-apps/api/event';
import { Button, Card, Divider, Empty, Input, List } from '@arco-design/web-react';
import { useLocalStorageState } from 'ahooks';

interface SchemaRecord {
  schema: string;
  name: string;
  modifiedAt: Date;
}

const SchemaRouter: React.FC = () => {
  const [editorValue, setEditorValue] = useState<SchemaRecord>({ schema: '', name: '', modifiedAt: new Date() });
  const [history, setHistory] = useLocalStorageState<SchemaRecord[]>('historySchemas', {
    defaultValue: [],
  });
  const [isDarkMode, setIsDarkMode] = useState(false);
  const [favorateSchemas, setfavorateSchemas] = useLocalStorageState<SchemaRecord[]>('favorateSchemas', {
    defaultValue: [],
  });

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
      setEditorValue({ schema: clipboardValue as string, name: 'New Schema', modifiedAt: new Date() });
    } catch (error) {
      console.error('Error getting clipboard value:', error);
    }
  };

  const handleGotoSchema = async () => {
    try {
      await invoke('goto_schema', { schema: editorValue.schema.trim() });
      // Add current value to history
      setHistory(prevHistory => [{ schema: editorValue.schema, name: editorValue.name, modifiedAt: new Date() }, ...(prevHistory || [])]);
    } catch (error) {
      console.error('Error navigating to schema:', error);
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

  const handleDeleteHistory = (item: SchemaRecord) => {
    setHistory(prevHistory => prevHistory?.filter(historyItem => historyItem.schema !== item.schema) || []);
  };

  const handleSaveToFavorate = (item: SchemaRecord) => {
    // check if the item is already in the favorateSchemas
    if (favorateSchemas?.some(favorateSchema => favorateSchema.schema === item.schema)) {
      // if the item is already in the favorateSchemas, update the name, and move to the top
      setfavorateSchemas(prevfavorateSchemas => [item, ...(prevfavorateSchemas?.filter(favorateSchema => favorateSchema.schema !== item.schema) || [])]);
    } else {
      setfavorateSchemas(prevfavorateSchemas => [item, ...(prevfavorateSchemas || [])]);
    };
  }

  const handleDeleteFavorate = (item: SchemaRecord) => {
    setfavorateSchemas(prevfavorateSchemas => prevfavorateSchemas?.filter(favorateSchema => favorateSchema.schema !== item.schema) || []);
  };

  return (
    <div style={{ display: 'flex', height: '100vh', padding: 0, margin: 0, overflow: 'hidden' }}>
      <div style={{ flex: 1, marginRight: '16px', display: 'flex', flexDirection: 'column', justifyContent: 'space-between', height: '100%' }}>
        <Card title='Schema Editor' extra={<Button type='primary' onClick={handleGotoSchema}>Go to Schema</Button>}>
          <Editor
            width="70vw"
            // height="50vh"
            height="60vh"
            defaultLanguage="plaintext"
            theme={isDarkMode ? 'vs-dark' : 'light'}
            value={editorValue.schema}
            onChange={(value) => setEditorValue({ schema: value || '', name: editorValue.name, modifiedAt: new Date() })}
          />
        </Card>
        <div style={{ height: '30vh' }}>
          <h3>Favorate Schemas</h3>
          <div style={{ display: 'flex', overflowX: 'scroll', whiteSpace: 'nowrap', padding: '4px 0', width: '70vw' }}>
            {favorateSchemas?.map((item, index) => (
              <Card
                size='small'
                key={index}
                style={{
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
                    value={item.name}
                    onChange={(e) => {
                      const updatedFavorate = favorateSchemas?.map((favorate) =>
                        favorate === item ? { ...favorate, name: e } : favorate
                      );
                      setfavorateSchemas(updatedFavorate);
                    }}
                    style={{ fontWeight: 'bold', border: 'none', background: 'transparent', padding: '3' }}
                  />
                }
              >
                <p style={{ margin: '4px 0 0', fontSize: '0.9em', color: '#666', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                  {item.schema.substring(0, 50)}...
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
              favorateSchemas?.length === 0 &&
              <Empty />
            }
          </div>
        </div>
      </div>
      <div style={{ width: '30%', height: '100vh', overflowY: 'scroll' }}>

        <h3>History</h3>
        {/* <Button onClick={handleClearHistory}>Clear History</Button> */}
        {history?.map((item, index) => (
          <Card
            key={index}
            title={
              <Input
                type="text"
                value={item.name}
                onChange={(e) => {
                  // update history item by index
                  const updatedHistory = history?.map((historyItem, idx) =>
                    idx === index ? { ...historyItem, name: e } : historyItem
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
              {item.schema}
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

export default SchemaRouter;
