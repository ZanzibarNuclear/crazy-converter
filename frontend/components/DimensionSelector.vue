<template>
  <div>
    <button
      @click="$emit('back')"
      class="text-sm text-indigo-600 hover:text-indigo-800 mb-4 inline-flex items-center gap-1"
    >
      &larr; Pick a different item
    </button>

    <h2 class="text-xl font-semibold text-gray-800 mb-2">
      {{ item.name }}
    </h2>
    <p class="text-gray-500 mb-4">Pick a dimension to compare</p>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
      <button
        v-for="prop in item.properties"
        :key="prop.dimension + (prop.qualifier || '')"
        @click="$emit('select', prop.dimension)"
        class="p-4 bg-white border border-gray-200 rounded-lg hover:border-indigo-400 hover:shadow-md transition-all text-left"
      >
        <div class="font-medium text-gray-800">{{ prop.label }}</div>
        <div class="text-lg text-indigo-600 font-semibold mt-1">
          {{ formatValue(prop.value) }} {{ prop.unit }}
        </div>
        <div class="text-xs text-gray-400 mt-1">{{ prop.dimension }}</div>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ItemDetail } from '~/composables/useComparison'

defineProps<{
  item: ItemDetail
}>()

defineEmits<{
  select: [dimension: string]
  back: []
}>()

const formatValue = (value: number): string => {
  if (value === 0) return '0'
  if (Math.abs(value) >= 1e9) return value.toExponential(2)
  if (Math.abs(value) < 0.001) return value.toExponential(2)
  if (Number.isInteger(value)) return value.toLocaleString()
  return value.toLocaleString(undefined, { maximumSignificantDigits: 4 })
}
</script>
