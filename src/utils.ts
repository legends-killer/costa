import { invoke } from "@tauri-apps/api/tauri";

export const getInstalledSimulatorRuntimeList = async () => {
  const runtimes = await invoke('get_installed_simulator_runtime_list');
  return runtimes;
};

export const getAvailableSimulatorRuntimeList = async () => {
  const runtimes = await invoke('get_available_simulator_runtime_list');
  return runtimes;
};
