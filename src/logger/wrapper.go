package logger

import (
	"go.uber.org/zap"
)

func Debug(msg string, fields ...zap.Field) {
	zap.L().Debug(msg, fields...)
}

func DebugFmt(fmt string, args ...interface{}) {
	zap.L().Sugar().Debugf(fmt, args...)
}

func DebugAny(args ...interface{}) {
	zap.L().Sugar().Debug(args...)
}

func Info(msg string, fields ...zap.Field) {
	zap.L().Info(msg, fields...)
}

func InfoFmt(fmt string, args ...interface{}) {
	zap.L().Sugar().Infof(fmt, args...)
}

func InfoAny(args ...interface{}) {
	zap.L().Sugar().Info(args...)
}

func Warn(msg string, fields ...zap.Field) {
	zap.L().Warn(msg, fields...)
}

func WarnFmt(fmt string, args ...interface{}) {
	zap.L().Sugar().Warnf(fmt, args...)
}

func WarnAny(args ...interface{}) {
	zap.L().Sugar().Warn(args...)
}

func Error(msg string, fields ...zap.Field) {
	zap.L().Error(msg, fields...)
}

func ErrorFmt(fmt string, args ...interface{}) {
	zap.L().Sugar().Errorf(fmt, args...)
}

func ErrorAny(args ...interface{}) {
	zap.L().Sugar().Error(args...)
}

func Fatal(msg string, fields ...zap.Field) {
	zap.L().Fatal(msg, fields...)
}

func FatalFmt(fmt string, args ...interface{}) {
	zap.L().Sugar().Fatalf(fmt, args...)
}

func FatalAny(args ...interface{}) {
	zap.L().Sugar().Fatal(args...)
}

func DPanic(msg string, fields ...zap.Field) {
	zap.L().DPanic(msg, fields...)
}

func DPanicFmt(fmt string, args ...interface{}) {
	zap.L().Sugar().Panicf(fmt, args...)
}

func DPanicAny(args ...interface{}) {
	zap.L().Sugar().Panic(args...)
}

func Panic(msg string, fields ...zap.Field) {
	zap.L().Panic(msg, fields...)
}

func PanicFmt(fmt string, args ...interface{}) {
	zap.L().Sugar().Panicf(fmt, args...)
}

func PanicAny(args ...interface{}) {
	zap.L().Sugar().Panic(args...)
}

// not all logging functions will be used,
// this declaration is used to ignore "unused" warnings.
var _ = []interface{}{
	Debug,
	DebugFmt,
	DebugAny,
	Info,
	InfoFmt,
	InfoAny,
	Warn,
	WarnFmt,
	WarnAny,
	Error,
	ErrorFmt,
	ErrorAny,
	Fatal,
	FatalFmt,
	FatalAny,
	DPanic,
	DPanicFmt,
	DPanicAny,
	Panic,
	PanicFmt,
	PanicAny,
}
