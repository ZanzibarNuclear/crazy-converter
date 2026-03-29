<template>
  <div class="bg-white rounded-lg shadow-lg p-6 min-h-[60vh]">
    <!-- Step indicators -->
    <div class="flex items-center justify-center gap-2 mb-8">
      <span
        v-for="(label, i) in steps"
        :key="i"
        :class="[
          'px-3 py-1 rounded-full text-xs font-medium transition-colors',
          i === currentStep
            ? 'bg-indigo-600 text-white'
            : i < currentStep
              ? 'bg-indigo-100 text-indigo-700'
              : 'bg-gray-100 text-gray-400'
        ]"
      >
        {{ label }}
      </span>
    </div>

    <!-- Step 0: Pick item -->
    <ItemPicker
      v-if="currentStep === 0"
      :items="items"
      :categories="categories"
      :is-loading="isLoadingItems"
      @select="onItemSelect"
      @filter-category="onCategoryFilter"
    />

    <!-- Step 1: Pick dimension -->
    <DimensionSelector
      v-if="currentStep === 1 && selectedItem"
      :item="selectedItem"
      @select="onDimensionSelect"
      @back="currentStep = 0"
    />

    <!-- Step 2: See comparison -->
    <ComparisonResult
      v-if="currentStep === 2"
      :result="comparisonResult"
      :is-loading="isLoadingComparison"
      :error="error"
      @back="currentStep = 1"
      @retry="retryComparison"
      @start-over="startOver"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useComparison } from '~/composables/useComparison'

const {
  items,
  categories,
  selectedItem,
  comparisonResult,
  isLoadingItems,
  isLoadingComparison,
  error,
  fetchItems,
  fetchCategories,
  selectItem,
  compare,
  reset,
} = useComparison()

const currentStep = ref(0)
const selectedDimension = ref<string | null>(null)

const steps = ['Pick an item', 'Pick a dimension', 'See comparison']

onMounted(async () => {
  await Promise.all([fetchItems(), fetchCategories()])
})

const onCategoryFilter = async (category: string | null) => {
  await fetchItems(category || undefined)
}

const onItemSelect = async (itemId: string) => {
  await selectItem(itemId)
  currentStep.value = 1
}

const onDimensionSelect = async (dimension: string) => {
  selectedDimension.value = dimension
  currentStep.value = 2
  await compare(selectedItem.value!.id, dimension)
}

const retryComparison = async () => {
  if (selectedItem.value && selectedDimension.value) {
    await compare(selectedItem.value.id, selectedDimension.value)
  }
}

const startOver = () => {
  reset()
  currentStep.value = 0
  selectedDimension.value = null
}
</script>
