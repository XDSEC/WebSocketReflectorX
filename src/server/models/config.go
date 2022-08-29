package models

type Config struct {
	Server ServerConfig `mapstructure:"server" yaml:"server"`
	Cache  CacheConfig  `mapstructure:"cache" yaml:"cache"`
	Auth   AuthConfig   `mapstructure:"auth" yaml:"auth"`
}

type CacheConfig struct {
	Path string `mapstructure:"path" yaml:"path"`
}

type ServerConfig struct {
	Address string `mapstructure:"address" yaml:"address"`
	Port    int    `mapstructure:"port" yaml:"port"`
}

type AuthConfig struct {
	Secret string `mapstructure:"secret" yaml:"secret"`
}
