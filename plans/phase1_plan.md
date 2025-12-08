# Phase 1 Foundation Plan for Crazy Converterator

## Architecture Decisions

### 1. Project Structure

Recommended modular structure that separates concerns and allows for clean integration:

```
converterator/
├── plans/
│   └── phase1_plan.md (this file)
├── rust/
│   ├── Cargo.toml
│   ├── pyproject.toml (for maturin)
│   └── src/
│       ├── lib.rs (Python bindings)
│       └── conversions/
│           ├── mod.rs
│           ├── length.rs
│           ├── weight.rs
│           ├── volume.rs
│           ├── energy.rs
│           └── time.rs
├── backend/
│   ├── main.py (FastAPI app)
│   ├── requirements.txt
│   ├── app/
│   │   ├── __init__.py
│   │   ├── api/
│   │   │   ├── __init__.py
│   │   │   └── routes.py (chat endpoints)
│   │   ├── services/
│   │   │   ├── __init__.py
│   │   │   ├── llm_service.py (Pydantic AI integration)
│   │   │   └── conversion_service.py (wrapper for Rust tools)
│   │   └── mcp/
│   │       ├── __init__.py
│   │       └── physics_client.py (Physics MCP integration)
├── frontend/
│   ├── index.html
│   ├── static/
│   │   └── app.js
│   └── package.json (if using build tools)
├── tests/
│   ├── test_rust_conversions.py
│   └── test_integration.py
├── README.md
└── .gitignore
```

**Rationale**:

- Separates Rust, Python backend, and frontend code
- Allows Rust crate to be built independently and imported as Python module
- MCP integration isolated in dedicated module
- Tests at multiple levels (unit + integration)

### 2. Rust-Python Integration: PyO3 + Maturin

**Approach**: Use PyO3 for Rust bindings and Maturin for building Python wheels.

**Why PyO3 + Maturin**:

- Industry standard for high-performance Rust-Python interop
- Maturin handles build automation and Python package generation
- Supports both development (pip install -e) and distribution workflows
- Minimal overhead for function calls

**Implementation**:

1. Create Rust library crate with PyO3 bindings
2. Expose conversion functions via `#[pyfunction]` macros
3. Build Python extension module using `maturin develop` (dev) or `maturin build` (distribution)
4. Import in Python: `from converterator_rust import convert_length, convert_weight, ...`

**Example structure**:

- Rust functions perform the actual conversions (pure, efficient)
- Python wrapper provides Pydantic AI tool interface
- FastAPI backend calls Python wrapper

### 3. Physics MCP Integration

**Recommendation: Yes, incorporate Physics MCP**

**Rationale**:

- Provides Computer Algebra System (CAS) for symbolic math beyond basic unit conversions
- Natural language interface complements the conversational UX
- Advanced plotting capabilities for visualization (future Phase 2)
- Handles complex physics calculations that may be needed for energy/power conversions
- Complements Rust conversion tool (Rust for speed on simple conversions, MCP for complex math)

**Integration approach**:

- Use `mcp` Python client library to connect to Physics MCP server
- Wrap MCP tools as Pydantic AI tools alongside Rust conversion tools
- LLM can choose between Rust (fast, simple) and Physics MCP (complex, symbolic) tools

**Setup**:

- Install Physics MCP server per its documentation
- Use MCP client library in Python backend
- Register Physics MCP tools with Pydantic AI agent

### 4. Phase 1 Implementation Steps

#### Step 1: Rust Conversion Tool (Basic Units)

**Goal**: Create Rust library with basic unit conversions

**Tasks**:

1. Initialize Rust crate: `cargo new --lib rust`
2. Add dependencies to `Cargo.toml`:

   - `pyo3 = { version = "0.22", features = ["extension-module"] }`
   - Consider `uom` crate for unit types (optional, start simple)

