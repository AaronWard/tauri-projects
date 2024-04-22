<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import { onMounted } from "vue";
import init, { add as wasmAdd, main_js } from "../wasm/out/rust_wasm.js"; // adjust path accordingly

async function loadWasm() {
  try {
    await init(); // This initializes the wasm module and loads the wasm file
    // main_js();
  } catch (error) {
    console.error("Error loading main html:", error);
  }

  try {
    const response = await fetch("rust_wasm_bg.wasm");
    if (!response.ok) {
      throw new Error(`Failed to fetch WASM: ${response.status}`);
    }
    const buffer = await response.arrayBuffer();
    const module = await WebAssembly.compile(buffer);
    const imports = {
      env: {
        // Create a new WebAssembly Memory object
        memory: new WebAssembly.Memory({ initial: 256 }),
      },
    };
    const instance = await WebAssembly.instantiate(module);
    console.log("WASM loaded:", instance);
    console.log("WASM Add function result:", instance.exports.add(22, 33));
  } catch (error) {
    console.error("Error loading WASM:", error);
  }
}

onMounted(() => {
  loadWasm().catch(console.error);
});
</script>

<template>
  <Suspense>
    <div class="container"></div>
  </Suspense>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
