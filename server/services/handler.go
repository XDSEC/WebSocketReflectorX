package services

import (
	"errors"
	"github.com/gin-gonic/gin"
	"github.com/gorilla/websocket"
	"go.etcd.io/bbolt"
	"net"
	"net/http"
	"strings"
	"wsrx/mapper"
	"wsrx/server/global"
	"wsrx/server/models"
)

func GetMapperHandler(ctx *gin.Context) {
	id := ctx.Param("uuid")
	if err := global.Cache.View(func(tx *bbolt.Tx) error {
		b := tx.Bucket([]byte("Default"))
		v := b.Get([]byte(id))
		ctx.JSON(200, gin.H{
			"id":      id,
			"address": string(v),
		})
		return nil
	}); err != nil {
		ctx.JSON(500, gin.H{
			"error": err.Error(),
		})
		ctx.Abort()
		return
	}
	ctx.SetAccepted()
}

func CreateMapperHandler(ctx *gin.Context) {
	var req models.CreateMapperRequest
	err := ctx.ShouldBind(&req)
	if err != nil {
		ctx.JSON(500, gin.H{
			"error": err.Error(),
		})
		ctx.Abort()
		return
	}
	//log.Print(req)
	if err = global.Cache.Update(func(tx *bbolt.Tx) error {
		b := tx.Bucket([]byte("Default"))
		err := b.Put([]byte(req.ID), []byte(req.Address))
		return err
	}); err != nil {
		ctx.JSON(500, gin.H{
			"error": err.Error(),
		})
		ctx.Abort()
		return
	}
	ctx.SetAccepted()
}

func DeleteMapperHandler(ctx *gin.Context) {
	id := ctx.Param("uuid")
	if err := global.Cache.Update(func(tx *bbolt.Tx) error {
		b := tx.Bucket([]byte("Default"))
		err := b.Delete([]byte(id))
		return err
	}); err != nil {
		ctx.JSON(500, gin.H{
			"error": err.Error(),
		})
		ctx.Abort()
		return
	}

	// force close all connections by this mapper
	for s, client := range mapper.Manager.Clients {
		clientId := strings.Split(s, "#")[0]
		if clientId == id {
			mapper.Manager.Unregister <- client.ID
		}
	}
	ctx.SetAccepted()
}

func GetMapperListHandler(ctx *gin.Context) {
	var mappers []models.Mapper
	err := global.Cache.View(func(tx *bbolt.Tx) error {
		b := tx.Bucket([]byte("Default"))
		c := b.Cursor()
		for k, v := c.First(); k != nil; k, v = c.Next() {
			mappers = append(mappers, models.Mapper{
				ID:      string(k),
				Address: string(v),
			})
		}
		return nil
	})
	if err != nil {
		ctx.JSON(500, gin.H{
			"error": err.Error(),
		})
		ctx.Abort()
	}
	ctx.JSON(200, mappers)
	ctx.SetAccepted()
}

func TrafficHandler(ctx *gin.Context) {
	id := ctx.Param("uuid")
	var tcpAddr string
	// check uuid exists and get address
	if err := global.Cache.View(func(tx *bbolt.Tx) error {
		b := tx.Bucket([]byte("Default"))
		v := b.Get([]byte(id))
		if v == nil {
			return errors.New("mapper not found")
		}
		tcpAddr = string(v)
		return nil
	}); err != nil {
		ctx.JSON(404, gin.H{
			"error": err.Error(),
		})
		ctx.Abort()
		return
	}
	conn, err := (&websocket.Upgrader{
		ReadBufferSize:  1024,
		WriteBufferSize: 1024,
		CheckOrigin: func(r *http.Request) bool {
			return true
		},
	}).Upgrade(ctx.Writer, ctx.Request, nil)
	if err != nil {
		ctx.JSON(400, gin.H{
			"error": err.Error(),
		})
		ctx.Abort()
		return
	}
	var remoteTCPConn net.Conn
	remoteTCPConn, err = net.Dial("tcp", tcpAddr)
	if err != nil {
		ctx.JSON(500, gin.H{
			"error": err.Error(),
		})
		ctx.Abort()
		return
	}
	//log.Printf("Proxying traffic to %v on behalf of %v", remoteTCPConn.RemoteAddr(), conn.RemoteAddr())
	client := &mapper.Client{
		ID:     id + "#" + ctx.ClientIP() + "[" + conn.RemoteAddr().String() + "]",
		Socket: conn,
		TCP:    remoteTCPConn,
	}
	mapper.Manager.Register <- client
}
