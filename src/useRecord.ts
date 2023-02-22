/*
 * @Autoor: Yorn Qiu
 * @Date: 2023-02-22 16:38:22
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-22 16:47:07
 * @FilePath: /power-copy/src/useRecord.ts
 * @Description:
 */
import type { Record } from '@/types';

import { getRecords } from '@/core/commands/records';

export function useRecord() {
  const searchKey = ref('');
  const origRecords = ref<Record[]>([]);
  const index = ref(-1);

  const records = computed(() =>
    searchKey.value ? origRecords.value.filter((record) => record.content.includes(searchKey.value)) : origRecords.value
  );

  const increaseIndex = () => {
    if (index.value < records.value.length - 1) index.value++;
  };

  const decreaseIndex = () => {
    if (index.value > 0) index.value--;
  };

  getRecords().then((res) => (origRecords.value = res));

  return { records, index, searchKey, increaseIndex, decreaseIndex };
}
