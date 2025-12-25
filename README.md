# Crazy Converterator

A whimsical unit conversion tool powered by natural language processing. Ask questions like *"How many horses would it take to replace a nuclear power plant?"* and get step-by-step answers.

## What It Does

Crazy Converterator uses LLMs to understand conversion queries and real mathematical tools to compute accurate answers. It handles everything from simple unit conversions to creative comparisons across dimensions.

**Example queries:**
- "Convert 5 miles to kilometers"
- "How many bathtubs would fill an Olympic swimming pool?"
- "If a car travels at 60 mph, how long to reach the moon?"

## Quick Start

### Prerequisites

- Python 3.10+, [uv](https://github.com/astral-sh/uv), Node.js 18+, Rust
- OpenAI or Anthropic API key

### Setup

```bash
# Clone the repo
git clone https://github.com/yourusername/crazy-converter.git
cd crazy-converter

# Set up Python environment
uv venv && source .venv/bin/activate

# Build Rust module
cd rust && uv pip install maturin && maturin develop && cd ..

# Install backend
cd backend && uv pip install -r requirements.txt && cd ..

# Create backend/.env with your API key:
# LLM_PROVIDER=openai
# LLM_MODEL=gpt-4o-mini
# OPENAI_API_KEY=sk-your-key

# Install frontend
cd frontend && npm install && cd ..
```

### Run

**Terminal 1 (Backend):**
```bash
source .venv/bin/activate
cd backend && uvicorn main:app --reload --port 8000
```

**Terminal 2 (Frontend):**
```bash
cd frontend && npm run dev
```

Open `http://localhost:3000` and start converting!

## Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Frontend   │────▶│   Backend   │────▶│  LLM API    │
│  (Nuxt 3)   │     │  (FastAPI)  │     │ (OpenAI/    │
│             │     │      │      │     │  Anthropic) │
└─────────────┘     └──────┼──────┘     └─────────────┘
                           │
                    ┌──────▼──────┐
                    │ Rust Module │
                    │ (PyO3)      │
                    └─────────────┘
```

- **Frontend**: Nuxt 3 chat interface with Tailwind CSS
- **Backend**: FastAPI with Pydantic AI for LLM orchestration
- **Rust Module**: High-performance unit conversions via PyO3

## Documentation

| Document | Description |
|----------|-------------|
| [SETUP.md](SETUP.md) | Detailed local development setup |
| [docs/deployment.md](docs/deployment.md) | Production deployment guide |
| [docs/architecture.md](docs/architecture.md) | System architecture details |
| [docs/features.md](docs/features.md) | Supported conversion types |

## Deployment

For production deployment, we recommend Docker with a PaaS platform:

```bash
# Quick Docker deployment
docker-compose up --build
```

See [docs/deployment.md](docs/deployment.md) for:
- Docker configuration
- PaaS deployment (Render, Railway, Cloud Run)
- CI/CD setup with GitHub Actions
- Environment configuration

## Supported Conversions

14 unit categories with 100+ unit types:

| Category | Example Units |
|----------|---------------|
| Length | meters, feet, miles, kilometers |
| Mass | kilograms, pounds, grams, ounces |
| Time | seconds, hours, days, years |
| Temperature | Celsius, Fahrenheit, Kelvin |
| Speed | mph, km/h, m/s, knots |
| Energy | joules, calories, kWh, BTU |
| ... | See [features.md](docs/features.md) for full list |

## Development

```bash
# Run Rust tests
cd rust && cargo test

# Run Python integration tests
source .venv/bin/activate
cd backend && python test_conversions.py

# Check backend health
curl http://localhost:8000/health
```

## License

MIT

---

*Crazy Converterator is for fun. While conversions are mathematically accurate, creative comparisons are meant to inspire wonder, not replace engineering calculations.*
