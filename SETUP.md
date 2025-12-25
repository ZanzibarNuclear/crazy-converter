# Setup Guide for Crazy Converterator

This guide will help you set up and run the Crazy Converterator project.

## Prerequisites

- Python 3.8+
- uv (Python package installer and virtual environment manager)
- Node.js 18+ and npm
- Rust (for building the conversion library)
- Cargo (Rust package manager)
- An OpenAI or Anthropic API key

## Setup Steps

### 1. Create Python Virtual Environment

```bash
# From the project root directory
uv venv
source .venv/bin/activate  # On Windows: .venv\Scripts\activate
```

This creates a virtual environment in `.venv/` and activates it. You should see `(.venv)` in your terminal prompt.

### 2. Build the Rust Conversion Library

```bash
cd rust
uv pip install maturin
maturin develop
cd ..
```

This will build the Rust extension module and make it available to Python in the virtual environment.

### 3. Set Up the Backend

```bash
cd backend
uv pip install -r requirements.txt

# Create a .env file with your API keys
# Create .env file with the following content (replace with your actual keys):
cat > .env << EOF
LLM_PROVIDER=openai
LLM_MODEL=gpt-4o-mini
OPENAI_API_KEY=your_openai_api_key_here
ANTHROPIC_API_KEY=your_anthropic_api_key_here
EOF
# Edit .env and add your actual API keys
cd ..
```

**Note**: Make sure your virtual environment is activated (you should see `(.venv)` in your prompt) before running these commands.

### 4. Set Up the Frontend

```bash
cd frontend
npm install
cd ..
```

### 5. Run the Application

**Important**: Make sure your virtual environment is activated before running the backend.

**Terminal 1 - Backend:**

```bash
# Activate virtual environment if not already active
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

cd backend
uvicorn main:app --reload --port 8000
```

**Terminal 2 - Frontend:**

```bash
cd frontend
npm run dev
```

### 6. Access the Application

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

**Using Cargo tests:**

```bash
cd rust
cargo test
```

**Using Python test script:**

```bash
# Make sure virtual environment is activated
source .venv/bin/activate  # On Windows: .venv\Scripts\activate

cd backend
python test_conversions.py
```

This script tests conversions across all categories and verifies the Rust module is properly installed.

### Test Backend API

**Health Check:**

```bash
curl http://localhost:8000/health
```

**Chat Endpoint:**

```bash
cd backend
# Start the server first, then in another terminal:
curl -X POST http://localhost:8000/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Convert 5 miles to kilometers", "conversation_history": []}'
```

## Troubleshooting

### Rust Module Not Found

If you get an import error for `converterator_rust`:

1. Make sure your virtual environment is activated
2. Make sure you've run `maturin develop` in the `rust/` directory with the virtual environment activated
3. Verify the module is installed: `python -c "import converterator_rust; print('OK')"` (should work from within the activated venv)

### API Key Errors

Ensure your `.env` file in the `backend/` directory has the correct API key set.

### Port Conflicts

If port 8000 or 3000 are in use, you can:

- Change the backend port: `uvicorn main:app --reload --port 8001`
- Change the frontend port by setting `PORT` environment variable

### Virtual Environment Not Activated

If you see import errors or "command not found" errors:

- Make sure you've activated the virtual environment: `source .venv/bin/activate` (or `.venv\Scripts\activate` on Windows)
- You should see `(.venv)` in your terminal prompt when it's activated
- The virtual environment must be activated in each new terminal session

## Next Steps

- Set up Physics MCP integration (optional)
- Configure CI/CD pipelines
- Deploy to your preferred cloud provider
