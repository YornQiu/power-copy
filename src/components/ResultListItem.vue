<script setup lang="ts">
import type { Record } from '@/types';

defineProps<{
  data: Record;
  index: number;
  selected?: boolean;
}>();
</script>

<template>
  <li class="result-list-item" :class="{ 'is-selected': selected }">
    <span class="result-list-item__index">{{ index + 1 }}</span>
    <div class="result-list-item__content">{{ data.content }}</div>
    <span class="result-list-item__datetime">{{ data.create_at }}</span>
    <span class="result-list-item__action"></span>
  </li>
</template>

<style>
.result-list-item {
  line-height: 1.5;
  padding: 8px;
  border-radius: 8px;
  border: 1px solid rgba(0, 0, 0, 0.1);
  background: rgba(255, 255, 255, 0.2);
  display: flex;
}
.result-list-item__index {
  width: 24px;
  height: 24px;
  color: #f5fcfc;
  font-size: 14px;
  line-height: 24px;
  text-align: center;
  border-radius: 4px;
  background-color: rgba(77, 157, 255, 1);
}
.result-list-item__content {
  flex: 1;
  padding: 0 8px;
}
.result-list-item__datetime {
  color: #909399;
  font-size: 14px;
}

.result-list-item.is-selected {
  color: #f5fcfc;
  background-color: rgba(77, 157, 255, 0.3);
  border: 1px solid rgba(77, 157, 255, 1);
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.122), 0 6px 16px rgba(0, 0, 0, 0.078);
}
.result-list-item.is-selected .result-list-item__datetime {
  color: #e9e9e9;
}

.result-list-item + .result-list-item {
  margin-top: 8px;
}
</style>
