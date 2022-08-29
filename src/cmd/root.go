package cmd

import (
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var rootCmd = cobra.Command{
	Use:   "wsrx",
	Short: "WebSocket Reflector X",
	Long:  "WebSocket Reflector X is a tool that can reflect TCP port over websocket addresses.",
	Run: func(cmd *cobra.Command, args []string) {
		connectCmd.Run(cmd, args)
	},
}

func Execute() error {
	return rootCmd.Execute()
}

func init() {
	rootCmd.PersistentFlags().BoolP("verbose", "v", false, "more runtime information")
	_ = viper.BindPFlag("verbose", rootCmd.PersistentFlags().Lookup("verbose"))

	rootCmd.AddCommand(&connectCmd)
	rootCmd.AddCommand(&serveCmd)
	rootCmd.AddCommand(&versionCmd)
}
