<template>
  <div>
    <!-- Display the controls conditionally -->
    <button v-if="isReady && !isRecording" @click="startRecording">Start Recording</button>
    <button v-if="isRecording" @click="stopRecording">Stop Recording</button>
    <audio controls v-if="audioSrc" :src="audioSrc"></audio>
  </div>
</template>

<script>
/**
 * This uses the permissions of entitlements.plist for access to the microphone.
 * This script will record audio continuously as long as the noise levels
 * remain above -40 decibels for atleast 3 seconds. The idea here is that you 
 * 
 * TODO: 
 * - impelement transcription
 * - send text to LLM.
 * - impelement auto recording again once a response is returned
 */
import { ref, onMounted, onBeforeUnmount } from 'vue';

export default {
  setup() {
    const isRecording = ref(false);
    const mediaRecorder = ref(null);
    const audioChunks = ref([]);
    const audioSrc = ref(null);
    const isReady = ref(false);
    const audioContext = ref(null);
    const silenceDuration = ref(0);
    const animFrame = ref(null);

    const requestMicrophoneAccess = async () => {
      try {
        const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
        console.log("Microphone access granted");
        audioContext.value = new AudioContext();
        setupMediaRecorder(stream);
        setupSilenceDetection(stream);
        isReady.value = true;
      } catch (error) {
        console.error('Error accessing the microphone', error);
      }
    };

    const setupMediaRecorder = (stream) => {
      mediaRecorder.value = new MediaRecorder(stream);
      mediaRecorder.value.ondataavailable = (e) => audioChunks.value.push(e.data);
      mediaRecorder.value.onstop = onRecordingStop;
    };

    const onRecordingStop = () => {
      console.log('Recording stopped...');
      const audioBlob = new Blob(audioChunks.value, { type: mediaRecorder.value.mimeType });
      audioSrc.value = URL.createObjectURL(audioBlob);
      audioChunks.value = [];
    };

    const startRecording = () => {
      if (!isRecording.value && mediaRecorder.value && mediaRecorder.value.state === 'inactive') {
        audioChunks.value = [];
        mediaRecorder.value.start();
        isRecording.value = true;
        console.log('Recording started...');
      }
    };

    const stopRecording = () => {
      if (isRecording.value && mediaRecorder.value && mediaRecorder.value.state === 'recording') {
        mediaRecorder.value.stop();
        isRecording.value = false;
        silenceDuration.value = 0; // Reset the silence duration when stopping
      }
    };

    const setupSilenceDetection = (stream) => {
      const source = audioContext.value.createMediaStreamSource(stream);
      const analyser = audioContext.value.createAnalyser();
      analyser.fftSize = 2048;
      analyser.minDecibels = -40;
      source.connect(analyser);
      const bufferLength = analyser.fftSize;
      const dataArray = new Uint8Array(bufferLength);

      let lastTime = Date.now();

      const checkSilence = () => {
        const currentTime = Date.now();
        const elapsedTime = currentTime - lastTime;
        lastTime = currentTime;

        analyser.getByteTimeDomainData(dataArray);
        let sum = 0;
        for (let i = 0; i < bufferLength; i++) {
          const x = dataArray[i] / 128.0 - 1.0;
          sum += x * x;
        }
        const rms = Math.sqrt(sum / bufferLength);
        const isSilent = rms < 0.02; // Adjusted threshold for silence detection

        if (!isSilent && isRecording.value) {
          console.log('!isSilent && isRecording.value...');
          silenceDuration.value = 0; // Reset silence duration on sound detection
        } else if (isSilent && isRecording.value) {
          console.log('isSilent && isRecording.value...');
          silenceDuration.value += elapsedTime; // Increment the silence duration based on actual elapsed time
          if (silenceDuration.value >= 3000) { // Stop recording after 3 seconds of silence
            console.log('isilenceDuration.value >= 3000...');
            stopRecording();
          }
        }
        animFrame.value = requestAnimationFrame(checkSilence);
      };
      checkSilence();
    };

    onMounted(async () => {
      await requestMicrophoneAccess();
    });

    onBeforeUnmount(() => {
      if (animFrame.value) {
        cancelAnimationFrame(animFrame.value);
      }
    });

    return { isRecording, audioSrc, isReady, startRecording, stopRecording };
  }
};
</script>
