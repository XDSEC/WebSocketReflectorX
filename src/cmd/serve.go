package cmd

import (
	"github.com/spf13/cobra"
	"wsrx/src/server"
)

var serveCmd = cobra.Command{
	Use:   "serve",
	Short: "serve a wsrx network",
	Long:  "serve a wsrx network which is described in your config file.",
	Run: func(cmd *cobra.Command, args []string) {
		server.LaunchServer()
	},
}
