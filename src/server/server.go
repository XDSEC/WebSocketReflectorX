package server

import (
	"strconv"
	"wsrx/src/config"
	"wsrx/src/logger"
	"wsrx/src/mapper"
	"wsrx/src/server/cache"
	"wsrx/src/server/services"
)

func LaunchServer() {
	logger.Info("Hi, I'm not RX, RX is launching...")
	err := cache.InitCache()
	if err != nil {
		logger.PanicAny("RX receives an error while initializing cache: ", err)
		return
	}
	engine, err := services.InitRouter()
	if err != nil {
		logger.PanicAny("RX receives an error while initializing router: ", err)
		return
	}
	go mapper.Manager.Start()
	serveAddr := config.ServerConfig.Address + ":" + strconv.FormatInt(int64(config.ServerConfig.Port), 10)
	logger.InfoAny("RX has launched: ", serveAddr)
	err = engine.Run(serveAddr)
	if err != nil {
		logger.PanicAny("RX receives an error while running server: ", err)
		return
	}
}
