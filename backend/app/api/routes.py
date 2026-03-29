import logging
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List, Optional
from app.services import llm_service
from app.data.item_store import catalog

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


# --- Item catalog endpoints ---


class PropertyResponse(BaseModel):
    dimension: str
    value: float
    unit: str
    label: str
    qualifier: Optional[str] = None


class ItemResponse(BaseModel):
    id: str
    name: str
    category: str
    properties: list[PropertyResponse]


class ItemSummaryResponse(BaseModel):
    id: str
    name: str
    category: str
    dimensions: list[str]


@router.get("/items", response_model=list[ItemSummaryResponse])
async def list_items(category: Optional[str] = None, dimension: Optional[str] = None):
    """List all items, optionally filtered by category or dimension."""
    if category and dimension:
        by_dim = {i.id for i in catalog.by_dimension(dimension)}
        items = [i for i in catalog.by_category(category) if i.id in by_dim]
    elif category:
        items = catalog.by_category(category)
    elif dimension:
        items = catalog.by_dimension(dimension)
    else:
        items = catalog.all_items()

    return [
        ItemSummaryResponse(
            id=i.id, name=i.name, category=i.category, dimensions=i.dimensions()
        )
        for i in items
    ]


@router.get("/items/categories", response_model=list[str])
async def list_categories():
    """List all item categories."""
    return catalog.categories()


@router.get("/items/dimensions", response_model=list[str])
async def list_dimensions():
    """List all available dimensions across items."""
    return catalog.dimensions()


@router.get("/items/{item_id}", response_model=ItemResponse)
async def get_item(item_id: str):
    """Get a specific item with all its properties."""
    item = catalog.get(item_id)
    if item is None:
        raise HTTPException(status_code=404, detail=f"Item '{item_id}' not found")
    return ItemResponse(
        id=item.id,
        name=item.name,
        category=item.category,
        properties=[PropertyResponse(**p.model_dump()) for p in item.properties],
    )


class CompareRequest(BaseModel):
    item_id: str
    dimension: str


class CompareResponse(BaseModel):
    message: str
    source_item: str
    dimension: str
    conversation_history: List[ChatMessage]


@router.post("/compare", response_model=CompareResponse)
async def compare(request: CompareRequest):
    """
    Given an item and dimension, find a surprising comparison.
    Uses the LLM with the item catalog and conversion tools.
    """
    item = catalog.get(request.item_id)
    if item is None:
        raise HTTPException(status_code=404, detail=f"Item '{request.item_id}' not found")

    prop = item.get_property(request.dimension)
    if prop is None:
        available = item.dimensions()
        raise HTTPException(
            status_code=400,
            detail=f"Item '{item.name}' has no '{request.dimension}' property. "
                   f"Available dimensions: {available}",
        )

    prompt = (
        f"Find a surprising comparison for {item.name}. "
        f"Its {prop.label} is {prop.value} {prop.unit}. "
        f"Use lookup_item and search_catalog to find a comparison target in the "
        f"'{request.dimension}' dimension from a very different category than "
        f"'{item.category}'. Then use convert_unit to compute the exact equivalence. "
        f"Present the result as a fun, memorable statement."
    )

    try:
        response = await llm_service.get_chat_response(message=prompt)
        return CompareResponse(
            message=response,
            source_item=request.item_id,
            dimension=request.dimension,
            conversation_history=[
                ChatMessage(role="user", content=prompt),
                ChatMessage(role="assistant", content=response),
            ],
        )
    except Exception as e:
        logger.error(f"Comparison failed: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail="Failed to generate comparison")

