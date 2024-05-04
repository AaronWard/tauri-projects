<template>
  <div>
    <h1>Trigger GitHub Workflow</h1>
    <form @submit.prevent="triggerWorkflow">
      <input v-model="formData.ref" placeholder="Branch (e.g., main)" required>
      <input v-model="formData.name" placeholder="Your Name" required>
      <input v-model="formData.home" placeholder="Home City" required>
      <button type="submit">Trigger Workflow</button>
    </form>
  </div>
</template>

<script>
export default {
  data() {
    return {
      formData: {
        ref: '',
        name: '',
        home: ''
      }
    };
  },
  methods: {
    async triggerWorkflow() {
      try {
        const response = await fetch('https://c29b-37-228-226-71.ngrok-free.app/api/trigger-workflow', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            owner: 'theyappstore',
            repo: 'yapp-backend',
            workflowId: 'build.yaml',
            ref: this.formData.ref,
            inputs: {
              name: this.formData.name,
              home: this.formData.home,
            }
          })
        });

        if (response.ok) {
          const data = await response.json();
          console.log("Workflow triggered successfully:", data); // Log success to console
          alert("Workflow triggered successfully: " + data.message);
        } else {
          const error = await response.json();
          console.error("Workflow trigger failed:", error); // Log errors to console
          alert("Workflow trigger failed: " + error.message);
        }
      } catch (error) {
        console.error("Failed to trigger workflow:", error);
        alert("Failed to trigger workflow: " + error.message);
      }
    }

  }
};
</script>