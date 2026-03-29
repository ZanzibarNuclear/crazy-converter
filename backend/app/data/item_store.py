"""
Item catalog: loads and queries the seed catalog of real-world items with verified properties.
"""
import json
from pathlib import Path
from typing import Optional
from pydantic import BaseModel


class Property(BaseModel):
    dimension: str
    value: float
    unit: str
    label: str
    qualifier: Optional[str] = None


class Item(BaseModel):
    id: str
    name: str
    category: str
    properties: list[Property]

    def get_property(self, dimension: str, qualifier: Optional[str] = None) -> Optional[Property]:
        """Get a specific property by dimension, optionally filtered by qualifier."""
        for p in self.properties:
            if p.dimension == dimension:
                if qualifier is None or p.qualifier == qualifier:
                    return p
        return None

    def dimensions(self) -> list[str]:
        """List all dimensions this item has properties for."""
        return list({p.dimension for p in self.properties})


class ItemCatalog:
    def __init__(self):
        self._items: dict[str, Item] = {}
        self._load()

    def _load(self):
        catalog_path = Path(__file__).parent / "items.json"
        with open(catalog_path) as f:
            data = json.load(f)
        for raw in data["items"]:
            item = Item(**raw)
            self._items[item.id] = item

    def get(self, item_id: str) -> Optional[Item]:
        return self._items.get(item_id)

    def all_items(self) -> list[Item]:
        return list(self._items.values())

    def by_category(self, category: str) -> list[Item]:
        return [i for i in self._items.values() if i.category == category]

    def by_dimension(self, dimension: str) -> list[Item]:
        return [i for i in self._items.values()
                if any(p.dimension == dimension for p in i.properties)]

    def search(self, query: str) -> list[Item]:
        """Simple text search over item names and property labels."""
        q = query.lower()
        results = []
        for item in self._items.values():
            if q in item.id.lower() or q in item.name.lower():
                results.append(item)
                continue
            for p in item.properties:
                if q in p.label.lower():
                    results.append(item)
                    break
        return results

    def categories(self) -> list[str]:
        return sorted({i.category for i in self._items.values()})

    def dimensions(self) -> list[str]:
        dims = set()
        for item in self._items.values():
            for p in item.properties:
                dims.add(p.dimension)
        return sorted(dims)


# Singleton instance
catalog = ItemCatalog()
