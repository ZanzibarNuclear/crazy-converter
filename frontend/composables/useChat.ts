export interface ChatMessage {
  role: 'user' | 'assistant'
  content: string
}

export const useChat = () => {
  const conversationHistory = ref<ChatMessage[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  const config = useRuntimeConfig()
  const apiBase = config.public.apiBase || 'http://localhost:8000'

  const sendMessage = async (message: string) => {
    if (!message.trim()) return

    // Add user message to history
    conversationHistory.value.push({
      role: 'user',
      content: message
    })

    isLoading.value = true
    error.value = null

    try {
      const response = await $fetch<{
        message: string
        conversation_history: ChatMessage[]
      }>(`${apiBase}/api/chat`, {
        method: 'POST',
        body: {
          message,
          conversation_history: conversationHistory.value.slice(0, -1) // Exclude the message we just added
        }
      })

      // Update conversation history with the response
      conversationHistory.value = response.conversation_history
      error.value = null
    } catch (err: any) {
      // Handle different types of errors
      let errorMessage = 'Failed to get response from server'
      
      if (err.statusCode === 400) {
        errorMessage = err.data?.detail || 'Invalid request. Please check your message.'
      } else if (err.statusCode === 500) {
        errorMessage = err.data?.detail || 'Server error. Please try again later.'
      } else if (err.statusCode === 0 || !err.statusCode) {
        errorMessage = 'Unable to connect to server. Please check if the backend is running.'
      } else {
        errorMessage = err.data?.detail || err.message || errorMessage
      }
      
      error.value = errorMessage
      console.error('Error sending message:', err)
      
      // Add error message to conversation
      conversationHistory.value.push({
        role: 'assistant',
        content: `Sorry, I encountered an error: ${errorMessage}`
      })
    } finally {
      isLoading.value = false
    }
  }

  return {
    conversationHistory,
    isLoading,
    error,
    sendMessage
  }
}

