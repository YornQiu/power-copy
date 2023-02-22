<script setup lang="ts">
import hotkeys from 'hotkeys-js';
import SearchBar from '@/components/SearchBar.vue';
import ResultList from '@/components/ResultList.vue';
import KeyTips from '@/components/KeyTips.vue';
import { Hotkeys } from '@/core/constants';
import { closeWindow } from '@/core/commands/window';
import { copyRecord } from '@/core/commands/records';
import { useRecord } from '@/useRecord';

const { records, index, searchKey, increaseIndex, decreaseIndex } = useRecord();

async function selectRecord() {
  const record = records.value[index.value];
  await copyRecord(record.id);
  closeWindow();
}

hotkeys(Hotkeys.Up, decreaseIndex);
hotkeys(Hotkeys.Down, increaseIndex);
hotkeys(Hotkeys.Esc, () => {
  closeWindow();
});
hotkeys(Hotkeys.Enter, () => {
  selectRecord();
});
</script>

<template>
  <main class="main">
    <SearchBar @change="(val) => (searchKey = val)" />
    <ResultList :list="records" :index="index" @click-item="selectRecord" @hover-item="(i) => (index = i)" />

    <KeyTips />
  </main>
</template>

<style>
.main {
  display: flex;
  flex-direction: column;
}
</style>
