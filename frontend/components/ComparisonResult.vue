<template>
  <div>
    <button
      @click="$emit('back')"
      class="text-sm text-indigo-600 hover:text-indigo-800 mb-4 inline-flex items-center gap-1"
    >
      &larr; Try a different dimension
    </button>

    <!-- Loading state -->
    <div v-if="isLoading" class="text-center py-12">
      <div class="animate-spin rounded-full h-10 w-10 border-b-2 border-indigo-600 mx-auto mb-4"></div>
      <p class="text-gray-500">Finding a crazy comparison...</p>
    </div>

    <!-- Result -->
    <div v-else-if="result" class="bg-white border border-gray-200 rounded-lg p-6">
      <div class="prose prose-indigo max-w-none whitespace-pre-wrap text-gray-800">
        {{ result.message }}
      </div>

      <div class="mt-6 flex flex-wrap gap-3">
        <button
          @click="$emit('retry')"
          class="px-4 py-2 bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition-colors text-sm"
        >
          Try another comparison
        </button>
        <button
          @click="$emit('startOver')"
          class="px-4 py-2 bg-gray-200 text-gray-700 rounded-lg hover:bg-gray-300 transition-colors text-sm"
        >
          Start over
        </button>
      </div>
    </div>

    <!-- Error -->
    <div v-else-if="error" class="bg-red-50 border border-red-200 rounded-lg p-6">
      <p class="text-red-700">{{ error }}</p>
      <button
        @click="$emit('retry')"
        class="mt-3 px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors text-sm"
      >
        Try again
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ComparisonResult } from '~/composables/useComparison'

defineProps<{
  result: ComparisonResult | null
  isLoading: boolean
  error: string | null
}>()

defineEmits<{
  back: []
  retry: []
  startOver: []
}>()
</script>
