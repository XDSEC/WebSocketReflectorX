package main

import (
	"fmt"
	"github.com/gorilla/websocket"
	uuid "github.com/satori/go.uuid"
	"log"
	"net"
	"os"
	"os/signal"
	"syscall"
	"strconv"
	"wsrx/client/utils"
	"wsrx/mapper"
)

func main() {
	go mapper.Manager.Start()
	argsWithoutPros := os.Args[1:]
	if len(argsWithoutPros) < 1 {
		fmt.Println("Usage: wsrxc <ws addr> [<port>]")
		os.Exit(1)
	}
	var tcpPort string
	var err error
	if len(argsWithoutPros) > 1 {
		tcpPort = argsWithoutPros[1]
	} else {
		tcpPortInt, err := utils.GetFreePort()
		if err != nil {
			log.Fatal("RX could not get a free port:", err)
		}
		tcpPort = strconv.Itoa(tcpPortInt)
	}
	l, err := net.Listen("tcp", "localhost:"+tcpPort)
	if err != nil {
		log.Printf("RX could not listen at aim address: %v", err.Error())
		os.Exit(1)
	}
	// Close the listener when the application closes.
	defer func(l net.Listener) {
		err := l.Close()
		if err != nil {
			log.Printf("RX has an error in disconnecting with client: %v", err.Error())
		}
	}(l)
	log.Println("Hi, I'm not RX, RX is here -> localhost:" + tcpPort)
	
	sigChan := make(chan os.Signal, 2)
	signal.Notify(sigChan, os.Interrupt, syscall.SIGTERM)
	go func() {
		<-sigChan
		fmt.Println("\nOk, RX will go to sleep now.")
		os.Exit(0)
	}()

	for {
		// Listen for an incoming connection.
		tcpConn, err := l.Accept()
		if err != nil {
			log.Printf("A new client is connected to RX: %v", err.Error())
		}
		wsConn, _, err := websocket.DefaultDialer.Dial(argsWithoutPros[0], nil)
		if err != nil {
			log.Println("RX has an error in communicating with server: ", err)
		}
		client := &mapper.Client{
			ID:     uuid.NewV4().String(),
			Socket: wsConn,
			TCP:    tcpConn,
		}
		mapper.Manager.Register <- client
	}
}
