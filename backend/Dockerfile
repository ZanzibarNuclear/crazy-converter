# Crazy Converterator Backend Dockerfile
# Multi-stage build: compile Rust extension, then create production Python image

# =============================================================================
# Stage 1: Build the Rust extension module
# =============================================================================
FROM python:3.12-slim AS rust-builder

# Install Rust and build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install maturin for building Python extensions
RUN pip install maturin

# Copy and build the Rust module
WORKDIR /build
COPY rust/ ./rust/
WORKDIR /build/rust
RUN maturin build --release

# =============================================================================
# Stage 2: Production Python image
# =============================================================================
FROM python:3.12-slim AS production

WORKDIR /app

# Install the built Rust wheel
COPY --from=rust-builder /build/rust/target/wheels/*.whl /tmp/
RUN pip install /tmp/*.whl && rm /tmp/*.whl

# Install Python dependencies
COPY backend/requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Add gunicorn for production ASGI serving
RUN pip install gunicorn

# Copy application code
COPY backend/app ./app
COPY backend/main.py .

# Create non-root user for security
RUN useradd --create-home --shell /bin/bash appuser && chown -R appuser:appuser /app
USER appuser

# Expose port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8000/health')" || exit 1

# Run with Gunicorn + Uvicorn workers for production
# -w 4: 4 worker processes (adjust based on CPU cores: 2-4 x num_cores)
# -b 0.0.0.0:8000: bind to all interfaces on port 8000
CMD ["gunicorn", "main:app", "-k", "uvicorn.workers.UvicornWorker", "-w", "4", "-b", "0.0.0.0:8000"]

