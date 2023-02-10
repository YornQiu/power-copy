<script setup lang="ts">
defineProps<{
  data: string;
  selected?: boolean;
}>();
</script>

<template>
  <div class="result-list-item" :class="{ 'is-selected': selected }">{{ data }}</div>
</template>

<style>
.result-list-item {
}
</style>
