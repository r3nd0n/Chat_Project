# Chat_Project
🇺🇲 En:
This is a Chat Project designed and programmed in Rust for a course on Modeling Programming.

We used some concepts of Networks like TCP, sockets and servers.

~ HOW TO START THE PROGRAM ~

in the terminal write the command as follows in the /server or
/client directory:

cargo run -- <"ip_address":"port">

Replace "ip_address" with the ip direction you are using to connect. Do the same with "port" to stablish the port that listen
for connections.

Ex:
cargo run -- 127.0.0.1:8080

Docker
------

Build images manually:

docker build -f server/Dockerfile -t chat-project-server .
docker build -f client/Dockerfile -t chat-project-client .

Run server image:

docker run --rm -p 8080:8080 chat-project-server

Run client image (interactive):

docker run --rm -it --network host chat-project-client client 127.0.0.1:8080

Use Docker Compose to build and run both services:

docker compose up --build

Notes:
- Server image listens on 0.0.0.0:8080 by default.
- Client image connects to chat-server:8080 by default inside compose network.


🇪🇦 ES:
Proyecto de chat diseñado y programado en Rust para un curso de Modelado y programación.

Hemos usado algunos conceptos de Redes como TCP, sockets y servidores.

~ PARA INICIAR EL PROGRAMA ~

En términal escribir el siguiente comando desde el directorio 
/server o /cleint respectivamente:

cargo run -- <"dirección_ip":"puerto">

Reemplazar "direccion_ip" por la dirección ip a la que se desea
establecer la conexión. Hacer lo mismo con "puerto" para 
establecer el puerto que escuchará conexiones.

Ejemplo:
cargo run -- 127.0.0.1:8080

Docker (ES)
-----------

Construir imágenes manualmente:

docker build -f server/Dockerfile -t chat-project-server .
docker build -f client/Dockerfile -t chat-project-client .

Ejecutar imagen del servidor:

docker run --rm -p 8080:8080 chat-project-server

Ejecutar imagen del cliente (interactivo):

docker run --rm -it --network host chat-project-client client 127.0.0.1:8080

Usar Docker Compose para construir y levantar ambos servicios:

docker compose up --build

Notas:
- La imagen del servidor escucha en 0.0.0.0:8080 por defecto.
- La imagen del cliente se conecta a chat-server:8080 por defecto dentro de la red de compose.

Guia rapida Docker (ES)
-----------------------

Requisitos:
- Docker Engine 24+.
- Docker Compose v2.

1) Levantar solo el servidor con Docker:

docker compose up --build chat-server

2) Conectar un cliente interactivo desde otra terminal:

docker compose run --rm chat-client

3) Conectar mas clientes (repetir comando en nuevas terminales):

docker compose run --rm chat-client

Variables de entorno utiles:
- CHAT_BIND_ADDR: direccion del servidor dentro del contenedor.
	Ejemplo: CHAT_BIND_ADDR=0.0.0.0:9090
- CHAT_SERVER_ADDR: destino del cliente.
	Ejemplo: CHAT_SERVER_ADDR=chat-server:8080

Ejemplos con variables:

CHAT_BIND_ADDR=0.0.0.0:9090 docker compose up --build chat-server
CHAT_SERVER_ADDR=chat-server:9090 docker compose run --rm chat-client

Parar y limpiar contenedores de compose:

docker compose down

Problemas comunes:
- Error "Address already in use": el puerto 8080 ya esta ocupado en tu host.
	Solucion: cambia el mapeo en docker-compose.yml (por ejemplo 9090:8080)
	o libera el proceso que usa el puerto 8080.

🇩🇪 De:
In Entwickluing...
