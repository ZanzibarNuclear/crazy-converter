<template>
  <div class="min-h-screen bg-gray-50">
    <div class="container mx-auto px-4 py-8 max-w-4xl">
      <h1 class="text-4xl font-bold text-center mb-2 text-gray-800">
        Crazy Converterator
      </h1>
      <p class="text-center text-gray-500 mb-8">
        Understand the unfamiliar by comparing it to what you know
      </p>

      <!-- Mode tabs -->
      <div class="flex justify-center gap-2 mb-6">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          @click="activeTab = tab.id"
          :class="[
            'px-4 py-2 rounded-lg text-sm font-medium transition-colors',
            activeTab === tab.id
              ? 'bg-indigo-600 text-white'
              : 'bg-white text-gray-600 hover:bg-gray-100 border border-gray-200'
          ]"
        >
          {{ tab.label }}
        </button>
      </div>

      <!-- Explore mode -->
      <ExplorationFlow v-if="activeTab === 'explore'" />

      <!-- Chat mode -->
      <ChatInterface v-if="activeTab === 'chat'" />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const tabs = [
  { id: 'explore', label: 'Explore' },
  { id: 'chat', label: 'Ask anything' },
]
const activeTab = ref('explore')
</script>
