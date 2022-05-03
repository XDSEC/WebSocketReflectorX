package main

import (
	"fmt"
	"github.com/gorilla/websocket"
	uuid "github.com/satori/go.uuid"
	"log"
	"net"
	"os"
	"strconv"
	"wsrx/client/utils"
	"wsrx/mapper"
)

func main() {
	go mapper.Manager.Start()
	argsWithoutPros := os.Args[1:]
	if len(argsWithoutPros) < 1 {
		fmt.Println("Usage: wsrxc <ws addr>")
		os.Exit(1)
	}
	tcpPort, err := utils.GetFreePort()
	if err != nil {
		log.Fatal("get free port:", err)
	}
	l, err := net.Listen("tcp", "localhost:"+strconv.Itoa(tcpPort))
	if err != nil {
		log.Printf("TCP LISTENER: %v", err.Error())
		os.Exit(1)
	}
	// Close the listener when the application closes.
	defer func(l net.Listener) {
		err := l.Close()
		if err != nil {
			log.Printf("TCP LISTENER DISCONNECT ERROR	: %v", err.Error())
		}
	}(l)
	log.Println("WSRX: Redirect challenge traffic to localhost:" + strconv.Itoa(tcpPort))
	for {
		// Listen for an incoming connection.
		tcpConn, err := l.Accept()
		if err != nil {
			log.Printf("TCP ACCEPT: %v", err.Error())
		}
		wsConn, _, err := websocket.DefaultDialer.Dial(argsWithoutPros[0], nil)
		if err != nil {
			log.Println("dial error: ", err)
		}
		client := &mapper.Client{
			ID:     uuid.NewV4().String(),
			Socket: wsConn,
			TCP:    tcpConn,
		}
		mapper.Manager.Register <- client
	}
}
