<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import { onMounted } from "vue";
import initSync, { add as wasmAdd, main_js, __wbg_get_imports } from "../wasm/out/rust_wasm.js"; // adjust path accordingly

try {
  await initSync(); // This initializes the wasm module and loads the wasm file
  } catch (error) {
    console.error("Error loading main html:", error);
}

async function loadWasm() {
  try {
    const response = await fetch("rust_wasm_bg.wasm");
    if (!response.ok) {
      throw new Error(`Failed to fetch WASM: ${response.status}`);
    }
    const buffer = await response.arrayBuffer();
    const module = await WebAssembly.compile(buffer);
    const instance = await WebAssembly.instantiate(module, __wbg_get_imports());

    console.log("WASM loaded:", instance);
    console.log("WASM Add function result:", instance.exports.add(22, 33));
  } catch (error) {
    console.error("Error loading WASM:", error);
  }
}

loadWasm().catch(console.error);
</script>

<template>
  <!-- <Suspense> -->
    <div class="container"></div>
  <!-- </Suspense> -->
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
