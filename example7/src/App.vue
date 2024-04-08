<script setup lang="ts">
import SpeechSynthesis from './components/SpeechSynthesis.vue';
import SpeechRecognition from './components/SpeechRecognition.vue';
import AudioRecorder from './components/AudioRecorder.vue';

</script>

<template>
  <!-- <div class="container">
    <SpeechSynthesis />
  </div> -->
    <hr><br>
  <div>
    <AudioRecorder/>
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
