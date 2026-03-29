# Crazy Converterator: Technical Architecture

## System Overview

The Crazy Converterator is a web application that generates surprising, memorable
comparisons between real-world items across physical dimensions. A user picks an
item (a banana), picks a dimension (energy), and the system finds a comparison
target (nuclear decay, a lightning bolt, a Saturn V launch) and computes the exact
equivalence using verified data and precise unit conversion.

The system has four layers, a browser-based UI, and no persistent storage.

```
┌─────────────────────────────────────────────────────────────┐
│                        USER BROWSER                         │
│                                                             │
│  ┌───────────────────────────────────────────────────────┐  │
│  │              Nuxt 3 SPA (Vue 3 + Tailwind)            │  │
│  │                                                       │  │
│  │  ExplorationFlow    ChatInterface                     │  │
│  │  ┌──────────────┐   ┌──────────────┐                  │  │
│  │  │ ItemPicker    │   │ Message list │                  │  │
│  │  │ DimensionSel. │   │ Text input   │                  │  │
│  │  │ CompResult    │   │              │                  │  │
│  │  └──────────────┘   └──────────────┘                  │  │
│  │         │                    │                         │  │
│  │     useComparison        useChat                      │  │
│  │         │                    │                         │  │
│  │         └────────┬───────────┘                        │  │
│  │                  ▼                                    │  │
│  │         HTTP (fetch to backend)                       │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────────────┬────────────────────────────────┘
                             │ REST / JSON
                             ▼
┌─────────────────────────────────────────────────────────────┐
│                     BACKEND (Python)                        │
│                     FastAPI + Uvicorn                        │
│                                                             │
│  ┌─────────────────────────────────────────────────┐       │
│  │                  API Layer                       │       │
│  │  GET  /api/items          (catalog queries)      │       │
│  │  GET  /api/items/:id      (item detail)          │       │
│  │  GET  /api/items/categories                      │       │
│  │  GET  /api/items/dimensions                      │       │
│  │  POST /api/compare        (comparison request)   │       │
│  │  POST /api/chat           (freeform chat)        │       │
│  │  GET  /health                                    │       │
│  └──────────────┬──────────────────────┬───────────┘       │
│                 │                      │                    │
│     ┌───────────▼──────────┐  ┌───────▼─────────┐         │
│     │   Item Catalog       │  │   LLM Agent     │         │
│     │   (items.json)       │  │   (Pydantic AI) │         │
│     │                      │  │                 │         │
│     │   ItemCatalog class  │  │   System prompt │         │
│     │   load / query / search│ │   + Tools:      │         │
│     └──────────────────────┘  │   - lookup_item │         │
│                               │   - search_catalog│        │
│                               │   - convert_unit│         │
│                               │   - list_*      │         │
│                               └────┬──────┬─────┘         │
│                                    │      │                │
│                    ┌───────────────┘      └──────┐         │
│                    ▼                             ▼         │
│  ┌──────────────────────┐    ┌─────────────────────────┐  │
│  │  Rust Conversion     │    │  External LLM Provider  │  │
│  │  Module (PyO3)       │    │  (network call)         │  │
│  │                      │    │                         │  │
│  │  14 dimension modules│    │  Ollama / Groq /        │  │
│  │  converterator_rust  │    │  OpenAI / Anthropic     │  │
│  └──────────────────────┘    └─────────────────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```


## Layer 1: Conversion Math (Rust)

### Purpose

Pure unit-to-unit conversion within a single physical dimension. This is the
"truth engine" — when the system says 89 kcal = 372,376 joules, this layer
guarantees the math is exact.

### Technology

- **Language**: Rust (edition 2021)
- **Python binding**: PyO3 0.22 with `extension-module` feature
- **Build tool**: Maturin — produces a Python wheel from Rust source
- **Package name**: `converterator_rust`

### Structure

