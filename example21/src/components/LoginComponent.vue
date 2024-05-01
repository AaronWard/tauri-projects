<template>
  <div>
    <form class="row" v-if="!session" @submit.prevent="login">
      <div style="display: flex; flex-direction: column;">
        <input id="login-email" type="email" v-model="email" placeholder="Enter an email address..." />
        <input id="login-password" type="password" v-model="password" placeholder="Enter a password..." />
      </div>
      <button type="submit" v-if="!loadingMessage">Login</button>
      <button type="button" @click="loginWithGoogle">Login with Google</button>
    </form>
    <button v-if="session" @click="signOut">Logout</button>
    <p>{{ confirmationMessage }}</p>
  </div>
</template>

<script>
import { ref, onMounted } from "vue";
import { supabase } from "../lib/supabaseClient";

export default {
  name: 'LoginComponent',
  setup() {
    const email = ref("");
    const password = ref("");
    const session = ref(null);
    const loadingMessage = ref(null);
    const confirmationMessage = ref("");

    onMounted(async () => {
      let { data } = await supabase.auth.getSession();
      session.value = data.session;
      supabase.auth.onAuthStateChange((_event, _session) => {
        session.value = _session;
      });
    });

    const login = async () => {
      loadingMessage.value = 'Loading...';
      if (!email.value || !password.value) {
        confirmationMessage.value = "Please provide an email and password!";
        loadingMessage.value = null;
        return;
      }

      const { error } = await supabase.auth.signInWithPassword({
        email: email.value,
        password: password.value,
      });

      if (error) {
        console.error('Error logging in:', error.message);
        confirmationMessage.value = `${error.message} - Try again!`;
      } else {
        confirmationMessage.value = `You have successfully logged in using ${email.value}, redirecting...`;
        // Add redirect logic here if necessary
      }

      loadingMessage.value = null;
    };

    const loginWithGoogle = async () => {
      loadingMessage.value = 'Redirecting to Google...';
      const { error } = await supabase.auth.signInWithOAuth({
        provider: 'google'
      });

      if (error) {
        console.error('Error with Google login:', error.message);
        confirmationMessage.value = `${error.message} - Try again!`;
      } else {
        // The page will redirect automatically
      }

      loadingMessage.value = null;
    };

    const signOut = async () => {
      const { error } = await supabase.auth.signOut();
      if (error) {
        console.error('Error signing out:', error.message);
      }
      confirmationMessage.value = 'Logged out successfully';
    };

    return {
      email,
      password,
      session,
      login,
      loginWithGoogle,
      signOut,
      loadingMessage,
      confirmationMessage
    };
  }
};
</script>

<style scoped>

#login-email {
  margin: 5px;
}

#login-password {
  margin: 5px;
}

.loading-message {
  margin-top: 20px;
}
</style>