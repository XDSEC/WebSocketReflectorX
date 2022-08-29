package cmd

import (
	"fmt"
	"github.com/spf13/cobra"
	"wsrx/src/config"
)

var versionCmd = cobra.Command{
	Use:   "version",
	Short: "Print the version number of WebSocketReflectorX.",
	Long:  `All software has versions, so do WSRX.`,
	Run: func(cmd *cobra.Command, args []string) {
		fmt.Println("WebSocketReflectorX", config.Version)
	},
}
