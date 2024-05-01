<template>
  <div id="container">
    <div v-if="hasError">Something went wrong!</div>
  </div>
</template>

<script>
import { loadConnectAndInitialize } from "@stripe/connect-js";

export default {
  data() {
    return { hasError: false };
  },
  async mounted() {
    const fetchClientSecret = async () => {
      try {
        // Use HTTP for local development
        const response = await fetch('http://localhost:4242/account_session', { method: "POST" });
        const data = await response.json();
        if (!response.ok) {
          throw new Error(data.error || 'Unknown error');
        }
        return data.client_secret;
      } catch (error) {
        console.error('An error occurred: ', error.message);
        this.hasError = true;
      }
    }

    try {
      const instance = await loadConnectAndInitialize({
        publishableKey: "pk_test_51PBZUP2NgfRpJtnpNO0gYfe5rm3bYTp4bgMqtkfSVVjuvXb0PcgODtZkA6ZIfpsjHkLqYKnYITahSqoYDUiNPyad00RQHYQCJV",
        fetchClientSecret: fetchClientSecret,
        appearance: {
          overlays: 'dialog',
          variables: {
            colorPrimary: '#625afa',
          },
        },
      });

      const container = document.getElementById("container");
      const paymentsComponent = instance.create("payments");
      container.appendChild(paymentsComponent);
    } catch (error) {
      console.error('Failed to load Stripe Connect:', error);
      this.hasError = true;
    }
  }
};
</script>
