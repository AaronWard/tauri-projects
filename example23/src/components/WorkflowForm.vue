<template>
  <div>
    <h1>Trigger GitHub Workflow</h1>
    <form @submit.prevent="triggerWorkflow">
      <input v-model="formData.name" placeholder="Your Name" required>
      <input v-model="formData.home" placeholder="Home City" required>
      <button type="submit" :disabled="loading || success">
        {{ buttonText }}
      </button>
    </form>
  </div>
</template>

<script>
export default {
  data() {
    return {
      formData: {
        name: '',
        home: ''
      },
      loading: false, // Add loading state to the data
      success: false, // Track success state
    };
  },
  computed: {
    buttonText() {
      if (this.success) {
        return 'Success';
      } else if (this.loading) {
        return 'Processing...';
      } else {
        return 'Trigger Workflow';
      }
    }
  },
  methods: {
    async triggerWorkflow() {
      this.loading = true;  // Set loading to true when the process starts
      this.success = false; // Reset success state on new submission

      const backendUrl = import.meta.env.VITE_BACKEND_URL;
      const owner = import.meta.env.VITE_GITHUB_ORG;
      const repo = import.meta.env.VITE_GITHUB_REPO;
      const workflowId = import.meta.env.VITE_WORKFLOW_ID;

      try {
        const response = await fetch(backendUrl, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({
            owner: owner,
            repo: repo,
            workflowId: workflowId,
            ref: 'main',
            inputs: {
              name: this.formData.name,
              home: this.formData.home,
            }
          })
        });

        if (response.ok) {
          const data = await response.json();
          console.log("Workflow triggered successfully:", data); // Log success to console
          this.success = true; // Set success state to true
          alert("Workflow triggered successfully: " + data.message);
        } else {
          const error = await response.json();
          console.error("Workflow trigger failed:", error); // Log errors to console
          alert("Workflow trigger failed: " + error.message);
        }
      } catch (error) {
        console.error("Failed to trigger workflow:", error);
        alert("Failed to trigger workflow: " + error.message);
      } finally {
        this.loading = false;  // Reset loading state regardless of outcome
      }
    }
  }
};
</script>
