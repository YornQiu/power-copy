/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-14 09:29:17
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-14 09:31:33
 * @Description: record commands
 * @FilePath: /power-copy/src/commands/records.ts
 */

import { invoke } from '@tauri-apps/api';

export async function getRecords() {
  return invoke('get_records');
}
