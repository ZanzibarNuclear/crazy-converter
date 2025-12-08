<template>
  <div class="bg-white rounded-lg shadow-lg flex flex-col h-[calc(100vh-12rem)]">
    <!-- Messages area -->
    <div ref="messagesContainer" class="flex-1 overflow-y-auto p-6 space-y-4">
      <div
        v-for="(msg, index) in conversationHistory"
        :key="index"
        :class="[
          'flex',
          msg.role === 'user' ? 'justify-end' : 'justify-start'
        ]"
      >
        <div
          :class="[
            'max-w-[80%] rounded-lg px-4 py-2',
            msg.role === 'user'
              ? 'bg-blue-600 text-white'
              : 'bg-gray-200 text-gray-800'
          ]"
        >
          <div class="whitespace-pre-wrap">{{ msg.content }}</div>
        </div>
      </div>
      <div v-if="isLoading" class="flex justify-start">
        <div class="bg-gray-200 text-gray-800 rounded-lg px-4 py-2">
          <div class="flex items-center space-x-2">
            <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-gray-600"></div>
            <span>Thinking...</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Input area -->
    <div class="border-t p-4">
      <form @submit.prevent="sendMessage" class="flex space-x-2">
        <input
          v-model="inputMessage"
          type="text"
          placeholder="Ask me to convert something..."
          class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          :disabled="isLoading"
        />
        <button
          type="submit"
          :disabled="isLoading || !inputMessage.trim()"
          class="px-6 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition-colors"
        >
          Send
        </button>
      </form>
    </div>
  </div>
</template>

<script setup>
import { ref, nextTick, watch } from 'vue'
import { useChat } from '~/composables/useChat'

const { conversationHistory, isLoading, sendMessage: sendChatMessage } = useChat()
const inputMessage = ref('')
const messagesContainer = ref(null)

const sendMessage = async () => {
  if (!inputMessage.value.trim() || isLoading.value) return
  
  const message = inputMessage.value.trim()
  inputMessage.value = ''
  
  await sendChatMessage(message)
  
  // Scroll to bottom after message is added
  await nextTick()
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
}

// Auto-scroll when new messages arrive
watch(conversationHistory, () => {
  nextTick(() => {
    if (messagesContainer.value) {
      messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
    }
  })
}, { deep: true })
</script>