3. Implement conversion functions in `src/conversions/`:

   - **Time**: seconds, minutes, hours, days, weeks, months, years
   - **Length**: meters, feet, inches, yards, miles, kilometers, centimeters, millimeters, nanometers, micrometers
   - **Area**: square meters, square feet, square inches, square miles, square kilometers, acres, hectares
   - **Volume**: liters, milliliters, cubic meters, cubic feet, cubic inches, gallons (US), gallons (UK), fluid ounces, cups, pints, quarts
   - **Mass**: kilograms, grams, milligrams, pounds, ounces, tons (metric), tons (US), stone
   - **Speed**: meters per second, kilometers per hour, miles per hour, feet per second, knots
   - **Acceleration**: meters per second squared, feet per second squared, standard gravity (g)
   - **Force**: newtons, pounds-force, kilogram-force, dynes
   - **Pressure**: pascals, kilopascals, megapascals, pounds per square inch (PSI), bars, atmospheres, torr, millimeters of mercury
   - **Energy**: joules, kilojoules, megajoules, kilowatt-hours, calories, kilocalories, BTUs, foot-pounds, electronvolts
   - **Power**: watts, kilowatts, megawatts, horsepower (mechanical), horsepower (metric), BTUs per hour, foot-pounds per second
   - **Momentum**: kilogram-meters per second, pound-feet per second, newton-seconds
   - **Torque**: newton-meters, pound-feet, pound-inches, kilogram-force-meters

   Note: Start with most common units per category, can expand later. Focus on SI units and common imperial/US customary units.

4. Create PyO3 bindings in `src/lib.rs`:

   - Export each conversion function with `#[pyfunction]`
   - Create `#[pymodule]` to register functions

5. Add `pyproject.toml` for maturin configuration
6. Build and test: `maturin develop`

**Deliverable**: Python-importable module `converterator_rust` with functions like `convert_length(value, from_unit, to_unit) -> float`

#### Step 2: FastAPI + Pydantic AI Backend

**Goal**: Set up web API with LLM integration

**Tasks**:

1. Create `backend/` directory structure
2. Set up `requirements.txt`:

   - `fastapi`
   - `uvicorn`
   - `pydantic-ai`
   - `python-dotenv` (for API keys)
   - `mcp` (MCP client library)
   - Local Rust module (via maturin)

3. Create `backend/main.py`:

   - FastAPI app initialization
   - CORS middleware for frontend
   - Health check endpoint

4. Create `backend/app/services/llm_service.py`:

   - Initialize Pydantic AI model (start with OpenAI or Anthropic)
   - Define system prompt for conversion agent
   - Create tool registry for conversion functions

5. Create `backend/app/api/routes.py`:

   - POST `/api/chat` endpoint for conversation
   - Request/response models for chat messages

6. Test API with simple curl/requests

**Deliverable**: Running FastAPI server that accepts chat messages and returns LLM responses

#### Step 3: Connect Rust Tool to Pydantic AI

**Goal**: Make Rust conversions available as LLM tools

**Tasks**:

1. Create `backend/app/services/conversion_service.py`:

   - Import Rust conversion functions
   - Create Python wrapper functions that match Pydantic AI tool signature
   - Add error handling and validation

2. Register conversion tools with Pydantic AI:

   - Use `@tool` decorator or `Tool` class
   - Provide clear descriptions for LLM tool selection
   - Example: `convert_unit(value: float, from_unit: str, to_unit: str, category: str) -> dict`

3. Update `llm_service.py`:

   - Add conversion tools to agent's tool list
   - Test tool calling with sample queries

4. Integration test:

   - Send query: "Convert 5 miles to kilometers"
   - Verify LLM calls Rust tool correctly
   - Verify response includes conversion result

**Deliverable**: LLM can call Rust conversion functions to answer user queries

#### Step 4: Nuxt.js Web Interface

**Goal**: Modern chat UI using Nuxt.js

**Decision**: Use Nuxt.js (Vue) for frontend framework

**Rationale**:

