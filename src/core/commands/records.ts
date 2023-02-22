/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-14 09:29:17
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-22 16:54:17
 * @Description: record commands
 * @FilePath: /power-copy/src/core/commands/records.ts
 */
import type { Record } from '@/types';

import { invoke } from '@tauri-apps/api';

export async function getRecords(): Promise<Record[]> {
  return invoke('get_records');
}

export async function deleteRecord(ids: number | number[]) {
  if (Array.isArray(ids)) return invoke('delete_records', { ids });
  return invoke('delete_record', { id: ids });
}

export async function clearRecord() {
  return invoke('clear_record');
}

export async function copyRecord(id: number) {
  return invoke('copy_record', { id });
}
