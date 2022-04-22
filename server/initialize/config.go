package initialize

import (
	"github.com/spf13/viper"
	"log"
	"wsrx/server/global"
)

func InitConfig() error {
	if err := initViper(); err != nil {
		log.Println("Could not init viper.")
		return err
	}
	return nil
}

func initViper() error {
	v := viper.New()
	v.SetConfigName("config")      // name of config file (without extension)
	v.SetConfigType("yaml")        // REQUIRED if the config file does not have the extension in the name
	v.AddConfigPath("/etc/wsrs/")  // path to look for the config file in
	v.AddConfigPath("$HOME/.wsrs") // call multiple times to add many search paths
	v.AddConfigPath(".")           // optionally look for config in the working directory
	if err := v.ReadInConfig(); err != nil {
		log.Println("Could not access config file. maybe it is not exist?")
		return err
	}
	log.Println("Initialized Viper.")
	err := setConfigWithViper(v)
	if err != nil {
		return err
	}
	return nil
}

func setConfigWithViper(v *viper.Viper) error {
	if err := v.Unmarshal(&global.Config); err != nil {
		return err
	}
	return nil
}
