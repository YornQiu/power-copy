<script setup lang="ts">
defineEmits<{
  (event: 'change', params: string): void;
  (event: 'focus', params: string): void;
}>();

const searchKey = ref('');
</script>

<template>
  <div class="search-bar" data-tauri-drag-region>
    <img src="/search.png" alt="search" class="search-bar__icon" />
    <input
      v-model="searchKey"
      type="text"
      class="search-bar__input"
      autocomplete="off"
      autoCorrect="off"
      autoCapitalize="off"
      spellCheck="false"
      :placeholder="$t('search.placeholder')"
      @change="$emit('change', searchKey)"
      @focus="$emit('focus', searchKey)"
    />
  </div>
</template>

<style>
.search-bar {
  display: flex;
  align-items: center;
  line-height: 32px;
  padding: 8px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}
.search-bar__icon {
  width: 32px;
  height: 32px;
  margin-right: 8px;
}
.search-bar__input {
  color: #3d3d3d;
  font-size: 26px;
  font-weight: 500;
  margin: 0;
  padding: 0;
  border: 0;
  border-radius: 0;
  line-height: 32px;
  background-color: transparent;
  box-shadow: none;
  outline: none;
}
</style>
