package proxy

import (
	"github.com/gorilla/websocket"
	"log"
	"net"
	"wsrx/adapter"
)

func chanFromConn(conn net.Conn) chan []byte {
	c := make(chan []byte)

	go func() {
		b := make([]byte, 1024)

		for {
			n, err := conn.Read(b)
			if n > 0 {
				res := make([]byte, n)
				// Copy the buffer, so it doesn't get changed while read by the recipient.
				copy(res, b[:n])
				c <- res
			}
			if err != nil {
				c <- nil
				break
			}
		}
	}()

	return c
}

// Copy accepts a websocket connection and TCP connection and copies data between them
func Copy(id string, wsConn *websocket.Conn, tcpConn net.Conn, unregister chan string) {
	wsConnAdapter := adapter.New(wsConn)
	wsChan := chanFromConn(wsConnAdapter)
	tcpChan := chanFromConn(tcpConn)
	for {
		select {
		case wsData := <-wsChan:
			if wsData == nil {
				log.Printf("WebSocket connection closed: D: %v, S: %v", tcpConn.LocalAddr(), wsConnAdapter.RemoteAddr())
				unregister <- id
				return
			} else {
				_, err := tcpConn.Write(wsData)
				if err != nil {
					log.Println("Error writing to TCP connection:", err)
					unregister <- id
					return
				}
			}
		case tcpData := <-tcpChan:
			if tcpData == nil {
				log.Printf("TCP connection closed: D: %v, S: %v", tcpConn.LocalAddr(), wsConnAdapter.LocalAddr())
				unregister <- id
				return
			} else {
				_, err := wsConnAdapter.Write(tcpData)
				if err != nil {
					log.Println("Error writing to WebSocket connection:", err)
					unregister <- id
					return
				}
			}
		}
	}

}
