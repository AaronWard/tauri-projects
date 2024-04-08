<template>
    <div>
      <div v-if="!isSupported">
        Your browser does not support the SpeechRecognition API,
        <a href="https://caniuse.com/mdn-api_speechrecognition" target="_blank">more details</a>.
      </div>
      <div v-else>
        <div>
          <label>
            <input v-model="lang" value="en-US" type="radio">
            English (US)
          </label>
          <label>
            <input v-model="lang" value="fr" type="radio">
            French
          </label>
          <label>
            <input v-model="lang" value="es" type="radio">
            Spanish
          </label>
        </div>
        <button v-if="!isListening" @click="startRecognition">
          Press and talk
        </button>
        <button v-else @click="stopRecognition">
          Stop
        </button>
        <div v-if="isListening">
          <p v-if="selectedLanguage === 'en-US'">
            <b>Please say a color:</b> {{ sampled.join(', ') }}
          </p>
          <p v-else-if="selectedLanguage === 'es'">Speak some Spanish!</p>
          <p v-else-if="selectedLanguage === 'fr'">Speak some French!</p>
          <div :style="{ backgroundColor: color }">{{ result }}</div>
        </div>
      </div>
    </div>
  </template>
  
  <script setup>
  import { ref, watch } from 'vue';
  import { useSpeechRecognition } from '@vueuse/core';


  navigator.mediaDevices.getUserMedia({ audio: true, video: false })
  .then(stream => {
    console.log('Microphone access granted');
    // Proceed with stream processing
  })
  .catch(error => {
    console.error('Microphone access denied', error);
  });
  
  const lang = ref('en-US');
  const selectedLanguage = ref(lang.value);
  const colors = [
    'aqua', 'azure', 'beige', 'bisque', 'black', 'blue', 'brown',
    'chocolate', 'coral', 'crimson', 'cyan', 'fuchsia', 'ghostwhite',
    'gold', 'goldenrod', 'gray', 'green', 'indigo', 'ivory', 'khaki',
    'lavender', 'lime', 'linen', 'magenta', 'maroon', 'moccasin', 'navy',
    'olive', 'orange', 'orchid', 'peru', 'pink', 'plum', 'purple', 'red',
    'salmon', 'sienna', 'silver', 'snow', 'tan', 'teal', 'thistle', 'tomato',
    'turquoise', 'violet', 'white', 'yellow', 'transparent'
  ];
  const color = ref('transparent');
  const sampled = ref([]);
  
  function sampleColors() {
    const shuffled = colors.sort(() => 0.5 - Math.random());
    sampled.value = shuffled.slice(0, 5);
    console.log("Sampled colors: ", sampled.value);
  }
  
  const { isSupported, isListening, result, start, stop } = useSpeechRecognition({
    lang,
    continuous: true,
    interimResults: true,
  });
  
  watch(result, newValue => {
    const spokenWords = newValue.value.toLowerCase().split(' ');
    const recognizedColor = colors.find(color => spokenWords.includes(color));
    if (recognizedColor) {
      color.value = recognizedColor;
      console.log("Recognized color: ", color.value);
    }
  });
  
  function startRecognition() {
    console.log("Starting speech recognition...");
    color.value = 'transparent';
    result.value = '';
    sampleColors();
    start();
  }
  
  function stopRecognition() {
    console.log("Stopping speech recognition.");
    stop();
  }
  
  watch(lang, newLang => {
    if (!isListening.value) {
      selectedLanguage.value = newLang;
      console.log("Language changed to: ", selectedLanguage.value);
    }
  });
  
  watch(isListening, listening => {
    if (!listening) {
      selectedLanguage.value = lang.value;
      console.log("Speech recognition stopped, language reset to: ", selectedLanguage.value);
    }
  });
  </script>