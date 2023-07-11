import { StateCreator } from 'zustand';
import { ConfigSlice } from './config-store';
import { immer } from 'zustand/middleware/immer';
import { LogSlice } from './log-store';

export type MenuType = 'global' | 'node';
export interface UI {
  // content menu on right click
  showMenu: MenuType | null;
  // mouse position when right click
  mousePos: {
    x: number;
    y: number;
  };
  // loadings
  loading: {
    // update all loading
    updateAll: boolean;
    // subs card loading
    subCrad: {
      url: string;
      loading: boolean;
    }[];
  };
  // current selected tabs
  tabs: {
    index: string;
    setting: string;
  };
}
export interface UIAction {
  toggleUI: (callback: (ui: UI) => void) => void;
}

export type UISlice = UI & UIAction;

const createUISlice: StateCreator<
  UISlice & ConfigSlice & LogSlice,
  [],
  [['zustand/immer', never]],
  UISlice
> = immer<UISlice>((set) => ({
  showMenu: null,
  mousePos: {
    x: 0,
    y: 0,
  },
  loading: {
    updateAll: false,
    subCrad: [],
  },
  tabs: {
    index: '1',
    setting: '1',
  },
  toggleUI(callback) {
    set(callback);
  },
}));

export default createUISlice;