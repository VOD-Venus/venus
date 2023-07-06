import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'antd';
import { useCallback } from 'react';
import useStore from 'store';
import { CoreConfig, RConfig } from 'store/config-store';

const useBackend = () => {
  const { updateRConfig, updateCoreConfig, updateConfig } = useStore();

  /**
   * Get newest config from backend
   */
  const reloadConfig = useCallback(async (type: 'core' | 'rua') => {
    const map = {
      rua: async () => {
        const rua = await invoke<RConfig>('get_rua_config');
        updateRConfig(rua);
      },
      core: async () => {
        const core = await invoke<CoreConfig>('get_core_config');
        updateCoreConfig(core);
      },
    };
    try {
      map[type]();
    } catch (err) {
      console.error(err);
      message.error('Get rua config failed', err.toString());
    }
  }, []);

  /**
   * Send current config in global state to backend
   */
  const writeConfig = useCallback((type: 'core' | 'rua') => {
    updateConfig((config) => {
      (async () => {
        const map = {
          core: 'coreConfig',
          rua: 'ruaConfig',
        };
        try {
          await invoke('update_config', { [`${map[type]}`]: config[type] });
          message.success('Update config success');
        } catch (err) {
          message.error(err);
        }
      })();
    });
  }, []);

  return {
    reloadConfig,
    writeConfig,
  };
};

export default useBackend;
