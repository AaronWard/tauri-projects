<template>
    <div>
        <!-- Display the controls conditionally -->
        <button v-if="isReady && !isRecording" @click="startRecording">Start Recording</button>
        <button v-if="isRecording" @click="stopRecording">Stop Recording</button>
        <audio controls v-if="audioSrc" :src="audioSrc"></audio>
    </div>
</template>
<script>
export default {
  data() {
    return {
      isRecording: false,
      mediaRecorder: null,
      audioChunks: [],
      audioSrc: null,
      silenceTimer: null,
      isReady: false,
      audioContext: null, // To manage a single instance of AudioContext
    };
  },
  mounted() {
    this.requestMicrophoneAccess();
  },
  beforeDestroy() {
    // Clean up to avoid memory leaks
    if (this.audioContext) {
      this.audioContext.close();
    }
    if (this.mediaRecorder && this.mediaRecorder.state !== 'inactive') {
      this.mediaRecorder.stop();
    }
  },
  methods: {
    async requestMicrophoneAccess() {
      try {
        const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
        this.audioContext = new AudioContext();
        this.setupMediaRecorder(stream);
        this.setupSilenceDetection(stream);
        this.isReady = true;
      } catch (error) {
        console.error('Error accessing the microphone', error);
      }
    },
    setupMediaRecorder(stream) {
      this.mediaRecorder = new MediaRecorder(stream);
      this.mediaRecorder.ondataavailable = (e) => this.audioChunks.push(e.data);
      this.mediaRecorder.onstop = () => this.onRecordingStop();
    },
    onRecordingStop() {
      console.log('Recording stopped...');
      const audioBlob = new Blob(this.audioChunks, { type: 'audio/webm' });
      this.audioSrc = URL.createObjectURL(audioBlob);
      this.audioChunks = []; // Reset chunks for the next recording
      // Here, you can add code to send the audioBlob to the backend for transcription
    },
    startRecording() {
      if (!this.isRecording && this.mediaRecorder && this.mediaRecorder.state === 'inactive') {
        this.audioChunks = [];
        this.mediaRecorder.start();
        this.isRecording = true;
        console.log('Recording started...');
      }
    },
    stopRecording() {
      if (this.isRecording && this.mediaRecorder && this.mediaRecorder.state === 'recording') {
        this.mediaRecorder.stop();
        this.isRecording = false;
        clearTimeout(this.silenceTimer);
        console.log('Recording stopped by silence...');
      }
    },
    setupSilenceDetection(stream) {
      const source = this.audioContext.createMediaStreamSource(stream);
      const analyser = this.audioContext.createAnalyser();
      analyser.fftSize = 2048;
      analyser.minDecibels = -90; // Adjust based on testing
      analyser.maxDecibels = -10; // Adjust based on testing
      source.connect(analyser);
      const bufferLength = analyser.fftSize;
      const dataArray = new Uint8Array(bufferLength);

      const checkSilence = () => {
        analyser.getByteTimeDomainData(dataArray);
        let sum = 0;
        for(let i = 0; i < bufferLength; i++) {
          const x = dataArray[i] / 128.0 - 1.0;
          sum += x * x;
        }
        const rms = Math.sqrt(sum / bufferLength);
        const isSilent = rms < 0.01; // This threshold determines silence, adjust as necessary

        if (!isSilent && !this.isRecording) {
          this.startRecording();
        }

        if (isSilent && this.isRecording) {
          clearTimeout(this.silenceTimer);
          this.silenceTimer = setTimeout(() => {
            this.stopRecording();
          }, 2000); // Stop after 2 seconds of silence
        }

        if (this.isReady) {
          requestAnimationFrame(checkSilence);
        }
      };
      checkSilence();
    },
  },
};
</script>
