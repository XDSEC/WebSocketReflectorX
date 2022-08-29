package logger

import (
	"fmt"
	prostates "github.com/lestrrat-go/file-rotatelogs"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
	"os"
	"path"
	"time"
	"wsrx/src/config"
	"wsrx/src/utility"
)

var level zapcore.Level

func Initialize() error {
	if config.LoggerConfig.LogInFile {
		if ok, _ := utility.PathExists(config.LoggerConfig.Directory); !ok {
			err := utility.CreateDir(config.LoggerConfig.Directory)
			if err != nil {
				return err
			}
		}
	}
	var logger *zap.Logger
	switch config.LoggerConfig.Level {
	case "debug":
		level = zap.DebugLevel
	case "info":
		level = zap.InfoLevel
	case "warn":
		level = zap.WarnLevel
	case "error":
		level = zap.ErrorLevel
	case "dpanic":
		level = zap.DPanicLevel
	case "panic":
		level = zap.PanicLevel
	case "fatal":
		level = zap.FatalLevel
	default:
		level = zap.InfoLevel
	}

	if level == zap.DebugLevel || level == zap.ErrorLevel {
		logger = zap.New(getEncoderCore(), zap.AddStacktrace(level))
	} else {
		logger = zap.New(getEncoderCore())
	}
	if config.LoggerConfig.ShowLine {
		logger = logger.WithOptions(zap.AddCaller())
	}
	zap.ReplaceGlobals(logger)
	return nil
}

func getEncoderConfig() (zapConfig zapcore.EncoderConfig) {
	zapConfig = zapcore.EncoderConfig{
		MessageKey:     "message",
		LevelKey:       "level",
		TimeKey:        "time",
		NameKey:        "logger",
		CallerKey:      "caller",
		StacktraceKey:  config.LoggerConfig.StacktraceKey,
		LineEnding:     zapcore.DefaultLineEnding,
		EncodeLevel:    zapcore.LowercaseLevelEncoder,
		EncodeTime:     customTimeEncoder,
		EncodeDuration: zapcore.SecondsDurationEncoder,
		EncodeCaller:   zapcore.FullCallerEncoder,
	}
	switch {
	case config.LoggerConfig.EncodeLevel == "LowercaseLevelEncoder":
		zapConfig.EncodeLevel = zapcore.LowercaseLevelEncoder
		break
	case config.LoggerConfig.EncodeLevel == "LowercaseColorLevelEncoder":
		zapConfig.EncodeLevel = zapcore.LowercaseColorLevelEncoder
		break
	case config.LoggerConfig.EncodeLevel == "CapitalLevelEncoder":
		zapConfig.EncodeLevel = zapcore.CapitalLevelEncoder
		break
	case config.LoggerConfig.EncodeLevel == "CapitalColorLevelEncoder":
		zapConfig.EncodeLevel = zapcore.CapitalColorLevelEncoder
		break
	default:
		zapConfig.EncodeLevel = zapcore.LowercaseLevelEncoder
		break
	}
	return zapConfig
}

func getEncoder() zapcore.Encoder {
	if config.LoggerConfig.Format == "json" {
		return zapcore.NewJSONEncoder(getEncoderConfig())
	} else if config.LoggerConfig.Format == "console" {
		return zapcore.NewConsoleEncoder(getEncoderConfig())
	}
	return zapcore.NewConsoleEncoder(getEncoderConfig())
}

func getEncoderCore() (core zapcore.Core) {
	writer, err := getWriteSyncer()
	if err != nil {
		fmt.Printf("Get Write Syncer Failed err:%v", err.Error())
		return
	}
	return zapcore.NewCore(getEncoder(), writer, level)
}

func customTimeEncoder(t time.Time, enc zapcore.PrimitiveArrayEncoder) {
	enc.AppendString(t.Format("[2006-01-02 15:04:05]"))
}

func getWriteSyncer() (zapcore.WriteSyncer, error) {
	if config.LoggerConfig.LogInFile {
		fileWriter, err := prostates.New(
			path.Join(config.LoggerConfig.Directory, "%Y-%m-%d.log"),
			prostates.WithMaxAge(time.Duration(config.LoggerConfig.MaxAge)*time.Hour),
			prostates.WithRotationTime(24*time.Hour),
		)
		if config.LoggerConfig.LogInConsole {
			return zapcore.NewMultiWriteSyncer(zapcore.AddSync(os.Stdout), zapcore.AddSync(fileWriter)), err
		}
		return zapcore.AddSync(fileWriter), err
	} else if config.LoggerConfig.LogInConsole {
		return zapcore.AddSync(os.Stdout), nil
	} else {
		return zapcore.AddSync(os.Stdout), nil
	}
}
