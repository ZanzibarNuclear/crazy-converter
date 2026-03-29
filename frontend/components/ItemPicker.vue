<template>
  <div>
    <h2 class="text-xl font-semibold text-gray-800 mb-4">Pick something familiar</h2>

    <!-- Category filter -->
    <div class="flex flex-wrap gap-2 mb-4">
      <button
        v-for="cat in allCategories"
        :key="cat"
        @click="toggleCategory(cat)"
        :class="[
          'px-3 py-1 rounded-full text-sm font-medium transition-colors',
          activeCategory === cat
            ? 'bg-indigo-600 text-white'
            : 'bg-gray-200 text-gray-700 hover:bg-gray-300'
        ]"
      >
        {{ formatCategory(cat) }}
      </button>
    </div>

    <!-- Search -->
    <input
      v-model="searchQuery"
      type="text"
      placeholder="Search items..."
      class="w-full px-4 py-2 mb-4 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
    />

    <!-- Item grid -->
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
      <button
        v-for="item in filteredItems"
        :key="item.id"
        @click="$emit('select', item.id)"
        class="p-3 bg-white border border-gray-200 rounded-lg hover:border-indigo-400 hover:shadow-md transition-all text-left"
      >
        <div class="font-medium text-gray-800 text-sm">{{ item.name }}</div>
        <div class="text-xs text-gray-500 mt-1">{{ item.dimensions.length }} dimensions</div>
      </button>
    </div>

    <p v-if="filteredItems.length === 0 && !isLoading" class="text-gray-500 text-center py-8">
      No items found. Try a different search or category.
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type { ItemSummary } from '~/composables/useComparison'

const props = defineProps<{
  items: ItemSummary[]
  categories: string[]
  isLoading: boolean
}>()

const emit = defineEmits<{
  select: [itemId: string]
  filterCategory: [category: string | null]
}>()

const searchQuery = ref('')
const activeCategory = ref<string | null>(null)

const allCategories = computed(() => props.categories)

const filteredItems = computed(() => {
  let result = props.items
  if (searchQuery.value) {
    const q = searchQuery.value.toLowerCase()
    result = result.filter(i => i.name.toLowerCase().includes(q))
  }
  return result
})

const toggleCategory = (cat: string) => {
  if (activeCategory.value === cat) {
    activeCategory.value = null
    emit('filterCategory', null)
  } else {
    activeCategory.value = cat
    emit('filterCategory', cat)
  }
}

const formatCategory = (cat: string) => {
  return cat.replace(/_/g, ' ').replace(/\b\w/g, c => c.toUpperCase())
}
</script>
