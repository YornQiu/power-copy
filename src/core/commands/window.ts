import { appWindow } from '@tauri-apps/api/window';

export async function closeWindow() {
  await appWindow.hide();
}
