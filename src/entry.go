package main

import (
	"wsrx/src/cmd"
	"wsrx/src/config"
	"wsrx/src/logger"
)

func main() {
	_ = config.Initialize()
	_ = logger.Initialize()
	_ = cmd.Execute()
}