- Modern, developer-friendly framework with great DX
- Built-in SSR/SSG capabilities
- Excellent tooling and ecosystem
- Vue's reactive system works well for chat interfaces
- Can use Nuxt UI or Tailwind CSS for styling

**Tasks**:

1. Initialize Nuxt.js project: `npx nuxi@latest init frontend`
2. Set up project structure:

   - `frontend/pages/index.vue` (main chat page)
   - `frontend/components/ChatInterface.vue` (chat UI component)
   - `frontend/composables/useChat.ts` (API integration composable)

3. Configure API integration:

   - Create composable for POST to `/api/chat`
   - Handle async responses and error states
   - Support streaming if Pydantic AI provides it (optional for Phase 1)

4. Build chat UI components:

   - Message list with user/assistant distinction
   - Input form with send button
   - Loading states during LLM processing

5. Styling:

   - Use Tailwind CSS or Nuxt UI for styling
   - Make it responsive and visually appealing

6. Configure development:

   - Set up proxy to FastAPI backend (or use separate ports)
   - Test end-to-end: open browser, ask conversion question, verify response

**Deliverable**: Modern Nuxt.js web interface where users can chat and get conversions

#### Step 5: Physics MCP Integration (Optional for Phase 1, Recommended)

**Goal**: Add Physics MCP as additional tool source

**Tasks**:

1. Install and configure Physics MCP server (per its docs)
2. Create `backend/app/mcp/physics_client.py`:

   - Initialize MCP client connection
   - Discover available Physics MCP tools
   - Create wrapper functions for Pydantic AI

3. Register Physics MCP tools alongside Rust tools
4. Test with complex query that benefits from CAS

**Deliverable**: LLM can use Physics MCP for advanced calculations when needed

## Success Criteria for Phase 1

- [ ] User can ask "Convert 5 miles to kilometers" via web interface
- [ ] LLM calls Rust conversion tool correctly
- [ ] Response includes accurate conversion result
- [ ] All basic unit categories work (length, weight, volume, time, energy)
- [ ] FastAPI server runs reliably
- [ ] Frontend displays conversation clearly

### 5. Frontend Framework: Nuxt.js

**Decision**: Use Nuxt.js (Vue) for frontend framework

**Rationale**:

- Modern, developer-friendly framework with great DX
- Built-in SSR/SSG capabilities
- Excellent tooling and ecosystem
- Vue's reactive system works well for chat interfaces
- Can use Nuxt UI or Tailwind CSS for styling

### 6. Command-Line Interface (Future Phase)

**Recommendation**: Use Python CLI libraries for command-line interface

**Options for CLI implementation**:

1. **Typer** (Recommended):

   - Modern, type-hint based CLI framework
   - Built on Click but with better type checking
   - Excellent for async operations
   - Great developer experience

2. **Click**:

   - Mature, widely-used library
   - Decorator-based API
   - Extensive ecosystem

3. **Rich + argparse**:

   - Use Python's built-in `argparse` for argument parsing
   - Use `rich` library (already in project) for beautiful terminal output
   - Lightweight approach

**Implementation approach**:

- Create `backend/app/cli/` module with CLI commands
- Reuse same services (LLM service, conversion service) from web API
- Use Rich for formatted output (progress bars, tables, syntax highlighting)
- Command structure: `converterator chat "Convert 5 miles to kilometers"`

**Example CLI structure**:

```
backend/app/cli/
├── __init__.py
├── main.py (entry point using Typer)
└── commands/
    ├── __init__.py
    └── chat.py (chat command implementation)
```

### 7. CI/CD Pipeline

**Decision**: Use GitHub Actions with cloud provider integration

**Considerations for SkyPilot**:

SkyPilot is primarily designed for:

- AI/ML batch jobs and training workloads
- Ephemeral compute tasks
- Multi-cloud cost optimization
- Research and experimentation

**For this web application**, SkyPilot may be:

