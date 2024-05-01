<script setup>
import { onMounted, ref } from 'vue';
import Account from "./components/Account.vue";
import LoginComponent from "./components/LoginComponent.vue";

const session = ref()

onMounted(() => {
  supabase.auth.getSession().then(({ data }) => {
    session.value = data.session
  })

  supabase.auth.onAuthStateChange((_, _session) => {
    session.value = _session
  })
})
</script>

<template>
  <div class="container">
    <h1>Supabase Login Page 📋</h1>
    <Account v-if="session" :session="session" />
    <LoginComponent v-else />
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
