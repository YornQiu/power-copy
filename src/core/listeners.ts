/*
 * @Autoor: Yorn Qiu
 * @Date: 2023-02-22 15:16:33
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-22 16:27:56
 * @FilePath: /power-copy/src/listeners.ts
 * @Description:
 */

import { listen } from '@tauri-apps/api/event';

export async function onLanguageChange(consumer: (lang: string) => void) {
  const unListen = await listen<string>('tauri://language-change', async function (event) {
    consumer(event.payload);
  });
  return unListen;
}

export async function onThemeChange(consumer: (theme: string) => void) {
  const unListen = await listen<string>('tauri://theme-change', async function (event) {
    consumer(event.payload);
  });
  return unListen;
}

export async function onRecordChange(consumer: (record: string) => void) {
  const unListen = await listen<string>('tauri://record-change', async function (event) {
    consumer(event.payload);
  });
  return unListen;
}
