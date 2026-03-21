FROM rust:1.85-bookworm AS builder
WORKDIR /app

# Etapa de compilacion: primero copiamos solo dependencias para aprovechar cache.
COPY server/Cargo.toml /app/server/Cargo.toml
RUN mkdir -p /app/server/src \
    && echo "fn main() {}" > /app/server/src/main.rs
RUN cd /app/server && cargo build --release

# Luego copiamos el codigo real y generamos el binario final.
COPY server/src /app/server/src
RUN cd /app/server && cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Imagen ligera de ejecucion: solo copiamos el ejecutable compilado.
COPY --from=builder /app/server/target/release/server /usr/local/bin/server

EXPOSE 8080
# Direccion por defecto donde el servidor escuchara conexiones.
ENV CHAT_BIND_ADDR=0.0.0.0:8080

# Permite sobreescribir direccion con variable de entorno en runtime.
CMD ["sh", "-c", "server ${CHAT_BIND_ADDR:-0.0.0.0:8080}"]
