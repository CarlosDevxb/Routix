# ==========================================
# 1. Builder Stage
# ==========================================
FROM rust:1.85-bookworm AS builder

WORKDIR /app

# Activar compilación offline de SQLx usando la carpeta .sqlx preparada
ENV SQLX_OFFLINE=true

# Copiar manifiestos, metadata de SQLx y código fuente
COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY src ./src

# Compilar el binario optimizado para producción
RUN cargo build --release

# ==========================================
# 2. Runtime Stage (Contenedor ligero y seguro)
# ==========================================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Instalar certificados CA y librerías básicas de red
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copiar el binario compilado
COPY --from=builder /app/target/release/routix_backend /usr/local/bin/routix_backend

# Copiar las migraciones
COPY migrations ./migrations

EXPOSE 3000
ENV PORT=3000

CMD ["routix_backend"]
