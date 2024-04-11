<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/api/dialog";
import { readTextFile, BaseDirectory } from "@tauri-apps/api/fs";

const content = ref("");


onMounted(() => {
  appWindow.listen("new-content", () => {
    console.log("new-content event emitted");
    content.value = "";
  });
  appWindow.listen("open-file", async () => {
    try {
      const filePath = await open({
        title: "Select a Text File",
        filters: [{ name: "Text", extensions: ["txt"] }],
      });
      if (!filePath) return;
      // Read the text file in the `$APPCONFIG/app.conf` path
      // const fileContent = await readTextFile('app.conf', { dir: BaseDirectory.AppConfig });
      const fileContent = await readTextFile(filePath, {});

      content.value = fileContent;
    } catch (error) {
      console.error(error);
    }
  });
});

</script>

<template>
  <div class="container">
    <textarea v-model="content" rows="20" class="content-area"/>
  </div>
</template>

<style scoped>
content-area {
  width: 100%;
}
</style>
