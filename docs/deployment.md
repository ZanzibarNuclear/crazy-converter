# Deployment Guide for Crazy Converterator

This guide covers deployment strategies for Crazy Converterator, from local development to production hosting.

## Table of Contents

- [Architecture Overview](#architecture-overview)
- [Local Development](#local-development)
- [Docker Deployment](#docker-deployment)
- [Production Deployment Options](#production-deployment-options)
- [Environment Configuration](#environment-configuration)
- [CI/CD Pipeline](#cicd-pipeline)
- [Troubleshooting](#troubleshooting)

---

## Architecture Overview

Crazy Converterator consists of three main components:

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│    Frontend     │────▶│    Backend      │────▶│  LLM Provider   │
│   (Nuxt 3)      │     │   (FastAPI)     │     │ (OpenAI/Claude) │
│   Port 3000     │     │   Port 8000     │     │                 │
└─────────────────┘     └────────┬────────┘     └─────────────────┘
                                 │
                        ┌────────▼────────┐
                        │  Rust Module    │
                        │ (converterator  │
                        │     _rust)      │
                        └─────────────────┘
```

**Key Deployment Considerations:**

1. **Rust Module**: The backend depends on a Rust extension module built with PyO3/Maturin. This must be compiled during the Docker build or installed in the Python environment.

2. **Frontend**: Nuxt 3 can be deployed as:
   - **Static files** (`nuxt generate`) - hosted on any static hosting service
   - **Node.js server** (`nuxt build`) - requires Node.js runtime

3. **Backend**: FastAPI requires an ASGI server. In production, use Gunicorn with Uvicorn workers.

---

## Local Development

For local development, see [SETUP.md](../SETUP.md) in the project root.

**Quick Start:**

```bash
# 1. Create and activate virtual environment
uv venv
source .venv/bin/activate

# 2. Build Rust module
cd rust && uv pip install maturin && maturin develop && cd ..

# 3. Install backend dependencies and create .env
cd backend && uv pip install -r requirements.txt
cp .env.example .env  # Then edit with your API keys
cd ..

# 4. Install frontend dependencies
cd frontend && npm install && cd ..

# 5. Run both services (in separate terminals)
# Terminal 1 (backend):
source .venv/bin/activate && cd backend && uvicorn main:app --reload --port 8000

# Terminal 2 (frontend):
cd frontend && npm run dev
```

---

## Docker Deployment

Docker is the **recommended approach** for production because it handles the Rust compilation complexity and ensures consistency across environments.

### Backend Dockerfile

Create `backend/Dockerfile`:

```dockerfile
# Stage 1: Build the Rust extension
FROM python:3.12-slim AS rust-builder

# Install Rust and build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install maturin
RUN pip install maturin

# Copy and build the Rust module
WORKDIR /build
COPY rust/ ./rust/
WORKDIR /build/rust
RUN maturin build --release

# Stage 2: Production Python image
FROM python:3.12-slim AS production

WORKDIR /app

# Install the built wheel and Python dependencies
COPY --from=rust-builder /build/rust/target/wheels/*.whl /tmp/
RUN pip install /tmp/*.whl && rm /tmp/*.whl

COPY backend/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Add gunicorn for production
RUN pip install gunicorn

# Copy application code
COPY backend/ .

# Create non-root user for security
RUN useradd --create-home appuser && chown -R appuser:appuser /app
USER appuser

# Expose port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8000/health')" || exit 1

# Run with Gunicorn + Uvicorn workers
CMD ["gunicorn", "main:app", "-k", "uvicorn.workers.UvicornWorker", "-w", "4", "-b", "0.0.0.0:8000"]
```

### Frontend Dockerfile (Static Build)

Create `frontend/Dockerfile`:

```dockerfile
# Build stage
FROM node:20-slim AS builder

WORKDIR /app
COPY frontend/package*.json ./
RUN npm ci

COPY frontend/ .
RUN npm run generate

# Production stage - serve with nginx
FROM nginx:alpine AS production

COPY --from=builder /app/.output/public /usr/share/nginx/html
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
```

### Nginx Configuration for Frontend

Create `frontend/nginx.conf`:

```nginx
server {
    listen 80;
    server_name _;
    root /usr/share/nginx/html;
    index index.html;

    # Handle SPA routing
    location / {
        try_files $uri $uri/ /index.html;
    }

    # Cache static assets
    location ~* \.(js|css|png|jpg|jpeg|gif|ico|svg|woff|woff2)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
```

### Docker Compose (Full Stack)

Create `docker-compose.yml` in project root:

```yaml
version: '3.8'

services:
  backend:
    build:
      context: .
      dockerfile: backend/Dockerfile
    ports:
      - "8000:8000"
    environment:
      - LLM_PROVIDER=${LLM_PROVIDER:-openai}
      - LLM_MODEL=${LLM_MODEL:-gpt-4o-mini}
      - OPENAI_API_KEY=${OPENAI_API_KEY}
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    healthcheck:
      test: ["CMD", "python", "-c", "import urllib.request; urllib.request.urlopen('http://localhost:8000/health')"]
      interval: 30s
      timeout: 3s
      retries: 3

  frontend:
    build:
      context: .
      dockerfile: frontend/Dockerfile
    ports:
      - "3000:80"
    depends_on:
      - backend
    environment:
      - API_BASE=http://backend:8000

networks:
  default:
    driver: bridge
```

### Building and Running with Docker

```bash
# Build and run all services
docker-compose up --build

# Or build individually
docker build -f backend/Dockerfile -t converterator-backend .
docker build -f frontend/Dockerfile -t converterator-frontend .

# Run with environment variables
docker run -p 8000:8000 \
  -e OPENAI_API_KEY=your_key \
  -e LLM_PROVIDER=openai \
  -e LLM_MODEL=gpt-4o-mini \
  converterator-backend
```

---

## Production Deployment Options

### Option 1: PaaS (Recommended for Simplicity)

**Best for**: Small to medium projects, quick deployment, minimal DevOps overhead.

| Platform | Pros | Cons | Cost |
|----------|------|------|------|
| **Render** | Easy GitHub deploys, free tier, native Docker support | Cold starts on free tier | Free → $7/mo |
| **Railway** | Fast deploys, good DX, monorepo support | Smaller community | Free → $5/mo |
| **Fly.io** | Global edge deployment, good for containers | Steeper learning curve | Free → $5/mo |

**Deploying to Render:**

1. Create a `render.yaml` in project root:

```yaml
services:
  - type: web
    name: converterator-backend
    env: docker
    dockerfilePath: ./backend/Dockerfile
    dockerContext: .
    envVars:
      - key: LLM_PROVIDER
        value: openai
      - key: LLM_MODEL
        value: gpt-4o-mini
      - key: OPENAI_API_KEY
        sync: false  # Set manually in dashboard
    healthCheckPath: /health

  - type: web
    name: converterator-frontend
    env: static
    buildCommand: cd frontend && npm install && npm run generate
    staticPublishPath: frontend/.output/public
    envVars:
      - key: API_BASE
        fromService:
          type: web
          name: converterator-backend
          property: host
```

2. Connect your GitHub repository to Render
3. Add your API keys in the Render dashboard
4. Deploy!

### Option 2: Cloud Run (Google Cloud)

**Best for**: Auto-scaling, pay-per-request, serverless containers.

```bash
# Build and push to Google Container Registry
gcloud builds submit --tag gcr.io/PROJECT_ID/converterator-backend

# Deploy to Cloud Run
gcloud run deploy converterator-backend \
  --image gcr.io/PROJECT_ID/converterator-backend \
  --platform managed \
  --allow-unauthenticated \
  --set-env-vars "LLM_PROVIDER=openai,LLM_MODEL=gpt-4o-mini" \
  --set-secrets "OPENAI_API_KEY=openai-key:latest"
```

### Option 3: AWS (ECS/Fargate)

**Best for**: Enterprise, complex infrastructure, AWS ecosystem integration.

See AWS documentation for ECS deployment with Fargate. Key steps:
1. Push Docker image to ECR
2. Create ECS task definition
3. Create ECS service with load balancer
4. Configure secrets in AWS Secrets Manager

### Option 4: Self-Hosted (VPS)

**Best for**: Full control, cost-effective at scale, specific compliance needs.

```bash
# On your VPS (Ubuntu/Debian)
# Install Docker
curl -fsSL https://get.docker.com | sh

# Clone your repo
git clone https://github.com/yourusername/crazy-converter.git
cd crazy-converter

# Create .env file with your keys
cat > .env << EOF
LLM_PROVIDER=openai
LLM_MODEL=gpt-4o-mini
OPENAI_API_KEY=your_key_here
EOF

# Run with Docker Compose
docker-compose up -d

# Set up Nginx reverse proxy with SSL (using Certbot)
sudo apt install nginx certbot python3-certbot-nginx
# Configure nginx and run certbot for SSL
```

---

## Environment Configuration

### Required Environment Variables

| Variable | Description | Required |
|----------|-------------|----------|
| `LLM_PROVIDER` | LLM provider: `openai` or `anthropic` | Yes |
| `LLM_MODEL` | Model name (e.g., `gpt-4o-mini`, `claude-3-5-sonnet-20241022`) | Yes |
| `OPENAI_API_KEY` | OpenAI API key | If using OpenAI |
| `ANTHROPIC_API_KEY` | Anthropic API key | If using Anthropic |

### Frontend Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `API_BASE` | Backend API URL | `http://localhost:8000` |

### Example `.env` File

Create `backend/.env.example`:

```bash
# LLM Configuration
LLM_PROVIDER=openai
LLM_MODEL=gpt-4o-mini

# API Keys (never commit actual keys!)
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
```

### Security Best Practices

1. **Never commit API keys** - Use `.env` files (gitignored) or platform secrets
2. **Rotate keys regularly** - Especially if exposed
3. **Use platform secret management**:
   - Render: Environment Groups
   - Railway: Secrets
   - AWS: Secrets Manager
   - GCP: Secret Manager

---

## CI/CD Pipeline

### GitHub Actions Workflow

Create `.github/workflows/deploy.yml`:

```yaml
name: Build and Deploy

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Set up Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Run Rust tests
        run: cd rust && cargo test

      - name: Set up Python
        uses: actions/setup-python@v5
        with:
          python-version: '3.12'

      - name: Install dependencies
        run: |
          pip install maturin
          cd rust && maturin build
          pip install rust/target/wheels/*.whl
          pip install -r backend/requirements.txt

      - name: Run Python tests
        run: cd backend && python test_conversions.py

  build-and-push:
    needs: test
    runs-on: ubuntu-latest
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    permissions:
      contents: read
      packages: write

    steps:
      - uses: actions/checkout@v4

      - name: Log in to Container Registry
        uses: docker/login-action@v3
        with:
          registry: ${{ env.REGISTRY }}
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Build and push backend
        uses: docker/build-push-action@v5
        with:
          context: .
          file: backend/Dockerfile
          push: true
          tags: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}-backend:latest

      - name: Build and push frontend
        uses: docker/build-push-action@v5
        with:
          context: .
          file: frontend/Dockerfile
          push: true
          tags: ${{ env.REGISTRY }}/${{ env.IMAGE_NAME }}-frontend:latest

  # Add deploy job based on your platform
  # deploy:
  #   needs: build-and-push
  #   ...
```

---

## Troubleshooting

### Common Issues

#### Rust Module Not Found in Docker

**Symptom**: `ModuleNotFoundError: No module named 'converterator_rust'`

**Solution**: Ensure the wheel is installed in the production stage:
```dockerfile
COPY --from=rust-builder /build/rust/target/wheels/*.whl /tmp/
RUN pip install /tmp/*.whl
```

#### CORS Errors in Production

**Symptom**: Frontend can't reach backend API

**Solution**: Update `backend/main.py` to use specific origins:
```python
app.add_middleware(
    CORSMiddleware,
    allow_origins=["https://your-frontend-domain.com"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
```

#### Cold Starts on Free Tier

**Symptom**: First request after idle period is slow

**Solution**: 
- Upgrade to paid tier, or
- Set up a health check ping to keep the service warm

#### API Key Errors

**Symptom**: `401 Unauthorized` or API key errors

**Solution**:
1. Verify environment variables are set correctly
2. Check API key validity with the provider
3. Ensure you're using the right provider (`LLM_PROVIDER`)

### Health Check Endpoint

The `/health` endpoint provides system status:

```bash
curl https://your-backend-url.com/health
```

Response:
```json
{
  "status": "healthy",
  "components": {
    "api": "ok",
    "rust_module": "ok"
  }
}
```

If `rust_module` is `"not_available"`, the Rust extension wasn't installed correctly.

---

## Summary

| Deployment Type | Complexity | Best For |
|----------------|------------|----------|
| Local (dev) | Low | Development, testing |
| Docker Compose | Medium | Staging, small production |
| PaaS (Render/Railway) | Low | Most production use cases |
| Cloud Run/Fargate | Medium | Auto-scaling, serverless |
| Self-hosted + K8s | High | Enterprise, high scale |

**Recommended path for most projects:**
1. Start with local development using `SETUP.md`
2. Test with Docker Compose locally
3. Deploy to Render or Railway for production
4. Scale up to Cloud Run or Kubernetes as needed

