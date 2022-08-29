package cmd

import (
	"github.com/spf13/cobra"
	"wsrx/src/logger"
)

var connectCmd = cobra.Command{
	Use:   "connect [ws|wss://connection]",
	Short: "connect wsrx network as client",
	Long:  "connect to a websocket link and serve a tcp server on your localhost, all traffics will be tunneled over the websocket link.",
	Run: func(cmd *cobra.Command, args []string) {
		logger.Info("WebSocketReflectorX client")
	},
}