```
rust/
├── Cargo.toml                    # pyo3 = "0.22"
├── pyproject.toml                # maturin config
└── src/
    ├── lib.rs                    # PyO3 module definition, exports 14 functions
    └── conversions/
        ├── mod.rs                # re-exports all modules
        ├── time.rs               # seconds ↔ minutes ↔ hours ↔ days ↔ weeks ↔ months ↔ years
        ├── length.rs             # meters ↔ cm ↔ mm ↔ μm ↔ nm ↔ km ↔ in ↔ ft ↔ yd ↔ mi
        ├── area.rs               # m² ↔ cm² ↔ km² ↔ in² ↔ ft² ↔ yd² ↔ acres ↔ hectares
        ├── volume.rs             # liters ↔ ml ↔ m³ ↔ cm³ ↔ in³ ↔ ft³ ↔ gal ↔ fl oz ↔ cups ↔ pints ↔ quarts
        ├── mass.rs               # kg ↔ g ↔ mg ↔ lb ↔ oz ↔ tons ↔ stones
        ├── speed.rs              # m/s ↔ km/h ↔ mph ↔ ft/s ↔ knots
        ├── acceleration.rs       # m/s² ↔ ft/s² ↔ g
        ├── force.rs              # N ↔ lbf ↔ dyn ↔ kN
        ├── pressure.rs           # Pa ↔ kPa ↔ MPa ↔ psi ↔ bar ↔ atm ↔ torr ↔ mmHg
        ├── energy.rs             # J ↔ kJ ↔ MJ ↔ kWh ↔ cal ↔ kcal ↔ BTU ↔ ft·lb ↔ eV
        ├── power.rs              # W ↔ kW ↔ MW ↔ hp ↔ ft·lb/s
        ├── momentum.rs           # kg·m/s ↔ lb·ft/s ↔ g·cm/s
        ├── torque.rs             # N·m ↔ lb·ft ↔ kgf·m ↔ oz·in
        └── temperature.rs        # °C ↔ °F ↔ K ↔ °R (offset-aware)
```

### Conversion Pattern

Every module follows a two-step pattern through a base unit:

```
source unit ──▶ base unit ──▶ target unit
   (e.g. kcal)    (joules)    (kilowatt_hours)
```

This means N units require only 2N conversion factors, not N².
Temperature is special — it uses offsets (Celsius → Kelvin adds 273.15)
rather than pure multiplication.

### Python Interface

Each module exports one `#[pyfunction]`:

```python
from converterator_rust import convert_energy
result = convert_energy(89.0, "kcal", "joules")  # → 372376.0
```

The backend's `conversion_service.py` wraps all 14 functions into a single
`convert_unit(value, from_unit, to_unit, category)` tool that routes to the
correct Rust function by category name.

### Build

```
cd rust && maturin develop       # dev: installs into active venv
cd rust && maturin build         # prod: produces .whl file
```

The Docker build uses a multi-stage approach: a Rust builder stage compiles the
wheel, and the production Python stage installs it from the wheel file.


## Layer 2: Item Knowledge (Python, static data)

### Purpose

A curated catalog of real-world items with verified physical properties.
Properties are the bridge between "a banana" and "89 kcal" — they turn
familiar things into numbers the conversion engine can work with.

### Technology

- **Storage**: JSON file (`backend/app/data/items.json`), loaded into memory
  at process startup
- **Access**: `ItemCatalog` class in `backend/app/data/item_store.py`
- **Models**: Pydantic `BaseModel` classes (`Item`, `Property`)

### Data Model

```
Item
├── id: str                      "banana"
├── name: str                    "a banana"
├── category: str                "food"
└── properties: list[Property]
    ├── Property
    │   ├── dimension: str       "energy"        ← matches Rust category name
    │   ├── value: float         89.0
    │   ├── unit: str            "kcal"           ← matches Rust unit name
    │   ├── label: str           "caloric energy"
    │   └── qualifier: str?      null             ← disambiguates multiple
    │                                                properties in same dimension
    ├── Property
    │   ├── dimension: str       "mass"
    │   ├── value: float         118.0
    │   ├── unit: str            "grams"
    │   ├── label: str           "weight"
    │   └── qualifier: str?      null
    ...
```

