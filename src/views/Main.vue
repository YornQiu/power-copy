<script setup lang="ts">
import type { Record } from '@/types';

import hotkeys from 'hotkeys-js';
import SearchBar from '@/components/SearchBar.vue';
import ResultList from '@/components/ResultList.vue';
import KeyTips from '@/components/KeyTips.vue';
import { Hotkeys } from '@/constants';
import { closeWindow } from '@/commands/window';
import { getRecords } from '@/commands/records';

const state = reactive({
  records: [
    { id: 0, content: 'qwertyui', ctype: '', create_at: 0 },
    { id: 1, content: 'asdfghjkl', ctype: '', create_at: 0 },
    { id: 2, content: 'zxcvbnm', ctype: '', create_at: 0 },
  ] as Record[],
  searchKey: '',
  index: 0,
});

getRecords().then((res) => (state.records = res));

const list = computed(() =>
  state.searchKey ? state.records.filter((record) => record.content.includes(state.searchKey)) : state.records
);

function onSearchChange(val: string) {
  state.searchKey = val;
}

function changeIndex(i: number) {
  state.index = i;
}

function moveIndexUp() {
  if (state.index > 0) {
    state.index -= 1;
  }
}

function moveIndexDown() {
  if (state.index < list.value.length - 1) {
    state.index += 1;
  }
}

hotkeys(Hotkeys.Up, moveIndexUp);
hotkeys(Hotkeys.Down, moveIndexDown);
hotkeys(Hotkeys.Esc, () => {
  closeWindow();
});
</script>

<template>
  <main class="main">
    <SearchBar @change="onSearchChange" />
    <ResultList :list="list" :index="state.index" @click-item="changeIndex" @hover-item="changeIndex" />

    <KeyTips />
  </main>
</template>

<style>
.main {
  display: flex;
  flex-direction: column;
}
</style>
