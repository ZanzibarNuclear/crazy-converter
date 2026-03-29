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
from app.services import conversion_service, comparison_service
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
        # Uses OpenAI-compatible API
        from pydantic_ai.models.openai import OpenAIModel

        ollama_base_url = os.getenv("OLLAMA_BASE_URL", "http://localhost:11434/v1")
        logger.info(f"Using Ollama model: {model_name} at {ollama_base_url}")
        return OpenAIModel(model_name, base_url=ollama_base_url, api_key="ollama")
    
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

# System prompt for the comparison agent
SYSTEM_PROMPT = """You are the Crazy Converterator. Your job is to help people understand \
unfamiliar quantities by finding surprising, memorable comparisons between real-world items.

You have a catalog of items (bananas, cows, lightning bolts, the Sun, etc.) with verified \
physical properties across many dimensions (energy, mass, length, volume, time, etc.).

Your workflow:
1. When given an item and dimension, use lookup_item to get its verified property value.
2. Use search_catalog to find comparison targets in the same dimension — prefer items from \
a very different category (food vs. astronomy, animals vs. engineering, human body vs. nuclear physics).
3. Use convert_unit to compute the exact equivalence between the two items' properties.
4. Present the result as a fun, memorable statement.

Guidelines for great comparisons:
- Cross-domain is best: comparing food to nuclear physics, insects to rockets, heartbeats to geology.
- Surprising ratios are best: "10 billion bananas" or "0.00000001 supernovas" are more memorable than "3.2 cars."
- Lead with the punchline: "A month of cow farts could drive you from New York to Philadelphia." Then explain the chain.
- Always use the catalog's verified numbers. If a user asks about something not in the catalog, \
you can use your knowledge, but note that the number is approximate.
- Keep the tone fun and slightly irreverent, but the math must be exact.

For multi-step chains (A→B across dimension 1, then B→C across dimension 2):
- Show each step clearly
- Use convert_unit for each conversion
- The chain reveals non-obvious connections — that's the whole point

Available dimensions: time, length, area, volume, mass, speed, acceleration, force, pressure, \
energy, power, momentum, torque, temperature.
"""

# Create the agent with conversion, comparison, and Physics MCP tools
all_tools = (
    conversion_service.conversion_tools
    + comparison_service.comparison_tools
    + physics_client.physics_mcp_tools
)

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