- **Less ideal** because we need:
  - Always-on web service (not batch jobs)
  - Continuous deployment of long-running services
  - Standard web app infrastructure (load balancers, domains, etc.)
- **Could work** if:
  - Deploying as containerized service
  - Using SkyPilot's job management for the service
  - Want multi-cloud flexibility from day one

**Recommended Approach**: Traditional container-based deployment

**CI/CD Strategy**:

#### Option A: Container-Based Deployment (Recommended)

**Architecture**:

- Build Docker container with Rust + Python dependencies
- Push to container registry (GitHub Container Registry, Docker Hub, or cloud provider registry)
- Deploy container to cloud service (Cloud Run, ECS, App Service, Railway, Render)

**GitHub Actions Workflow**:

1. **CI Phase** (on every PR/push):

   - Install Rust toolchain, build Rust crate
   - Test Rust code (cargo test)
   - Install Python dependencies, test Python code
   - Build Rust-Python extension using maturin
   - Run integration tests
   - Lint and format checks

2. **CD Phase** (on merge to main):

   - Build multi-stage Docker image:
     - Stage 1: Build Rust extension
     - Stage 2: Python runtime + dependencies + built extension
   - Push to container registry
   - Deploy to cloud provider (using OIDC for authentication)
   - Optionally deploy Nuxt.js frontend separately (static hosting or SSR)

**Project Structure Addition**:

```
converterator/
├── .github/
│   └── workflows/
│       ├── ci.yml (continuous integration)
│       └── deploy.yml (continuous deployment)
├── Dockerfile (multi-stage build)
├── docker-compose.yml (local development)
└── .dockerignore
```

**Cloud Provider Options**:

1. **Railway/Render** (Simplest):

   - Automatic deployments from GitHub
   - Built-in container registry
   - Easy environment variable management
   - Good for MVP and small teams

2. **AWS (ECS/Fargate or Lambda)**:

   - Use GitHub Actions with OIDC
   - Push to ECR (Elastic Container Registry)
   - Deploy to ECS or Lambda (if using Lambda, need to handle Rust extension differently)

3. **Google Cloud Run**:

   - Serverless container platform
   - Automatic scaling
   - Good cost model for low-traffic apps
   - GitHub Actions with OIDC integration

4. **Azure Container Instances or App Service**:

   - Similar to GCP Cloud Run
   - GitHub Actions with OIDC

#### Option B: SkyPilot Deployment (If Multi-Cloud Needed)

**When to consider**:

- Need to run across multiple clouds simultaneously
- Want cost optimization across providers
- Need to handle spot/preemptible instances
- Have variable workload patterns

**Implementation**:

- Define SkyPilot task YAML for service deployment
- GitHub Actions triggers SkyPilot API
- SkyPilot manages deployment across chosen clouds
- Requires SkyPilot API server setup

**Recommendation for Phase 1**: Use Option A (container-based) with Railway, Render, or Cloud Run. Simpler, more standard for web apps, easier to debug. SkyPilot can be considered later if multi-cloud becomes a requirement.

### 8. Project Structure (Updated with CI/CD)

```
converterator/
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── deploy.yml
├── plans/
│   └── phase1_plan.md
├── rust/
│   ├── Cargo.toml
│   ├── pyproject.toml
│   └── src/
│       ├── lib.rs
│       └── conversions/
├── backend/
│   ├── Dockerfile
│   ├── main.py
│   ├── requirements.txt
│   ├── app/
│   │   ├── api/
│   │   ├── services/
│   │   ├── mcp/
│   │   └── cli/ (future)
├── frontend/
│   └── (Nuxt.js project)
├── tests/
├── README.md
├── .gitignore
└── docker-compose.yml
```

## Dependencies to Research

- Pydantic AI tool registration patterns
- MCP client library Python API
- Physics MCP server setup requirements
- Maturin development workflow best practices
- Docker multi-stage builds with Rust + Python
- GitHub Actions OIDC setup for chosen cloud provider