An item can have multiple properties in the same dimension (a cow has body
weight AND monthly methane output, both in mass). The optional `qualifier`
field distinguishes them.

### Catalog Contents

The catalog contains ~45 items across 10 categories:

```
Category            Example items                            Count
─────────────────   ──────────────────────────────────────   ─────
food                banana, egg, pizza slice, chocolate bar      6
animal              cow, elephant, blue whale, ant, hummingbird  5
human_body          tooth, hair, brain, adult human, sneeze      6
vehicle             car, bicycle, Boeing 747, Saturn V, ship     5
building            Eiffel Tower, Great Pyramid, pool, house     4
astronomy           Sun, Moon, Earth, neutron star               5
subatomic           electron, proton, grain of sand              3
natural_phenomena   lightning bolt                               1
energy_reference    AA battery, dynamite, gasoline, Hiroshima    5
time_reference      heartbeat, blink, age of universe, Bolt      5
```

Items span extreme orders of magnitude within each dimension. This maximizes
surprise: comparing an electron's mass to the Sun's mass produces a ratio
around 10^57.

### Query Interface

```python
from app.data.item_store import catalog

catalog.get("banana")                    # → Item or None
catalog.all_items()                      # → list[Item]
catalog.by_category("food")             # → list[Item]
catalog.by_dimension("energy")          # → list[Item] that have energy properties
catalog.search("lightning")             # → text search over names and labels
catalog.categories()                    # → sorted list of category names
catalog.dimensions()                    # → sorted list of dimension names
```

### Data Integrity

All numbers in the catalog are verified against authoritative sources.
This matters: wrong numbers build wrong intuition, which is the opposite of
the product's purpose. The LLM can suggest items not in the catalog, but those
numbers are flagged as unverified/approximate.

### Persistence

There is no database. The catalog is a static JSON file, version-controlled
with the source code, loaded once at startup. This is intentional:

- ~50 items is trivially small — no query optimization needed
- Catalog changes are code changes, reviewed in PRs
- No migration, backup, or connection management overhead
- The catalog grows by editing the JSON file and redeploying


## Layer 3: Comparison Engine (Python + LLM)

### Purpose

The creative brain. Takes an item and dimension, finds a surprising comparison
target, computes the exact equivalence, and generates a narrative explanation.
This layer is where the LLM and the Rust converter collaborate.

### Technology

- **Framework**: Pydantic AI 0.4.x — an agent framework with typed tool calling
- **LLM providers**: Ollama (local), Groq (free cloud), OpenAI, Anthropic
- **Agent pattern**: The LLM is given tools and a system prompt. It decides
  which tools to call and in what order. Pydantic AI handles the tool-call
  loop automatically.

### Architecture

```
┌────────────────────────────────────────────────────────┐
│                    Pydantic AI Agent                    │
│                                                        │
│  System prompt: "You are the Crazy Converterator..."   │
│                                                        │
│  Registered tools:                                     │
│  ┌──────────────┐ ┌──────────────┐ ┌───────────────┐  │
│  │ lookup_item  │ │search_catalog│ │ convert_unit   │  │
│  │              │ │              │ │                │  │
│  │ Gets item +  │ │ Finds items  │ │ Rust converter │  │
│  │ properties   │ │ by dimension │ │ (14 dimensions)│  │
│  │ from catalog │ │ or category  │ │                │  │
│  └──────────────┘ └──────────────┘ └───────────────┘  │
│  ┌──────────────┐ ┌──────────────┐                    │
│  │list_categories│ │list_dimensions│                    │
│  └──────────────┘ └──────────────┘                    │
│                                                        │
│  The LLM calls these tools in whatever sequence it     │
│  decides. Pydantic AI manages the tool-call loop:      │
│  prompt → LLM → tool call → result → LLM → ...        │
│  until the LLM produces a final text response.         │
│                                                        │
└────────────────────────────────────────────────────────┘
```

