"""
LLM service that provides chat responses using various LLM providers.

Supported providers:
- ollama: Local models via Ollama (free, private, offline)
- groq: Free cloud API with fast inference
- openai: OpenAI API (paid)
- anthropic: Anthropic API (paid)
"""
import os
import logging
from typing import List, Dict
from dotenv import load_dotenv
from pydantic_ai import Agent
from app.services import conversion_service
from app.mcp import physics_client

load_dotenv()

logger = logging.getLogger(__name__)

# Configuration from environment
model_provider = os.getenv("LLM_PROVIDER", "ollama")
model_name = os.getenv("LLM_MODEL", "qwen2.5-coder:7b")


def get_model():
    """
    Initialize and return the appropriate LLM model based on configuration.
    
    Provider options:
    - ollama: Local models (default, free)
    - groq: Free cloud API with generous limits
    - openai: OpenAI API (paid)
    - anthropic: Anthropic API (paid)
    """
    provider = model_provider.lower()
    
    if provider == "ollama":
        # Ollama - local models, completely free
        # Popular coding models: qwen2.5-coder, deepseek-coder-v2, codellama, devstral
        from pydantic_ai.models.ollama import OllamaModel
        
        ollama_base_url = os.getenv("OLLAMA_BASE_URL", "http://localhost:11434/v1")
        logger.info(f"Using Ollama model: {model_name} at {ollama_base_url}")
        return OllamaModel(model_name, base_url=ollama_base_url)
    
    elif provider == "groq":
        # Groq - free cloud API with OpenAI-compatible endpoint
        # Fast inference on models like llama-3.3-70b-versatile
        from pydantic_ai.models.openai import OpenAIModel
        
        groq_api_key = os.getenv("GROQ_API_KEY")
        if not groq_api_key:
            raise ValueError("GROQ_API_KEY environment variable required for Groq provider")
        
        logger.info(f"Using Groq model: {model_name}")
        return OpenAIModel(
            model_name,
            base_url="https://api.groq.com/openai/v1",
            api_key=groq_api_key
        )
    
    elif provider == "openai":
        # OpenAI - paid API
        from pydantic_ai.models.openai import OpenAIModel
        
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            raise ValueError("OPENAI_API_KEY environment variable required for OpenAI provider")
        
        logger.info(f"Using OpenAI model: {model_name}")
        return OpenAIModel(model_name, api_key=api_key)
    
    elif provider == "anthropic":
        # Anthropic - paid API
        from pydantic_ai.models.anthropic import AnthropicModel
        
        api_key = os.getenv("ANTHROPIC_API_KEY")
        if not api_key:
            raise ValueError("ANTHROPIC_API_KEY environment variable required for Anthropic provider")
        
        logger.info(f"Using Anthropic model: {model_name}")
        return AnthropicModel(model_name, api_key=api_key)
    
    else:
        raise ValueError(
            f"Unknown model provider: {model_provider}. "
            f"Supported providers: ollama, groq, openai, anthropic"
        )


# Initialize the model
model = get_model()

# System prompt for the conversion agent
SYSTEM_PROMPT = """You are the Crazy Converterator, a helpful assistant that converts between different units and measurements.

Your role is to:
1. Understand user queries about unit conversions
2. Use the available conversion tools to perform accurate conversions
3. Provide clear, step-by-step explanations of how you arrived at your answer
4. Be friendly and conversational while maintaining accuracy

You can convert between many types of units:
- Time (seconds, minutes, hours, days, etc.)
- Length (meters, feet, miles, kilometers, etc.)
- Area (square meters, acres, hectares, etc.)
- Volume (liters, gallons, cubic meters, etc.)
- Mass (kilograms, pounds, grams, etc.)
- Speed (m/s, mph, km/h, etc.)
- Acceleration, Force, Pressure
- Energy, Power, Momentum, Torque
- Temperature (Celsius, Fahrenheit, Kelvin, Rankine)

When a user asks a conversion question, identify the relevant conversion category and use the appropriate tool.
Always explain your reasoning and show the calculation when possible.
"""

# Create the agent with conversion tools and Physics MCP tools
all_tools = conversion_service.conversion_tools + physics_client.physics_mcp_tools

agent = Agent(
    model=model,
    system_prompt=SYSTEM_PROMPT,
    tools=all_tools,
)


async def get_chat_response(message: str, conversation_history: List[Dict[str, str]] = None) -> str:
    """
    Get a chat response from the LLM agent.
    
    Args:
        message: The user's message
        conversation_history: Previous messages in the conversation
    
    Returns:
        The agent's response as a string
    
    Raises:
        ValueError: If the message is empty or invalid
        RuntimeError: If the LLM service fails to respond
    """
    if not message or not message.strip():
        raise ValueError("Message cannot be empty")
    
    if conversation_history is None:
        conversation_history = []
    
    try:
        # Run the agent with the message
        # Pydantic AI handles conversation context internally through the agent's run method
        # We pass the user message directly
        logger.debug(f"Calling LLM agent with message: {message[:100]}...")
        result = await agent.run(message)
        
        # Extract the text response from the result
        # Pydantic AI returns a Result object with .data containing the response
        if hasattr(result, 'data'):
            response = str(result.data)
        else:
            response = str(result)
        
        logger.debug(f"LLM response received: {response[:100]}...")
        return response
    except Exception as e:
        logger.error(f"Error getting LLM response: {str(e)}", exc_info=True)
        raise RuntimeError(f"Failed to get response from LLM: {str(e)}")
