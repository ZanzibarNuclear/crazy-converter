import os
from typing import List, Dict
from dotenv import load_dotenv
from pydantic_ai import Agent
from app.services import conversion_service
from app.mcp import physics_client

load_dotenv()

# Initialize the Pydantic AI agent
# Default to OpenAI, but can be changed via environment variable
model_provider = os.getenv("LLM_PROVIDER", "openai")
model_name = os.getenv("LLM_MODEL", "gpt-4o-mini")

if model_provider == "openai":
    from pydantic_ai.models.openai import OpenAIModel
    model = OpenAIModel(model_name, api_key=os.getenv("OPENAI_API_KEY"))
elif model_provider == "anthropic":
    from pydantic_ai.models.anthropic import AnthropicModel
    model = AnthropicModel(model_name, api_key=os.getenv("ANTHROPIC_API_KEY"))
else:
    raise ValueError(f"Unknown model provider: {model_provider}")

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
    """
    if conversation_history is None:
        conversation_history = []
    
    # Run the agent with the message
    # Pydantic AI handles conversation context internally through the agent's run method
    # We pass the user message directly
    result = await agent.run(message)
    
    # Extract the text response from the result
    # Pydantic AI returns a Result object with .data containing the response
    if hasattr(result, 'data'):
        return str(result.data)
    else:
        return str(result)

