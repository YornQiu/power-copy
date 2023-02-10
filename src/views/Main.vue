<script setup lang="ts">
import SearchBar from '@/components/SearchBar.vue';
import ResultList from '@/components/ResultList.vue';
import KeyTips from '@/components/KeyTips.vue';

const list = ref([]);
const index = ref(-1);

function onSearchChange(val: string) {
  //
  console.log(val);
}
</script>

<template>
  <main class="main">
    <SearchBar @change="onSearchChange" />
    <ResultList :list="list" :index="index" />

    <KeyTips />
  </main>
</template>

<style>
.main {
  display: flex;
  flex-direction: column;
}
</style>
