package mapper

import (
	"github.com/gorilla/websocket"
	"net"
	"sync"
)

// clientManagerStrut is a websocket manager
type clientManagerStrut struct {
	Clients    sync.Map
	Register   chan *Client
	Unregister chan string
}

// Client is a websocket client
type Client struct {
	ID     string
	Socket *websocket.Conn
	TCP    net.Conn
}

// Manager define a ws server manager
var Manager = clientManagerStrut{
	Register:   make(chan *Client),
	Unregister: make(chan string),
	Clients:    sync.Map{},
}

func (manager *clientManagerStrut) Start() {
	for {
		select {
		case client := <-Manager.Register:
			Manager.Clients.Store(client.ID, client)
			go Copy(client.ID, client.Socket, client.TCP)
		case clientId := <-Manager.Unregister:
			if value, ok := manager.Clients.Load(clientId); ok {
				client := value.(*Client)
				_ = client.TCP.Close()
				_ = client.Socket.Close()
				manager.Clients.Delete(clientId)
			}
		}
	}
}
