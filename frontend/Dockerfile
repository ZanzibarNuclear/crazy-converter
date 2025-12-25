# Frontend Production Dockerfile
# Builds static files and serves with nginx

# =============================================================================
# Stage 1: Build the Nuxt application
# =============================================================================
FROM node:20-slim AS builder

WORKDIR /app

# Copy package files
COPY frontend/package*.json ./

# Install dependencies
RUN npm ci

# Copy application code
COPY frontend/ .

# Build static files
RUN npm run generate

# =============================================================================
# Stage 2: Serve with nginx
# =============================================================================
FROM nginx:alpine AS production

# Copy built static files
COPY --from=builder /app/.output/public /usr/share/nginx/html

# Copy nginx configuration
COPY frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Expose port
EXPOSE 80

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost/ || exit 1

CMD ["nginx", "-g", "daemon off;"]

