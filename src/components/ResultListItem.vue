<script setup lang="ts">
defineProps<{
  data: string;
  selected?: boolean;
}>();
</script>

<template>
  <li class="result-list-item" :class="{ 'is-selected': selected }">{{ data }}</li>
</template>

<style>
.result-list-item {
  line-height: 32px;
  padding: 8px;
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  background: rgba(255, 255, 255, 0.4);
}

.result-list-item.is-selected {
  color: #f5fcfc;
  background-color: #4d9dff;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.122), 0 6px 16px rgba(0, 0, 0, 0.078);
}

.result-list-item + .result-list-item {
  margin-top: 8px;
}
</style>
