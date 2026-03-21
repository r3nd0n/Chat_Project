FROM rust:1.85-bookworm AS builder
WORKDIR /app

# Etapa de compilacion: primero copiamos solo dependencias para aprovechar cache.
COPY client/Cargo.toml /app/client/Cargo.toml
RUN mkdir -p /app/client/src \
    && echo "fn main() {}" > /app/client/src/main.rs
RUN cd /app/client && cargo build --release

# Luego copiamos el codigo real y compilamos el binario del cliente.
COPY client/src /app/client/src
RUN cd /app/client && cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Imagen ligera de ejecucion: solo se incluye el ejecutable.
COPY --from=builder /app/client/target/release/client /usr/local/bin/client

# Host:puerto del servidor al que se conecta el cliente por defecto.
ENV CHAT_SERVER_ADDR=chat-server:8080

# Permite cambiar el destino de conexion mediante variable de entorno.
CMD ["sh", "-c", "client ${CHAT_SERVER_ADDR:-chat-server:8080}"]