### How the LLM Is Used

The LLM does NOT do math. It does three things:

1. **Selects comparison targets**: Given "banana, energy", the LLM searches
   the catalog for items in the energy dimension from a different category and
   picks the most surprising one (e.g., a lightning bolt, the Hiroshima bomb).

2. **Orchestrates tool calls**: The LLM calls `lookup_item` to get numbers,
   `search_catalog` to find candidates, and `convert_unit` to compute the
   exact equivalence. Pydantic AI passes the results back to the LLM
   automatically.

3. **Writes the narrative**: "The caloric energy of one banana equals the
   energy of 0.001 lightning bolts" — the punchline, the explanation, and the
   tone all come from the LLM.

### Tool Definitions

Tools are plain Python functions wrapped in `pydantic_ai.Tool()`:

```python
# Comparison tools (read from item catalog)
Tool(lookup_item)          # (item_id: str) → ItemDetail
Tool(search_catalog)       # (dimension?, category?, query?) → CatalogSearchResult
Tool(list_categories)      # () → list[str]
Tool(list_dimensions)      # () → list[str]

# Conversion tool (calls Rust)
Tool(convert_unit)         # (value, from_unit, to_unit, category) → ConversionResult
```

The LLM sees each tool's function signature and docstring. It decides when
and how to call them based on the user's request and the system prompt's
instructions.

### Request Flow for POST /api/compare

```
1. Client sends: { item_id: "banana", dimension: "energy" }
         │
         ▼
2. Backend looks up item in catalog, validates dimension exists
         │
         ▼
3. Backend constructs a prompt:
   "Find a surprising comparison for a banana.
    Its caloric energy is 89.0 kcal.
    Use lookup_item and search_catalog to find a comparison target
    in the 'energy' dimension from a very different category than 'food'.
    Then use convert_unit to compute the exact equivalence."
         │
         ▼
4. Pydantic AI sends prompt + system prompt + tool schemas to LLM
         │
         ▼
5. LLM responds with tool calls (e.g., search_catalog(dimension="energy"))
         │
         ▼
6. Pydantic AI executes tool, returns result to LLM
         │
         ▼
7. LLM calls more tools (e.g., convert_unit(89.0, "kcal", "joules", "energy"))
         │
         ▼
8. Repeat 5-7 until LLM produces a final text response
         │
         ▼
9. Backend returns: { message: "The caloric energy of...", source_item: "banana", ... }
```

### Request Flow for POST /api/chat

The chat endpoint passes the user's freeform message directly to the same
Pydantic AI agent. The LLM has access to all the same tools and can decide
how to handle the request — whether it's a comparison, a question about an
item, or general conversation.

### LLM Provider Configuration

```
┌──────────────────────────────────────────────────────────────────┐
│                     LLM Provider Selection                       │
│                                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐    │
│  │  Ollama  │  │   Groq   │  │  OpenAI  │  │  Anthropic   │    │
│  │  (local) │  │  (free)  │  │  (paid)  │  │   (paid)     │    │
│  ├──────────┤  ├──────────┤  ├──────────┤  ├──────────────┤    │
│  │ OpenAI-  │  │ OpenAI-  │  │ Native   │  │ Native       │    │
│  │ compat   │  │ compat   │  │ client   │  │ client       │    │
│  │ endpoint │  │ endpoint │  │          │  │              │    │
│  ├──────────┤  ├──────────┤  ├──────────┤  ├──────────────┤    │
│  │ No key   │  │ Free key │  │ Paid key │  │ Paid key     │    │
│  │ needed   │  │ required │  │ required │  │ required     │    │
│  └──────────┘  └──────────┘  └──────────┘  └──────────────┘    │
│                                                                  │
│  Selected by: LLM_PROVIDER env var (default: "ollama")           │
│  Model name:  LLM_MODEL env var (default: "qwen2.5-coder:7b")   │
│                                                                  │
│  All providers use the same Pydantic AI Agent interface.         │
│  Switching providers requires zero code changes.                 │
└──────────────────────────────────────────────────────────────────┘
```

