from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List, Optional
from app.services import llm_service

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
        # Convert conversation history format if needed
        history = [
            {"role": msg.role, "content": msg.content}
            for msg in request.conversation_history
        ]
        
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
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

