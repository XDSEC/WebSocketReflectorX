# WSRX (Web Socket Reflector X)

> WebSocket Reflector X?
>
> ko no [Wo Shi Reverier Xu] da!

## What is it?

A port mapper that transport/tunnel a TCP connection though the websockets protocol.

## Usage

### Server

```bash
go build -o wsrx src/entry.go
```

`config.yaml`:

```yaml
server:
  address: 0.0.0.0 # server address
  port: 1145 # main server port

  auth:
    enabled: true
    auth_token: 1145141919810 # Accessing the management api

  cache:
    cache_path: ./cache.db # where to store mapper config?

# config logger
logger:
  level: info
  format: console
  directory: /var/log/wsrx
  max_age: 30
  link_name: wsrx.latest.log
  show_line: false
  encode_level: CapitalLevelEncoder
  stacktrace_key: wsrx
  log_in_console: true
  log_in_file: false
```

then put config.yaml and server executable file in the same directory, run:

```bash
./wsrx serve
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
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection "Upgrade";
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
./wsrx connect ws://challenge.domain/traffic/<challenge id>
```

then the client will open a tcp server on your localhost, you can access it directly and all traffic will be tunneled through the websocket to challenge.domain.
