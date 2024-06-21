<template>
    <div class="relative inline-flex w-full">
      <button
        ref="trigger"
        class="btn w-full justify-between min-w-44 bg-white dark:bg-slate-800 border-slate-200 dark:border-slate-700 hover:border-slate-300 dark:hover:border-slate-600 text-slate-500 hover:text-slate-600 dark:text-slate-300 dark:hover:text-slate-200"
        aria-label="Select date range"
        aria-haspopup="true"
        @click.prevent="dropdownOpen = !dropdownOpen"
        :aria-expanded="dropdownOpen"
      >
        <span class="flex items-center">
          <span>{{ selectedRepo ? selectedRepo.period : 'Select Repository' }}</span>
        </span>
        <svg class="shrink-0 ml-1 fill-current text-slate-400" width="11" height="7" viewBox="0 0 11 7">
          <path d="M5.4 6.8L0 1.4 1.4 0l4 4 4-4 1.4 1.4z" />
        </svg>
      </button>
      <transition
        enter-active-class="transition ease-out duration-100 transform"
        enter-from-class="opacity-0 -translate-y-2"
        enter-to-class="opacity-100 translate-y-0"
        leave-active-class="transition ease-out duration-100"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div v-show="dropdownOpen" class="z-10 absolute top-full left-0 w-full bg-white dark:bg-slate-800 border border-slate-200 dark:border-slate-700 py-1.5 rounded shadow-lg overflow-hidden mt-1">
          <div
            ref="dropdown"
            class="font-medium text-sm text-slate-600 dark:text-slate-300 divide-y divide-slate-200 dark:divide-slate-700"
            @focusin="dropdownOpen = true"
            @focusout="dropdownOpen = false"
          >
            <input
              type="text"
              placeholder="Search..."
              v-model="searchQuery"
              class="w-full p-2 mb-2 ml-2 mr-5"
            />
            <button
              v-for="option in filteredOptions"
              :key="option.id"
              class="flex items-center justify-between w-full hover:bg-slate-50 dark:hover:bg-slate-700/20 py-2 px-3 cursor-pointer"
              :class="option.id === selected && 'text-pink-500'"
              @click="selectOption(option)"
            >
              <span>{{ option.period }}</span>
              <svg class="shrink-0 ml-2 fill-current text-pink-400" :class="option.id !== selected && 'invisible'" width="12" height="9" viewBox="0 0 12 9">
                <path d="M10.28.28L3.989 6.575 1.695 4.28A1 1 0 00.28 5.695l3 3a1 1 0 001.414 0l7-7A1 1 0 0010.28.28z" />
              </svg>
            </button>          
          </div>
        </div>
      </transition>
    </div>
  </template>
  
  <script>
  import { ref, onMounted, onUnmounted, computed } from 'vue'
  
  export default {
    name: 'DropdownFull',
    props: {
      options: Array
    },
    setup(props, { emit }) {
      const dropdownOpen = ref(false)
      const trigger = ref(null)
      const dropdown = ref(null)    
      const selected = ref(0)
      const selectedRepo = ref(null)
      const searchQuery = ref('')
  
      console.log(props.options)
  
      const filteredOptions = computed(() => {
        if (!searchQuery.value) {
          return props.options
        }
        return props.options.filter(option =>
          option.period.toLowerCase().includes(searchQuery.value.toLowerCase())
        )
      })
  
      const clickHandler = ({ target }) => {
        if (!dropdownOpen.value || dropdown.value.contains(target) || trigger.value.contains(target)) return
        dropdownOpen.value = false
      }
  
      const keyHandler = ({ keyCode }) => {
        if (!dropdownOpen.value || keyCode !== 27) return
        dropdownOpen.value = false
      }
  
      const selectOption = (option) => {
        selectedRepo.value = option;
        selected.value = option.id;
        dropdownOpen.value = false;
        emit('selectRepo', option);
      }
  
      onMounted(() => {
        document.addEventListener('click', clickHandler)
        document.addEventListener('keydown', keyHandler)
      })
  
      onUnmounted(() => {
        document.removeEventListener('click', clickHandler)
        document.removeEventListener('keydown', keyHandler)
      })    
      
      return {
        dropdownOpen,
        trigger,
        dropdown,
        selected,
        selectedRepo,
        searchQuery,
        filteredOptions,
        selectOption
      }
    }
  }
  </script>
  