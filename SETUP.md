# Local Development Setup

This guide walks you through setting up Crazy Converterator for local development.

> **Looking for production deployment?** See [docs/deployment.md](docs/deployment.md).

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Python | 3.10+ | Backend runtime |
| uv | Latest | Python package manager |
| Node.js | 18+ | Frontend runtime |
| npm | 9+ | Frontend package manager |
| Rust | Latest stable | Conversion library |
| Cargo | Latest | Rust package manager |

**Plus one of:**
- OpenAI API key, or
- Anthropic API key

## Quick Start

```bash
# 1. Clone and enter the project
git clone https://github.com/yourusername/crazy-converter.git
cd crazy-converter

# 2. Set up Python environment
uv venv
source .venv/bin/activate  # Windows: .venv\Scripts\activate

# 3. Build Rust module
cd rust
uv pip install maturin
maturin develop
cd ..

# 4. Set up backend
cd backend
uv pip install -r requirements.txt
# Create .env file (see Configuration section below)
cd ..

# 5. Set up frontend
cd frontend
npm install
cd ..
```

## Configuration

Create a `.env` file in the `backend/` directory:

```bash
# backend/.env
LLM_PROVIDER=openai
LLM_MODEL=gpt-4o-mini
OPENAI_API_KEY=sk-your-key-here
# Or for Anthropic:
# LLM_PROVIDER=anthropic
# LLM_MODEL=claude-3-5-sonnet-20241022
# ANTHROPIC_API_KEY=sk-ant-your-key-here
```

| Variable | Options | Description |
|----------|---------|-------------|
| `LLM_PROVIDER` | `openai`, `anthropic` | Which LLM provider to use |
| `LLM_MODEL` | See below | Model name |
| `OPENAI_API_KEY` | Your key | Required if using OpenAI |
| `ANTHROPIC_API_KEY` | Your key | Required if using Anthropic |

**Supported Models:**
- OpenAI: `gpt-4o-mini`, `gpt-4o`, `gpt-4-turbo`
- Anthropic: `claude-3-5-sonnet-20241022`, `claude-3-opus-20240229`

## Running the Application

You need **two terminal windows** - one for the backend, one for the frontend.

### Terminal 1: Backend

```bash
# Activate virtual environment (required in each new terminal!)
source .venv/bin/activate  # Windows: .venv\Scripts\activate

# Start the backend server
cd backend
uvicorn main:app --reload --port 8000
```

The backend will be available at: `http://localhost:8000`

### Terminal 2: Frontend

```bash
cd frontend
npm run dev
```

The frontend will be available at: `http://localhost:3000`

### Verify It's Working

1. **Check backend health:**
   ```bash
   curl http://localhost:8000/health
   ```
   
   You should see:
   ```json
   {"status": "healthy", "components": {"api": "ok", "rust_module": "ok"}}
   ```

2. **Open the app:** Navigate to `http://localhost:3000` in your browser

3. **Test a conversion:** Type "Convert 5 miles to kilometers" and press Enter

## Alternative: Docker Development

If you prefer Docker, you can run the full stack with:

```bash
# Set your API key
export OPENAI_API_KEY=sk-your-key-here

# Start everything
docker-compose up --build
```

This builds the Rust module inside Docker, so you don't need Rust installed locally.

## Testing

### Test Rust Conversions

```bash
# Run Rust unit tests
cd rust
cargo test
```

```bash
# Test Python integration
source .venv/bin/activate
cd backend
python test_conversions.py
```

### Test Backend API

```bash
# Health check
curl http://localhost:8000/health

# Test chat endpoint
curl -X POST http://localhost:8000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Convert 5 miles to kilometers", "conversation_history": []}'
```

## Troubleshooting

### "ModuleNotFoundError: No module named 'converterator_rust'"

The Rust module isn't installed. Fix:

```bash
source .venv/bin/activate  # Make sure venv is active!
cd rust
maturin develop
```

Then verify:
```bash
python -c "import converterator_rust; print('OK')"
```

### "(.venv) not showing in prompt"

Your virtual environment isn't activated. Run:
```bash
source .venv/bin/activate  # Windows: .venv\Scripts\activate
```

### API key errors

1. Check that `backend/.env` exists and has the correct key
2. Verify your API key is valid with your provider
3. Make sure `LLM_PROVIDER` matches your key type

### Port already in use

Change the port:
```bash
# Backend
uvicorn main:app --reload --port 8001

# Then update frontend to point to new backend
# Edit frontend/nuxt.config.ts or set API_BASE env var
API_BASE=http://localhost:8001 npm run dev
```

### CORS errors

If the frontend can't reach the backend, check:
1. Backend is running on the expected port
2. The `API_BASE` in `frontend/nuxt.config.ts` matches
3. No firewall blocking localhost connections

## Project Structure

```
crazy-converter/
├── backend/           # FastAPI backend
│   ├── main.py        # App entry point
│   ├── app/           # Application modules
│   │   ├── api/       # API routes
│   │   ├── services/  # Business logic
│   │   └── mcp/       # MCP integration
│   └── requirements.txt
├── frontend/          # Nuxt 3 frontend
│   ├── pages/         # Page components
│   ├── components/    # Vue components
│   └── composables/   # Vue composables
├── rust/              # Rust conversion library
│   └── src/
│       ├── lib.rs     # PyO3 bindings
│       └── conversions/  # Conversion modules
└── docs/              # Documentation
    ├── architecture.md
    ├── deployment.md  # Production deployment guide
    └── features.md
```

## Next Steps

- Read [docs/architecture.md](docs/architecture.md) to understand the system design
- Read [docs/deployment.md](docs/deployment.md) when you're ready to deploy
- Check [docs/features.md](docs/features.md) for supported conversions
