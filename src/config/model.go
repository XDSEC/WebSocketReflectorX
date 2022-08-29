package config

type WSRXServerConfig struct {
	Address string      `mapstructure:"address" json:"address" yaml:"address"`
	Port    int         `mapstructure:"port" json:"port" yaml:"port"`
	Auth    AuthConfig  `mapstructure:"auth" json:"auth" yaml:"auth"`
	Cache   CacheConfig `mapstructure:"cache" json:"cache" yaml:"cache"`
}

var ServerConfig WSRXServerConfig

type AuthConfig struct {
	Enabled   bool   `mapstructure:"enabled" json:"enabled" yaml:"enabled"`
	AuthToken string `mapstructure:"auth_token" json:"auth-token" yaml:"auth_token"`
}

type CacheConfig struct {
	CachePath string `mapstructure:"cache_path" json:"cache-path" yaml:"cache_path"`
}

type WSRXClientConfig struct {
	Plain bool `mapstructure:"plain" json:"plain" yaml:"plain"`
	Port  int  `mapstructure:"port" json:"port" yaml:"port"`
}

var ClientConfig WSRXClientConfig

type WSRXLoggerConfig struct {
	Level     string `mapstructure:"level" json:"level" yaml:"level"`
	Format    string `mapstructure:"format" json:"format" yaml:"format"`
	Directory string `mapstructure:"directory" json:"directory"  yaml:"directory"`
	MaxAge    int64  `mapstructure:"max_age" json:"max-age" yaml:"max_age"`
	// LinkName is the name of the symlink to the current log file.
	LinkName      string `mapstructure:"link_name" json:"link-name" yaml:"link_name"`
	ShowLine      bool   `mapstructure:"show_line" json:"show-line" yaml:"show_line"`
	EncodeLevel   string `mapstructure:"encode_level" json:"encode-level" yaml:"encode_level"`
	StacktraceKey string `mapstructure:"stacktrace_key" json:"stacktrace-key" yaml:"stacktrace_key"`
	LogInConsole  bool   `mapstructure:"log_in_console" json:"log-in-console" yaml:"log_in_console"`
	LogInFile     bool   `mapstructure:"log_in_file" json:"log-in-file" yaml:"log_in_file"`
}

var LoggerConfig WSRXLoggerConfig
