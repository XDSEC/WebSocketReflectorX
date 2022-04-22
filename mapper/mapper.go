package mapper

import (
	"github.com/gorilla/websocket"
	"log"
	"net"
	"wsrx/proxy"
)

// ClientManager is a websocket manager
type ClientManager struct {
	Clients    map[string]*Client
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
var Manager = ClientManager{
	Register:   make(chan *Client),
	Unregister: make(chan string),
	Clients:    make(map[string]*Client),
}

func (manager *ClientManager) Start() {
	for {
		select {
		case client := <-Manager.Register:
			log.Printf("[CONNECTED]: %s", client.ID)
			Manager.Clients[client.ID] = client
			go proxy.Copy(client.ID, client.Socket, client.TCP, Manager.Unregister)
		case clientId := <-Manager.Unregister:
			log.Printf("[DISCONNECTED]: %s", clientId)
			if client, ok := Manager.Clients[clientId]; ok {
				_ = client.TCP.Close()
				_ = client.Socket.Close()
				delete(Manager.Clients, clientId)
			}
		}
	}
}
