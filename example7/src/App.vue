<script setup lang="ts">
import SpeechSynthesis from './components/SpeechSynthesis.vue';
import SpeechRecognition from './components/SpeechRecognition.vue';
import Example from './components/Example.vue';

// const config = useRuntimeConfig()

useHead({
  title: config.public.appTitle,
  meta: [
    { name: 'description', content: 'A chatbox application using OpenAI Completion endpoint to simulate conversation.' }
  ],
})

</script>

<template>
  <!-- <div class="container">
    <SpeechSynthesis />
  </div> -->
    <hr><br>
  <div>
    <Example />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
