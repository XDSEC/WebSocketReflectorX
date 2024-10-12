# Local Daemon API

WebSocket Reflector X provides a local API server to control it from remote.

## Start the daemon API server

WebSocket Reflector X Desktop App will automatically start the daemon server at `127.0.0.1:3307`, if `3307` is not available, it will fallback to random available port, you can get it from the Logs panel in Desktop App.

If you want to integrate the daemon server from `popen` without desktop app, you can run the daemon server with the options below:

```shell
$ wsrx daemon
```

Available options could be shown by `--help`:

```
$ wsrx daemon --help
Launch wsrx daemon

Usage: wsrx daemon [OPTIONS]

Options:
      --host <HOST>            The admin and ws http address to listen on
  -p, --port <PORT>            The admin and ws http port to listen on
  -s, --secret <SECRET>        
  -l, --log-json <LOG_JSON>    Log in json format [possible values: true, false]
      --heartbeat <HEARTBEAT>  The heartbeat interval in seconds. If not set, the daemon will not automatically exit when heartbeat timeout
  -h, --help                   Print help
```

## Admin API

### Auth

WebSocket Reflector X's API daemon server has two methods to protect the admin API.

#### HTTP-Header `Authorization`

If you launch the daemon server with option `-s xxx` or `--secret xxx`, the API server will protect admin route with authorization header validation. You should insert `Authorization: xxx` in every request headers.

#### CORS protection

By default, the daemon server will reject any request from a different host. If you want to access daemon server from the browser (eg. WebApp integration), you could send connect request to:

```
$ curl -v -i -X POST http://127.0.0.1:3307/connect --data '"https://your.website"' --header 'Content-Type: application/json'
```

this will add your website origin to the pending list. You can check your website status through this endpoint:

```
$ curl -v -i http://127.0.0.1:3307/connect
```

## WIP

refer to [daemon.rs](https://github.com/XDSEC/WebSocketReflectorX/blob/master/wsrx/src/cli/daemon.rs#L100) and [daemon.h](https://github.com/XDSEC/WebSocketReflectorX/blob/master/desktop/daemon.h)
