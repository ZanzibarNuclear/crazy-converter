# Setup Guide for Crazy Converterator

This guide will help you set up and run the Crazy Converterator project.

## Prerequisites

- Python 3.8+ with pip
- Node.js 18+ and npm
- Rust (for building the conversion library)
- Cargo (Rust package manager)
- An OpenAI or Anthropic API key

## Setup Steps

### 1. Build the Rust Conversion Library

```bash
cd rust
pip install maturin
maturin develop
cd ..
```

This will build the Rust extension module and make it available to Python.

### 2. Set Up the Backend

```bash
cd backend
pip install -r requirements.txt

# Create a .env file with your API keys
cp .env.example .env
# Edit .env and add your OPENAI_API_KEY or ANTHROPIC_API_KEY
cd ..
```

### 3. Set Up the Frontend

```bash
cd frontend
npm install
cd ..
```

### 4. Run the Application

**Terminal 1 - Backend:**
```bash
cd backend
uvicorn main:app --reload --port 8000
```

**Terminal 2 - Frontend:**
```bash
cd frontend
npm run dev
```

### 5. Access the Application

Open your browser and navigate to `http://localhost:3000` (or the port shown by Nuxt).

## Configuration

### Environment Variables (backend/.env)

- `LLM_PROVIDER`: Either "openai" or "anthropic" (default: "openai")
- `LLM_MODEL`: Model name (e.g., "gpt-4o-mini", "claude-3-5-sonnet-20241022")
- `OPENAI_API_KEY`: Your OpenAI API key (if using OpenAI)
- `ANTHROPIC_API_KEY`: Your Anthropic API key (if using Anthropic)

### Frontend Configuration

The frontend API base URL can be configured in `frontend/nuxt.config.ts` or via the `API_BASE` environment variable.

## Testing

### Test Rust Conversions

```bash
cd rust
cargo test
```

### Test Backend API

```bash
cd backend
# Start the server first, then in another terminal:
curl -X POST http://localhost:8000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Convert 5 miles to kilometers"}'
```

## Troubleshooting

### Rust Module Not Found

If you get an import error for `converterator_rust`, make sure you've run `maturin develop` in the `rust/` directory.

### API Key Errors

Ensure your `.env` file in the `backend/` directory has the correct API key set.

### Port Conflicts

If port 8000 or 3000 are in use, you can:
- Change the backend port: `uvicorn main:app --reload --port 8001`
- Change the frontend port by setting `PORT` environment variable

## Next Steps

- Set up Physics MCP integration (optional)
- Configure CI/CD pipelines
- Deploy to your preferred cloud provider

