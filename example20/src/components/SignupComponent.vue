<template>
  <form class="row" @submit.prevent="signUp">
    <div style="display: flex; flex-direction: column;">
      <input id="signup-email" type="email" v-model="email" placeholder="Enter an email address..." />
      <input id="signup-password" type="password" v-model="password" placeholder="Enter a password..." />
    </div>
    <button type="submit" v-if="!loadingMessage">Sign Up</button>
    <button id="loading" v-if="loadingMessage" disabled>
      <svg class="animate-spin w-4 h-4 fill-current shrink-0" viewBox="0 0 16 16">
        <path d="M8 16a7.928 7.928 0 01-3.428-.77l.857-1.807A6.006 6.006 0 0014 8c0-3.309-2.691-6-6-6a6.006 6.006 0 00-5.422 8.572l-1.806.859A7.929 7.929 0 010 8c0-4.411 3.589-8 8-8s8 3.589 8 8-3.589 8-8 8z"></path>
      </svg>
      <span class="ml-2">{{ loadingMessage }}</span>
    </button>
  </form>
  <p>{{ confirmationMessage }}</p>
</template>

<script>
import { ref } from "vue";
import { supabase } from "../lib/supabaseClient";

export default {
  name: 'SignupComponent',
  setup() {
    const email = ref("");
    const password = ref("");
    const loadingMessage = ref(null);
    const confirmationMessage = ref("");

    const signUp = async () => {
      loadingMessage.value = 'Loading...';
      
      if (email.value == "" | password.value == "") {
        console.log('Please provide an email and password!')
        loadingMessage.value = null;
        return;
      }

      try {
        const { data, error } = await supabase.auth.signUp({
          email: email.value,
          password: password.value,
        });

        if (error) {
          console.error('Error signing up:', error.message);
          alert(error.message);
          loadingMessage.value = null;
          return;
        }

        confirmationMessage.value = `You have successfully signed up using ${email.value}`;
        alert('Check your email for the login link!');
      } catch (err) {
        console.error(err);
        if (error.message.includes('rate limit exceeded')) {
          alert('You have hit the rate limit for sending emails. Please try again later.');
        }
      } finally {
        loadingMessage.value = null;
      }
    };

    return {
      email,
      password,
      signUp,
      loadingMessage,
      confirmationMessage
    };
  }
};
</script>

<style scoped>

#signup-email {
  margin: 5px;
}

#signup-password {
  margin: 5px;
}

.loading-message {
  margin-top: 20px;
}
</style>