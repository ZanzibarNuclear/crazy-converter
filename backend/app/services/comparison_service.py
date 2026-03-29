"""
Comparison service: Pydantic AI tools for finding surprising item-to-item comparisons.

These tools let the LLM look up items, search the catalog, and construct
comparisons using the Rust conversion engine for exact math.
"""
from typing import Optional
from pydantic import BaseModel
from pydantic_ai import Tool

from app.data.item_store import catalog


class ItemSummary(BaseModel):
    id: str
    name: str
    category: str
    dimensions: list[str]


class ItemDetail(BaseModel):
    id: str
    name: str
    category: str
    properties: list[dict]


class CatalogSearchResult(BaseModel):
    items: list[ItemSummary]
    total: int


def lookup_item(item_id: str) -> ItemDetail:
    """
    Look up an item in the catalog by ID and return its properties.

    Args:
        item_id: The item identifier (e.g., "banana", "cow", "lightning_bolt")

    Returns:
        The item with all its measured properties (value, unit, dimension, label).
        Use these verified numbers for accurate comparisons.
    """
    item = catalog.get(item_id)
    if item is None:
        raise ValueError(
            f"Item '{item_id}' not found. Use search_catalog to find available items."
        )
    return ItemDetail(
        id=item.id,
        name=item.name,
        category=item.category,
        properties=[p.model_dump() for p in item.properties],
    )


def search_catalog(
    dimension: Optional[str] = None,
    category: Optional[str] = None,
    query: Optional[str] = None,
) -> CatalogSearchResult:
    """
    Search the item catalog. Filter by dimension, category, or text query.

    Args:
        dimension: Filter to items with properties in this dimension (e.g., "energy", "mass", "length")
        category: Filter to items in this category (e.g., "food", "animal", "astronomy")
        query: Text search over item names and property labels

    Returns:
        Matching items with their IDs, names, categories, and available dimensions.
    """
    if query:
        items = catalog.search(query)
    elif dimension and category:
        by_dim = set(i.id for i in catalog.by_dimension(dimension))
        items = [i for i in catalog.by_category(category) if i.id in by_dim]
    elif dimension:
        items = catalog.by_dimension(dimension)
    elif category:
        items = catalog.by_category(category)
    else:
        items = catalog.all_items()

    summaries = [
        ItemSummary(
            id=i.id,
            name=i.name,
            category=i.category,
            dimensions=i.dimensions(),
        )
        for i in items
    ]
    return CatalogSearchResult(items=summaries, total=len(summaries))


def list_categories() -> list[str]:
    """
    List all item categories in the catalog.

    Returns:
        Category names like "food", "animal", "astronomy", "human_body", etc.
    """
    return catalog.categories()


def list_dimensions() -> list[str]:
    """
    List all physical dimensions available across catalog items.

    Returns:
        Dimension names like "energy", "mass", "length", "volume", etc.
    """
    return catalog.dimensions()


comparison_tools = [
    Tool(lookup_item),
    Tool(search_catalog),
    Tool(list_categories),
    Tool(list_dimensions),
]
