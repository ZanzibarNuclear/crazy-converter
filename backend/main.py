from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from app.api import routes

app = FastAPI(title="Crazy Converterator API", version="0.1.0")

# Configure CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # In production, specify exact origins
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
app.include_router(routes.router, prefix="/api", tags=["api"])


@app.get("/")
async def root():
    return {"message": "Crazy Converterator API", "status": "running"}


@app.get("/health")
async def health():
    """Health check endpoint that verifies all critical components."""
    import os
    from app.services import conversion_service
    
    llm_provider = os.getenv("LLM_PROVIDER", "ollama")
    llm_model = os.getenv("LLM_MODEL", "qwen2.5-coder:7b")
    
    health_status = {
        "status": "healthy",
        "components": {
            "api": "ok",
            "rust_module": "ok" if conversion_service.RUST_MODULE_AVAILABLE else "not_available",
            "llm_provider": llm_provider,
            "llm_model": llm_model
        }
    }
    
    warnings = []
    
    if not conversion_service.RUST_MODULE_AVAILABLE:
        health_status["status"] = "degraded"
        warnings.append(
            "Rust conversion module not available. "
            "Run 'maturin develop' in the rust/ directory to enable conversions."
        )
    
    # Add provider-specific status info
    if llm_provider == "ollama":
        ollama_url = os.getenv("OLLAMA_BASE_URL", "http://localhost:11434/v1")
        health_status["components"]["ollama_url"] = ollama_url
    
    if warnings:
        health_status["warnings"] = warnings
    
    return health_status

