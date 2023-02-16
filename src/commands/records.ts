/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-14 09:29:17
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-16 15:46:53
 * @Description: record commands
 * @FilePath: /power-copy/src/commands/records.ts
 */
import type { Record } from '@/types';

import { invoke } from '@tauri-apps/api';

export async function getRecords(): Promise<Record[]> {
  return invoke('get_records');
}

export async function deleteRecord(id: number | number[]) {
  return invoke('delete_record', { id });
}

export async function clearRecord() {
  return invoke('clear_record');
}
