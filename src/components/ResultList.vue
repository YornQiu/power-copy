<script setup lang="ts">
import type { Record } from '@/types';

import ResultListEmpty from './ResultListEmpty.vue';
import ResultListItem from './ResultListItem.vue';

defineProps<{
  list: Record[];
  index: number;
}>();

defineEmits<{
  (event: 'clickItem', params: number): void;
  (event: 'hoverItem', params: number): void;
  (event: 'removeItem', params: number): void;
}>();
</script>

<template>
  <div class="result-list">
    <ul v-if="list.length" class="result-list__ul">
      <ResultListItem
        v-for="(e, i) in list"
        :key="i"
        :data="e"
        :index="i"
        :selected="i === index"
        @click="$emit('clickItem', i)"
        @mouseenter="$emit('hoverItem', i)"
      ></ResultListItem>
    </ul>

    <ResultListEmpty v-else />
  </div>
</template>

<style>
.result-list {
  flex: 1;
  padding: 8px;
  overflow: auto;
}
.result-list__ul {
  margin: 0;
  padding: 0;
  list-style: none;
}
</style>