Ollama and Groq both use Pydantic AI's `OpenAIModel` (they expose
OpenAI-compatible endpoints). OpenAI uses the same class natively.
Anthropic uses `AnthropicModel`.

### LLM Interaction Pattern

The system uses a **single-turn agentic pattern**:

- Each API request creates one `agent.run(prompt)` call
- The agent may make multiple tool calls within that single run
- Conversation history is passed from the frontend but is not currently
  threaded into the agent (the agent sees only the current prompt)
- There is no streaming — the entire response is returned at once

### System Prompt

The system prompt instructs the LLM to:
- Use catalog tools for verified numbers
- Prefer cross-domain comparisons (food vs. physics, animals vs. engineering)
- Prefer surprising ratios (billions or tiny fractions, not "3.2")
- Lead with the punchline, then explain the math
- Flag approximate numbers when using items outside the catalog
- Support multi-step chains across dimensions


## Layer 4: UI (Nuxt 3 SPA)

### Purpose

A guided exploration interface that teaches users what the system can do and
funnels them toward interesting comparisons. Not a generic chat box.

### Technology

- **Framework**: Nuxt 3.20 (Vue 3)
- **Styling**: Tailwind CSS 3.4
- **Rendering**: Client-side only (SSR disabled)
- **Build output**: Static files served by Nginx (production) or dev server

### Component Tree

```
pages/index.vue
├── Tab: "Explore" (primary)
│   └── ExplorationFlow.vue                  ← orchestrates the 3-step flow
│       ├── Step 0: ItemPicker.vue           ← searchable grid, category filter
│       ├── Step 1: DimensionSelector.vue    ← cards showing item's properties
│       └── Step 2: ComparisonResult.vue     ← comparison text + retry/reset
│
└── Tab: "Ask anything" (secondary)
    └── ChatInterface.vue                    ← freeform chat with LLM
```

### State Management

Two composables manage API communication and state:

```
composables/
├── useComparison.ts     ← ExplorationFlow state
│   ├── items            reactive list of ItemSummary
│   ├── categories       reactive list of strings
│   ├── selectedItem     reactive ItemDetail or null
│   ├── comparisonResult reactive ComparisonResult or null
│   ├── fetchItems()     GET /api/items
│   ├── fetchCategories()GET /api/items/categories
│   ├── selectItem()     GET /api/items/:id
│   ├── compare()        POST /api/compare
│   └── reset()          clear all state
│
└── useChat.ts           ← ChatInterface state
    ├── conversationHistory  reactive ChatMessage[]
    ├── isLoading            reactive boolean
    ├── sendMessage()        POST /api/chat
    └── error                reactive string or null
```

### Exploration Flow (Primary UX)

```
┌─────────────────────────────────────────────────┐
│  ● Pick an item    ○ Pick a dimension    ○ See  │  ← step indicators
│                                                 │
│  [food] [animal] [human_body] [vehicle] ...     │  ← category filter pills
│                                                 │
│  [Search items...                         ]     │  ← text search
│                                                 │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐       │
│  │ a banana │ │ a cow    │ │ an ant   │ ...    │  ← clickable item cards
│  │ 3 dims   │ │ 3 dims   │ │ 3 dims   │       │
│  └──────────┘ └──────────┘ └──────────┘       │
└─────────────────────────────────────────────────┘
                    │ click
                    ▼
┌─────────────────────────────────────────────────┐
│  ○ Pick an item    ● Pick a dimension    ○ See  │
│                                                 │
│  ← Pick a different item                        │
│                                                 │
│  a banana                                       │
│  Pick a dimension to compare                    │
│                                                 │
│  ┌─────────────────────┐ ┌──────────────────┐  │
│  │ caloric energy      │ │ weight           │  │
│  │ 89 kcal             │ │ 118 grams        │  │
│  │ energy              │ │ mass             │  │
│  └─────────────────────┘ └──────────────────┘  │
│  ┌─────────────────────┐                       │
│  │ length              │                       │
│  │ 20 centimeters      │                       │
│  │ length              │                       │
│  └─────────────────────┘                       │
└─────────────────────────────────────────────────┘
                    │ click
                    ▼
┌─────────────────────────────────────────────────┐
│  ○ Pick an item    ○ Pick a dimension    ● See  │
│                                                 │
│  ← Try a different dimension                    │
│                                                 │
│  ┌───────────────────────────────────────────┐  │
│  │ The caloric energy of one banana (89 kcal)│  │
│  │ is equivalent to the energy released by   │  │
│  │ 0.000089 sticks of dynamite...            │  │
│  │                                           │  │
│  │ [Try another comparison]  [Start over]    │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

### API Communication

All API calls use Nuxt's `$fetch` (built on `ofetch`). The backend URL
is configured via the `API_BASE` environment variable at build time,
defaulting to `http://localhost:8000`.

