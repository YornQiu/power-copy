<script setup lang="ts">
import ResultListEmpty from './ResultListEmpty.vue';
import ResultListItem from './ResultListItem.vue';

defineProps<{
  list: string[];
  index: number;
}>();

defineEmits<{
  (event: 'clickItem', params: number): void;
  (event: 'hoverItem', params: number): void;
}>();
</script>

<template>
  <div class="result-list">
    <ul v-if="list.length">
      <ResultListItem
        v-for="(e, i) in list"
        :key="i"
        :data="e"
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
}
</style>
