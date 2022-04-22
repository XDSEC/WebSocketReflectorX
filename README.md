# WSRX (Web Socket Reflector X)

> WebSocket Reflector X

## What is it?

A port mapper that transport/tunnel a TCP connection though the websockets protocol.

## Usage

### Server

```bash
go build -o server server/entry.go
```

`config.yaml`:

```yaml
server:
  address: localhost # server address
  port: 8000 # main server port

cache:
  path: "./pool.db" # where to store mapper config?

auth:
  secret: "<admin api secret>" # Accessing the management api
```

then put config.yaml and server executable file in the same directory, run:

```bash
./server
```

you can use nginx to proxy the server to a public address.

```
server {
  listen 80;
  server_name challenge.domain;
  
  ...
  
  location /traffic {
    proxy_pass http://localhost:8000;
    proxy_redirect off;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Host $server_name;
    proxy_set_header X-Forwarded-Proto $scheme;
  }
  location /pool {
    proxy_pass http://localhost:8000;
    proxy_redirect off;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Host $server_name;
    proxy_set_header X-Forwarded-Proto $scheme;
  }
}
```

### Client

```bash
go build -o client client/entry.go
./client ws://challenge.domain/traffic/<challenge id>
```

then the client will open a tcp server on your localhost, you can access it directly and all traffic will be tunneled through the websocket to challenge.domain.
