<script>
// Method 1
import { save, confirm } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
// Method 2
import { writeTextFile, BaseDirectory } from "@tauri-apps/api/fs"

import { ref } from "vue";

export default {
  setup() {
    const message = ref("");

    const saveFileContents = async () => {
      if (message.value == ""){
        throw Error('No value passed')
      }

      const confirmed = await confirm('Are you sure?', 'EXAMPLE TITLE');
      console.log(confirmed);

      if(confirmed) {
        const confirmed2 = await confirm('This action cannot be reverted. Are you sure?', { title: 'Tauri', type: 'warning' });

        if(confirmed2) {
          try {
            console.log(message.value);
            
            // Use if you want to confirm filename with user
            // const savePath = await save();  
            // console.log(savePath)
            // if (!savePath) return;

            // // Method 1
            // await invoke("save_file", { path: savePath, contents: message.value });
            await invoke("save_file", { path: "/Users/award40/Desktop/output.txt", contents: message.value });

            // // Method 2
            // // Another option for saving files but with a hard coded file name and directory:
            // await writeTextFile("test.txt", message.value, {
            //       dir: BaseDirectory.Desktop,
            //   });
          } catch (err) {
            console.error(err);
          }
        }
      }
    };

    // Make sure to return everything that will be used in the template
    return {
      message,
      saveFileContents,
    };
  }
}
</script>


<template>
  <div class="form-container">
    <textarea v-model="message" rows="10" />
    <button @click="saveFileContents">Save File</button>
  </div>
</template>

<style>
.form-container {
  display: flex;
  flex-direction: column;
}
</style>