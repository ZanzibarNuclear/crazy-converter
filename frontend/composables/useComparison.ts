export interface ItemProperty {
  dimension: string
  value: number
  unit: string
  label: string
  qualifier?: string
}

export interface ItemSummary {
  id: string
  name: string
  category: string
  dimensions: string[]
}

export interface ItemDetail {
  id: string
  name: string
  category: string
  properties: ItemProperty[]
}

export interface ComparisonResult {
  message: string
  source_item: string
  dimension: string
}

export const useComparison = () => {
  const items = ref<ItemSummary[]>([])
  const categories = ref<string[]>([])
  const selectedItem = ref<ItemDetail | null>(null)
  const comparisonResult = ref<ComparisonResult | null>(null)
  const isLoadingItems = ref(false)
  const isLoadingComparison = ref(false)
  const error = ref<string | null>(null)

  const config = useRuntimeConfig()
  const apiBase = config.public.apiBase || 'http://localhost:8000'

  const fetchItems = async (category?: string, dimension?: string) => {
    isLoadingItems.value = true
    error.value = null
    try {
      const params = new URLSearchParams()
      if (category) params.set('category', category)
      if (dimension) params.set('dimension', dimension)
      const query = params.toString() ? `?${params.toString()}` : ''
      items.value = await $fetch<ItemSummary[]>(`${apiBase}/api/items${query}`)
    } catch (err: any) {
      error.value = err.data?.detail || 'Failed to load items'
    } finally {
      isLoadingItems.value = false
    }
  }

  const fetchCategories = async () => {
    try {
      categories.value = await $fetch<string[]>(`${apiBase}/api/items/categories`)
    } catch (err: any) {
      error.value = err.data?.detail || 'Failed to load categories'
    }
  }

  const selectItem = async (itemId: string) => {
    error.value = null
    try {
      selectedItem.value = await $fetch<ItemDetail>(`${apiBase}/api/items/${itemId}`)
      comparisonResult.value = null
    } catch (err: any) {
      error.value = err.data?.detail || 'Failed to load item'
    }
  }

  const compare = async (itemId: string, dimension: string) => {
    isLoadingComparison.value = true
    error.value = null
    comparisonResult.value = null
    try {
      const response = await $fetch<ComparisonResult>(`${apiBase}/api/compare`, {
        method: 'POST',
        body: { item_id: itemId, dimension },
      })
      comparisonResult.value = response
    } catch (err: any) {
      error.value = err.data?.detail || 'Failed to generate comparison'
    } finally {
      isLoadingComparison.value = false
    }
  }

  const reset = () => {
    selectedItem.value = null
    comparisonResult.value = null
    error.value = null
  }

  return {
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
  }
}