```
Frontend                         Backend
────────                         ───────
GET /api/items?category=food  →  returns ItemSummary[]       (no LLM)
GET /api/items/banana         →  returns ItemDetail          (no LLM)
POST /api/compare             →  returns ComparisonResult    (calls LLM)
POST /api/chat                →  returns ChatResponse        (calls LLM)
```

The catalog endpoints (GET) are pure data lookups — fast, no LLM cost.
The comparison and chat endpoints (POST) involve an LLM round-trip and
are slower (seconds, depending on provider).


## Data Persistence

**There is none.** This is a deliberate choice for the current stage.

- **Item catalog**: Static JSON file, loaded into memory. Changes are code
  changes committed to git.
- **Conversation history**: Held in the browser's Vue reactive state. Lost
  on page refresh. Sent to the backend per-request, but the backend does
  not store it.
- **User state**: None. No accounts, no sessions, no cookies (beyond what
  the browser adds by default).
- **Comparison results**: Not saved anywhere. Each comparison is generated
  fresh.

### Future persistence considerations

When persistence becomes necessary, the likely additions are:

| Need                     | Likely solution                        |
|--------------------------|----------------------------------------|
| Save favorite comparisons| localStorage in browser, or a DB       |
| Share comparisons by URL | URL-encoded parameters or short IDs    |
| Grow the item catalog    | Still JSON until > ~500 items           |
| User accounts            | Auth provider + user DB (not planned)  |
| Analytics / usage data   | Event stream to analytics service      |


## Deployment

### Development

```
┌─────────────────────────────────────────────────┐
│                 Developer machine                │
│                                                  │
│  Terminal 1: uv run uvicorn main:app --reload    │
│              (backend on :8000)                   │
│                                                  │
│  Terminal 2: npm run dev                         │
│              (frontend on :3000)                  │
│                                                  │
│  Prereqs:                                        │
│  - uv venv --python 3.13                         │
│  - cd rust && maturin develop                    │
│  - uv pip install -r backend/requirements.txt    │
│  - cd frontend && npm install                    │
│                                                  │
│  LLM: Ollama on localhost, or cloud provider     │
│       configured via .env                        │
└─────────────────────────────────────────────────┘
```

### Docker Compose (local full-stack)

```
docker-compose up --build

┌──────────────────────────────────────────────────────┐
│  docker network (bridge)                              │
│                                                      │
│  ┌─────────────────────┐  ┌────────────────────────┐ │
│  │  backend             │  │  frontend              │ │
│  │  FastAPI + Rust .whl │  │  Nuxt dev server       │ │
│  │  :8000               │  │  :3000                 │ │
│  │                      │  │  API_BASE=backend:8000 │ │
│  │  Multi-stage Docker: │  │                        │ │
│  │  1. Rust builder     │  │  Volume-mounted source │ │
│  │  2. Python runtime   │  │  for hot reload        │ │
│  └─────────────────────┘  └────────────────────────┘ │
│           │                                          │
│           │ host.docker.internal                     │
│           ▼                                          │
│  ┌─────────────────────┐                             │
│  │  Ollama (on host)   │                             │
│  │  :11434             │                             │
│  └─────────────────────┘                             │
└──────────────────────────────────────────────────────┘
```

