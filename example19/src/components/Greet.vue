<template>
  <div>
    <input v-model="inputValue" placeholder="Enter some text" />
    <button @click="storeValue">Store Value</button>
    <button @click="retrieveValue">Retrieve Value</button>
    <p>Stored Value: {{ retrievedValue }}</p>
  </div>
</template>

<script>
import { Store } from '@tauri-apps/plugin-store';
import { exists, BaseDirectory } from '@tauri-apps/plugin-fs';
// https://v2.tauri.app/references/javascript/fs/#basedirectory - all files stored here
// https://v2.tauri.app/references/javascript/fs/#example-2 0 - opening files.


AppConfig
// Store will be loaded automatically when used in JavaScript binding.
const store = new Store('store.bin');
const key = 'user-input';

export default {
  data() {
    return {
      inputValue: '',
      retrievedValue: null,
    };
  },
  methods: {
    async storeValue() {
      // Set a value.
      await store.set(key, { value: this.inputValue });
      await store.save();

    },
    async retrieveValue() {
      const result = await store.get(key);
      result = result + BaseDirectory.AppDirectory
      console.log(result);
      this.retrievedValue = result ? result.value : 'No value stored';
    }
  }
};
</script>
