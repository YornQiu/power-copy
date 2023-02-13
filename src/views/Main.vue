<script setup lang="ts">
import hotkeys from 'hotkeys-js';
import SearchBar from '@/components/SearchBar.vue';
import ResultList from '@/components/ResultList.vue';
import KeyTips from '@/components/KeyTips.vue';
import { Hotkeys } from '@/constants';
import { closeWindow } from '@/commands/window';

const state = reactive({
  list: ['qwe3rtyu', 'asdfgh', 'zxcvbnm,'] as string[],
  index: 0,
});

function onSearchChange(val: string) {
  //
  console.log(val);
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
  if (state.index < state.list.length - 1) {
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
    <ResultList :list="state.list" :index="state.index" @click-item="changeIndex" @hover-item="changeIndex" />

    <KeyTips />
  </main>
</template>

<style>
.main {
  display: flex;
  flex-direction: column;
}
</style>