### Production

```
┌──────────────────────┐     ┌──────────────────────────────┐
│  CDN / Static host   │     │  Container platform          │
│                      │     │  (Render / Railway /          │
│  Nuxt static build   │     │   Cloud Run / ECS)           │
│  served by Nginx     │     │                              │
│  (gzip, caching,     │     │  ┌────────────────────────┐  │
│   SPA routing)       │     │  │  Backend container     │  │
│                      │────▶│  │  Gunicorn + Uvicorn    │  │
│  Built with:         │ API │  │  4 workers             │  │
│  npm run generate    │     │  │  Rust .whl inside      │  │
│                      │     │  └──────────┬─────────────┘  │
└──────────────────────┘     │             │                │
                              └─────────────┼────────────────┘
                                            │ HTTPS
                                            ▼
                              ┌──────────────────────────────┐
                              │  LLM Provider API            │
                              │  (Groq / OpenAI / Anthropic) │
                              │                              │
                              │  Or: Ollama on a separate    │
                              │  server / GPU instance        │
                              └──────────────────────────────┘
```

### Production Stack

| Component    | Technology              | Serves                        |
|-------------|-------------------------|-------------------------------|
| Frontend    | Nuxt 3 static + Nginx   | HTML/JS/CSS to browser        |
| Backend     | FastAPI + Gunicorn      | REST API, tool orchestration  |
| Rust module | PyO3 wheel in container | Unit conversion math          |
| LLM         | External API            | Creative comparison selection |

The backend is stateless. It can be scaled horizontally with no coordination
between instances. Each request is self-contained.


## Dependency Map

```
┌─────────────────────────────────────────────────────────────┐
│                      RUNTIME DEPENDENCIES                    │
│                                                              │
│  Frontend (Node 18+)          Backend (Python 3.13)          │
│  ─────────────────            ──────────────────             │
│  nuxt 3.20                    fastapi 0.115                  │
│  @nuxtjs/tailwindcss 6.12     uvicorn 0.32                   │
│  tailwindcss 3.4               pydantic-ai 0.4                │
│  vue 3.x (via nuxt)           pydantic 2.9                   │
│                                python-dotenv 1.0              │
│                                pydantic-settings 2.5          │
│                                converterator_rust (Rust wheel)│
│                                                              │
│  BUILD DEPENDENCIES                                          │
│  ─────────────────                                           │
│  Rust toolchain (stable)       maturin 1.12                  │
│  pyo3 0.22                     uv (Python env manager)       │
│  npm / Node 18+                                              │
│                                                              │
│  EXTERNAL SERVICES                                           │
│  ─────────────────                                           │
│  LLM provider (one of):                                      │
│    - Ollama (local, free, requires GPU or CPU inference)      │
│    - Groq (cloud, free tier, API key required)               │
│    - OpenAI (cloud, paid, API key required)                  │
│    - Anthropic (cloud, paid, API key required)               │
└─────────────────────────────────────────────────────────────┘
```


## API Reference

### Catalog Endpoints (no LLM, fast)

```
GET /api/items
  Query params: ?category=food&dimension=energy
  Response: [{ id, name, category, dimensions: string[] }, ...]

GET /api/items/categories
  Response: ["animal", "astronomy", "building", ...]

GET /api/items/dimensions
  Response: ["acceleration", "area", "energy", ...]

GET /api/items/{item_id}
  Response: { id, name, category, properties: [{ dimension, value, unit, label }, ...] }
  Error 404: item not found
```

### Comparison Endpoint (calls LLM, slow)

```
POST /api/compare
  Body: { item_id: "banana", dimension: "energy" }
  Response: { message: "The caloric energy of...", source_item: "banana", dimension: "energy", conversation_history: [...] }
  Error 404: item not found
  Error 400: item has no property in requested dimension
  Error 500: LLM failure
```

