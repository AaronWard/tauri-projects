<template>
  <div class="flex h-[100dvh] overflow-hidden">
    <!-- Sidebar -->

    <!-- Content area -->
    <div class="relative flex flex-col flex-1 overflow-y-auto overflow-x-hidden">
      <!-- Site header -->

      <main class="grow">
        <div class="lg:relative lg:flex">
          <!-- Content -->
          <div class="px-4 sm:px-6 lg:px-8 py-8 lg:grow lg:pr-8 xl:pr-16 2xl:ml-[80px]">
            <div class="lg:max-w-[900px] lg:mx-auto">

              <!-- Cart items -->
              <div class="mb-6 lg:mb-0">
                <!-- Submission Information -->
                <div>
                  <div class="text-slate-800 dark:text-slate-100 font-semibold mb-4">Fill out the form below:</div>

                  <p class="mt-3">
                    For Yapp Store to bundle your project into an executable application, you must first provide access
                    to the Yapp Github App.
                    <br><br>
                  </p>

                  <form @submit.prevent="startGitHubOAuth">
                    <button @click="startGitHubOAuth"
                      class="btn dark:bg-slate-200 bg-slate-900 text-white dark:text-black mt-2"
                      :disabled="loading || success">
                      {{ githubButtonText }}
                    </button>
                  </form>

                  <div v-if="repositories.length" class="mt-3">
                    <DropdownFull :options="repositories" @selectRepo="selectRepo" />
                  </div>

                  <div v-else class="mt-3">
                    <p>No repositories found. Please install the GitHub App to grant access.</p>
                  </div>


                  <div class="mt-3">
                    <p>Missing a repository? make sure access has been granted to read the repository</p>
                    <a href="https://github.com/settings/installations/51940992" target="_blank" class="btn dark:bg-slate-200 bg-slate-900 text-white dark:text-black mt-2">Install GitHub App</a>

                  </div>

                  <form @submit.prevent="triggerWorkflow" v-if="repositories.length">
                    <hr class="my-6 border-t border-slate-200 dark:border-slate-700" />
                    <div class="text-right">
                      <button type="submit"
                        class="btn mt-2 bg-white dark:bg-slate-800 border-slate-200 dark:border-slate-700 hover:border-slate-300 dark:hover:border-slate-600 text-pink-500"
                        :disabled="loading || success">
                        {{ buttonText }}
                      </button>
                    </div>
                  </form>


                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script>
import { ref } from 'vue';
import DropdownFull from './DropdownFull.vue';

export default {
  name: 'Submission',
  components: {
    DropdownFull
  },
  setup() {
    const safetyToggle = ref('No');
    return {
      safetyToggle
    };
  },
  data() {
    return {
      formData: {
        previewImage: null,
        name: 'Example App Submission',
        description: 'This is a random description for the app',
        subtitle: 'Putting the Yapp into Apps',
        projectType: null,
        framework: 'vue',
        bundler: 'vite',
        package_manager: 'npm',
        build_command: 'npm run build',
        install_command: 'npm install',
        windowTitle: 'Title',
        fullscreenToggle: false,
        resizableToggle: false,
        category: 'Productivity',
        privacyPolicy: 'example.com/privacy',
        fsToggle: false,
        shellToggle: false,
        sqlToggle: false,
        monetizationToggle: false,
        agreementToggle: false,
        safetyToggle: false,
        targetOperatingSystems: {
          MacOS: false,
          Windows: false,
          Linux: false,
        },
      },
      loading: false,
      success: false,
      tags: ['Test Tag'],
      token: null,
      repositories: [],
      selectedRepo: '',
    };
  },
  computed: {
    buttonText() {
      if (this.success) {
        return 'Success';
      } else if (this.loading) {
        return 'Processing...';
      } else {
        return 'Submit for Review';
      }
    },
    githubButtonText() {
      if (this.success) {
        return 'Synced';
      } else if (this.loading) {
        return 'Syncing...';
      } else {
        return 'Sync Github Repository';
      }
    },
  },
  created() {
    const urlParams = new URLSearchParams(window.location.search);
    this.token = urlParams.get('token');
    if (this.token) {
      this.fetchRepositories();
    }
  },
  methods: {
    async fetchRepositories() {
      try {
        const response = await fetch(`${import.meta.env.VITE_BACKEND_REPO_URL}`, {
          method: 'POST',
          headers: {
            Authorization: `Bearer ${this.token}`,
          },
        });

        console.log('Response:', response);
        console.log('Token:', this.token);

        if (response.status === 429) {
          console.error('Rate limit exceeded. Please wait and try again.');
          alert('Rate limit exceeded. Please wait and try again.');
          return;
        }

        if (response.ok) {
          const data = await response.json();
          console.log('Repositories:', data); // Log repositories for debugging
          this.repositories = data.map((repo, index) => ({
            id: index,
            period: repo.full_name,
            private: repo.private,
          }));
        } else {
          console.error('Failed to fetch repositories:', response.statusText);
        }
      } catch (error) {
        console.error('Error fetching repositories:', error);
      }
    },
    selectRepo(repo) {
      this.selectedRepo = repo;

      const selectedRepo = {
        name: repo.full_name,
        token: this.token,
      };
      localStorage.setItem('selectedRepo', JSON.stringify(selectedRepo));
      this.$router.push('/');
    },
    wakeUpLambda() {
      const backendUrl = import.meta.env.VITE_BACKEND_WAKEUP; // Define a new endpoint for wake-up
      fetch(backendUrl, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      })
        .then((response) => {
          if (response.ok) {
            console.log('Lambda function wake-up successful');
          } else {
            console.error('Lambda function wake-up failed');
          }
        })
        .catch((error) => {
          console.error('API Issue:', error);
        });
    },
    async startGitHubOAuth() {
      const clientId = import.meta.env.VITE_GITHUB_CLIENT_ID;
      const redirectUri = import.meta.env.VITE_BACKEND_OAUTH_URL;
      const scope = 'repo';
      const oauthUrl = `https://github.com/login/oauth/authorize?client_id=${clientId}&redirect_uri=${redirectUri}&scope=${scope}`;

      console.log(`OAuth URL: ${oauthUrl}`); // Log the OAuth URL
      window.location.href = oauthUrl;
    },
    async triggerWorkflow() {
      if (!this.selectedRepo) {
        alert('Please select a repository.');
        return;
      }
      
      this.loading = true;
      this.success = false;

      const backendUrl = import.meta.env.VITE_BACKEND_URL;
      const encodedDescription = btoa(this.formData.description);

      const requestBody = {
        ref: 'main',
        inputs: {
          config_json: JSON.stringify({
            ...this.formData,
            description: encodedDescription,
            app_creator_id: '782d241b-ca64-4936-9fa3-380a82580e95',
          }),
          repo_full_name: this.selectedRepo.period,
          repo_private: this.selectedRepo.private,
          repo_token: this.token,
        },
      };

      console.log('Request Body:', requestBody.inputs);

      try {
        const response = await fetch(backendUrl, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify(requestBody),
        });

        console.log('Raw Response:', response);

        if (response.ok) {
          const data = await response.json();
          console.log('Workflow triggered successfully:', data);
          this.success = true;
          alert('Workflow triggered successfully');
        } else {
          const error = await response.json();
          console.error('Workflow trigger failed:', error);
          alert('Workflow trigger failed: ' + error.message);
        }
      } catch (error) {
        console.error('API Issue:', error);
        alert('Uh-oh, something broke. Please try again!');
      } finally {
        this.loading = false;
      }
    },
  },
};
</script>
