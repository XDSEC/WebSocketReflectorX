package main

import (
	"log"
	"strconv"
	"wsrx/mapper"
	"wsrx/server/global"
	"wsrx/server/initialize"
)

func main() {
	err := initialize.InitConfig()
	if err != nil {
		log.Panic(err)
		return
	}
	err = initialize.InitCache()
	if err != nil {
		log.Panic(err)
		return
	}
	engine, err := initialize.InitRouter()
	if err != nil {
		log.Panic(err)
		return
	}
	go mapper.Manager.Start()
	err = engine.Run(global.Config.Server.Address + ":" + strconv.FormatInt(int64(global.Config.Server.Port), 10))
	if err != nil {
		log.Panic(err)
		return
	}
}
