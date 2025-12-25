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
    from app.services import conversion_service
    
    health_status = {
        "status": "healthy",
        "components": {
            "api": "ok",
            "rust_module": "ok" if conversion_service.RUST_MODULE_AVAILABLE else "not_available"
        }
    }
    
    if not conversion_service.RUST_MODULE_AVAILABLE:
        health_status["status"] = "degraded"
        health_status["warnings"] = [
            "Rust conversion module not available. "
            "Run 'maturin develop' in the rust/ directory to enable conversions."
        ]
    
    return health_status

