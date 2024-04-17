<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const pluginCode = ref("");

async function loadPlugin() {
  try {
    const pluginCode = await invoke('load_plugin', { name: 'example-plugin.js' });
    eval(pluginCode);
  } catch (error) {
    console.error('Error loading plugin:', error);
  }
}
</script>

<template>
  <form class="row" @submit.prevent="loadPlugin">
    <button type="submit">Greet</button>
  </form>

  <p>{{ pluginCode }}</p>
</template>
