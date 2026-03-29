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

**Plus an LLM provider (choose one):**
- **Ollama** (recommended) - Free, local, private
- **Groq** - Free cloud API with generous limits
- OpenAI or Anthropic - Paid APIs

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
# Create .env file (see LLM Configuration section below)
cd ..

# 5. Set up frontend
cd frontend
npm install
cd ..
```

## LLM Configuration

Crazy Converterator supports multiple LLM providers. **We recommend Ollama for free, local inference.**

### Option 1: Ollama (Recommended - Free & Local)

Ollama runs models locally on your machine. No API keys, no costs, complete privacy.

**1. Install Ollama:**
```bash
# macOS/Linux
curl -fsSL https://ollama.com/install.sh | sh

# Or download from https://ollama.com/download
```

**2. Pull a coding model:**
```bash
# Recommended: Qwen 2.5 Coder (excellent for code and reasoning)
ollama pull qwen2.5-coder:7b

# Alternatives:
# ollama pull deepseek-coder-v2:16b    # Great for complex code
# ollama pull codellama:7b              # Meta's coding model
# ollama pull llama3.2:3b               # Smaller, faster
```

**3. Start Ollama (if not running):**
```bash
ollama serve
```

**4. Create backend/.env:**
```bash
LLM_PROVIDER=ollama
LLM_MODEL=qwen2.5-coder:7b
# Optional: custom Ollama URL (default: http://localhost:11434/v1)
# OLLAMA_BASE_URL=http://localhost:11434/v1
```

### Option 2: Groq (Free Cloud API)

Groq offers free API access with generous rate limits and fast inference.

**1. Get a free API key:**
- Sign up at [console.groq.com](https://console.groq.com)
- Create an API key (instant, no credit card required)

**2. Create backend/.env:**
```bash
LLM_PROVIDER=groq
LLM_MODEL=llama-3.3-70b-versatile
GROQ_API_KEY=gsk_your_key_here
```

**Recommended Groq models:**
| Model | Best For |
|-------|----------|
| `llama-3.3-70b-versatile` | Best quality, general purpose |
| `llama-3.1-8b-instant` | Faster, good for simple queries |
| `mixtral-8x7b-32768` | Good reasoning, large context |

### Option 3: OpenAI (Paid)

```bash
LLM_PROVIDER=openai
LLM_MODEL=gpt-4o-mini
OPENAI_API_KEY=sk-your-key-here
```

### Option 4: Anthropic (Paid)

```bash
LLM_PROVIDER=anthropic
LLM_MODEL=claude-3-5-sonnet-20241022
ANTHROPIC_API_KEY=sk-ant-your-key-here
```

## Configuration Reference

| Variable | Options | Description |
|----------|---------|-------------|
| `LLM_PROVIDER` | `ollama`, `groq`, `openai`, `anthropic` | Which LLM provider to use |
| `LLM_MODEL` | See above | Model name for the provider |
| `OLLAMA_BASE_URL` | URL | Ollama server URL (default: `http://localhost:11434/v1`) |
| `GROQ_API_KEY` | Your key | Required if using Groq |
| `OPENAI_API_KEY` | Your key | Required if using OpenAI |
| `ANTHROPIC_API_KEY` | Your key | Required if using Anthropic |

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
   {
     "status": "healthy",
     "components": {
       "api": "ok",
       "rust_module": "ok",
       "llm_provider": "ollama",
       "llm_model": "qwen2.5-coder:7b",
       "ollama_url": "http://localhost:11434/v1"
     }
   }
   ```

2. **Open the app:** Navigate to `http://localhost:3000` in your browser

3. **Test a conversion:** Type "Convert 5 miles to kilometers" and press Enter

## Alternative: Docker Development

If you prefer Docker, you can run the full stack with:

```bash
# For Groq (cloud API)
export GROQ_API_KEY=gsk-your-key-here
docker-compose up --build

# For Ollama, run Ollama on host and configure:
export LLM_PROVIDER=ollama
export OLLAMA_BASE_URL=http://host.docker.internal:11434/v1
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

### Ollama connection errors

1. Make sure Ollama is running: `ollama serve`
2. Check if the model is downloaded: `ollama list`
3. Pull the model if needed: `ollama pull qwen2.5-coder:7b`
4. Test Ollama directly: `ollama run qwen2.5-coder:7b "Hello"`

### Groq API errors

1. Verify your API key is correct
2. Check your rate limits at [console.groq.com](https://console.groq.com)
3. Try a different model if one is overloaded

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
│   │   ├── services/  # Business logic (LLM, conversions)
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
