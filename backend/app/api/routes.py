import logging
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List, Optional
from app.services import llm_service

logger = logging.getLogger(__name__)

router = APIRouter()


class ChatMessage(BaseModel):
    role: str  # "user" or "assistant"
    content: str


class ChatRequest(BaseModel):
    message: str
    conversation_history: Optional[List[ChatMessage]] = []


class ChatResponse(BaseModel):
    message: str
    conversation_history: List[ChatMessage]


@router.post("/chat", response_model=ChatResponse)
async def chat(request: ChatRequest):
    """
    Handle a chat message and return the AI's response.
    """
    try:
        if not request.message or not request.message.strip():
            raise HTTPException(status_code=400, detail="Message cannot be empty")
        
        # Convert conversation history format if needed
        history = [
            {"role": msg.role, "content": msg.content}
            for msg in request.conversation_history
        ]
        
        logger.info(f"Processing chat message: {request.message[:50]}...")
        
        # Get response from LLM service
        response = await llm_service.get_chat_response(
            message=request.message,
            conversation_history=history
        )
        
        # Build response with updated conversation history
        updated_history = request.conversation_history + [
            ChatMessage(role="user", content=request.message),
            ChatMessage(role="assistant", content=response)
        ]
        
        return ChatResponse(
            message=response,
            conversation_history=updated_history
        )
    except HTTPException:
        # Re-raise HTTP exceptions as-is
        raise
    except ValueError as e:
        # Handle validation errors
        logger.error(f"Validation error: {str(e)}")
        raise HTTPException(status_code=400, detail=f"Invalid request: {str(e)}")
    except Exception as e:
        # Handle unexpected errors
        logger.error(f"Unexpected error processing chat message: {str(e)}", exc_info=True)
        raise HTTPException(
            status_code=500,
            detail="An error occurred while processing your message. Please try again."
        )

