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
    } catch (err: any) {
      error.value = err.message || 'Failed to get response from server'
      console.error('Error sending message:', err)
      
      // Add error message to conversation
      conversationHistory.value.push({
        role: 'assistant',
        content: `Sorry, I encountered an error: ${error.value}`
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

