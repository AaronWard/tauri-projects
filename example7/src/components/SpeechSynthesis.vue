<template>
    <div>
      <div v-if="!isSupported">
        Your browser does not support the SpeechSynthesis API,
        <a href="https://caniuse.com/mdn-api_speechsynthesis" target="_blank">more details</a>.
      </div>
      <div v-else>
        <div>
          <label for="spokenText">Spoken Text:</label>
          <input id="spokenText" v-model="text" type="text">
        </div>
  
        <div>
          <label for="language">Language:</label>
          <select id="language" v-model="selectedVoice">
            <option disabled value="">Select Language</option>
            <option v-for="voice in voices" :key="voice.name" :value="voice">{{ voice.name }} ({{ voice.lang }})</option>
          </select>
        </div>
  
        <div>
          <label for="pitch">Pitch:</label>
          <input id="pitch" v-model="pitch" type="range" min="0.5" max="2" step="0.1">
        </div>
  
        <div>
          <label for="rate">Rate:</label>
          <input id="rate" v-model="rate" type="range" min="0.5" max="2" step="0.1">
        </div>
  
        <button :disabled="isPlaying" @click="speak">Speak</button>
        <button :disabled="!isPlaying" @click="pause">Pause</button>
        <button :disabled="!isPlaying" @click="stop">Stop</button>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, onMounted, watch } from 'vue';
  
  const text = ref('Hello, everyone! Good morning!');
  const pitch = ref(1);
  const rate = ref(1);
  const selectedVoice = ref(null);
  const voices = ref([]);
  const isPlaying = ref(false);
  const isSupported = ref('speechSynthesis' in window);
  let synth = window.speechSynthesis;
  let utterance;
  
  onMounted(() => {
    voices.value = synth.getVoices();
    if (voices.value.length === 0) {
      synth.addEventListener('voiceschanged', () => {
        voices.value = synth.getVoices();
      });
    }
  });
  
  watch(selectedVoice, (newVoice) => {
    if (newVoice) {
      utterance.voice = newVoice;
    }
  });
  
  function speak() {
    if (utterance && synth.speaking) {
      return;
    }
    utterance = new SpeechSynthesisUtterance(text.value);
    utterance.pitch = pitch.value;
    utterance.rate = rate.value;
    utterance.voice = selectedVoice.value || voices.value.find(v => v.default);
    synth.speak(utterance);
    isPlaying.value = true;
    utterance.onend = () => isPlaying.value = false;
  }
  
  function pause() {
    synth.pause();
    isPlaying.value = false;
  }
  
  function stop() {
    synth.cancel();
    isPlaying.value = false;
  }
  </script>
  
  <style scoped>
  /* Add your styles here */
  </style>
  