### Chat Endpoint (calls LLM, slow)

```
POST /api/chat
  Body: { message: "Compare a banana to a nuclear bomb", conversation_history: [...] }
  Response: { message: "...", conversation_history: [...] }
  Error 400: empty message
  Error 500: LLM failure
```

### Health

```
GET /health
  Response: { status: "healthy"|"degraded", components: { api, rust_module, llm_provider, llm_model } }
```


## Environment Variables

| Variable           | Required | Default                            | Purpose                      |
|-------------------|----------|------------------------------------|------------------------------|
| `LLM_PROVIDER`    | No       | `ollama`                           | LLM provider selection       |
| `LLM_MODEL`       | No       | `qwen2.5-coder:7b`                | Model name                   |
| `OLLAMA_BASE_URL`  | No       | `http://localhost:11434/v1`        | Ollama server URL            |
| `GROQ_API_KEY`     | If Groq  | —                                  | Groq API key                 |
| `OPENAI_API_KEY`   | If OpenAI| —                                  | OpenAI API key               |
| `ANTHROPIC_API_KEY`| If Anthropic| —                               | Anthropic API key            |
| `API_BASE`         | No       | `http://localhost:8000`            | Frontend → backend URL       |


## Error Handling

### Backend

- **Missing Rust module**: `conversion_service.py` catches `ImportError` at
  startup and sets `RUST_MODULE_AVAILABLE = False`. The health endpoint
  reports "degraded" status. Conversion tool calls raise `ImportError` with
  instructions to run `maturin develop`.

- **Missing LLM provider**: `get_model()` raises `ValueError` at startup if
  a required API key is missing. The process fails to start — this is
  intentional (fail fast).

- **LLM call failures**: `get_chat_response()` catches all exceptions and
  wraps them in `RuntimeError`. The API layer returns HTTP 500.

- **Invalid items/dimensions**: API routes return HTTP 400 or 404 with
  descriptive error messages.

### Frontend

- **Network errors**: Detected by `$fetch` — displayed inline in the UI.
- **API errors**: HTTP status codes mapped to user-friendly messages.
- **Empty states**: Shown when search/filter produces no results.


## Security

- **CORS**: Currently allows all origins (`*`). Must be restricted to the
  frontend domain in production.
- **API keys**: Stored in environment variables, loaded via `python-dotenv`.
  Never committed to source control (`.env` contains placeholders).
- **Input validation**: Pydantic models validate all API inputs. Item IDs are
  looked up in a fixed catalog (no injection surface). The only free-text
  input is the chat message, which is passed to the LLM (not to a database
  or shell).
- **LLM prompt injection**: The system prompt is not secret. User messages
  are passed as user messages (not system messages). The LLM's tools can
  only read catalog data and call the Rust converter — no write operations,
  no filesystem access, no network calls beyond the LLM's own API.


## What Is Not Built Yet

| Feature                    | Status      | Notes                                          |
|---------------------------|-------------|-------------------------------------------------|
| Multi-step chains          | Designed    | A→B→C across dimensions; needs chain UI + endpoint |
| "Surprise Me" button       | Designed    | Random item + dimension; trivial to add         |
| Streaming responses        | Not started | Would improve perceived latency for LLM calls   |
| Conversation threading     | Not started | Agent currently sees only current prompt         |
| Featured comparisons       | Not started | Pre-computed, curated set for the landing page   |
| Sharing / bookmarking      | Not started | URL-encoded comparison state or short IDs        |
| Item catalog expansion     | Ongoing     | ~45 items now; target ~200+ for good coverage    |
| Visual scale representations| Not started | e.g., stacking banana icons                     |
| Animated transitions       | Not started | Between exploration flow steps                   |
| Rate limiting              | Not started | Needed before public deployment                  |
| CI/CD pipeline             | Not started | GitHub Actions for build + deploy                |
