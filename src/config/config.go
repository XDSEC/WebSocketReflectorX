package config

import (
	"github.com/spf13/viper"
)

var Version = "UNKNOWN"

func Initialize() error {
	if err := initViper(); err != nil {
		return err
	}
	return nil
}

func initViper() error {
	v := viper.New()
	v.SetConfigName("config")      // name of config file (without extension)
	v.SetConfigType("yaml")        // REQUIRED if the config file does not have the extension in the name
	v.AddConfigPath("/etc/wsrx/")  // path to look for the config file in
	v.AddConfigPath("$HOME/.wsrx") // call multiple times to add many search paths
	v.AddConfigPath(".")           // optionally look for config in the working directory
	v.SetDefault("server", WSRXServerConfig{
		Address: "0.0.0.0",
		Port:    1145,
		Auth:    AuthConfig{Enabled: false},
		Cache:   CacheConfig{CachePath: "."},
	})
	v.SetDefault("client", WSRXClientConfig{
		Plain: false,
		Port:  0,
	})
	v.SetDefault("logger", WSRXLoggerConfig{
		Level:         "info",
		Format:        "console",
		Directory:     ".",
		MaxAge:        4320,
		LinkName:      "wsrx",
		ShowLine:      false,
		EncodeLevel:   "CapitalLevelEncoder",
		StacktraceKey: "WSRX",
		LogInConsole:  true,
		LogInFile:     false,
	})
	if err := v.ReadInConfig(); err != nil {
		//log.Println("Could not access config file. maybe it is not exist?")
		return err
	}

	clientConfigReader := v.Sub("client")
	if clientConfigReader != nil {
		err := clientConfigReader.Unmarshal(&ClientConfig)
		if err != nil {
			return err
		}
	}

	serverConfigReader := v.Sub("server")
	if serverConfigReader != nil {
		err := serverConfigReader.Unmarshal(&ServerConfig)
		if err != nil {
			return err
		}
	}

	loggerConfigReader := v.Sub("logger")
	if loggerConfigReader != nil {
		err := loggerConfigReader.Unmarshal(&LoggerConfig)
		if err != nil {
			return err
		}
	}
	return nil
}
