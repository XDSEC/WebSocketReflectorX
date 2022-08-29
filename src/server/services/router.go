package services

import (
	"github.com/gin-gonic/gin"
	"go.uber.org/zap"
	"net"
	"net/http"
	"net/http/httputil"
	"os"
	"runtime/debug"
	"strings"
	"time"
)

func InitRouter() (*gin.Engine, error) {
	gin.SetMode(gin.ReleaseMode)
	router := gin.New()
	router.Use(GinZap(zap.L(), "", true))
	router.Use(RecoveryWithZap(zap.L(), true))

	// this handler will upgrade to a websocket connection
	// uuid is the key to find which address will be tunneled
	router.GET("/traffic/:uuid", TrafficHandler)

	adminRouter := router.Group("")
	adminRouter.Use(AdminRequired())
	{
		// get all the mappers
		adminRouter.GET("pool", GetMapperListHandler)
		// get logs of this mapper
		adminRouter.GET("pool/:uuid", GetMapperHandler)
		// create a new mapper
		adminRouter.POST("pool", CreateMapperHandler)
		// delete a mapper
		adminRouter.DELETE("pool/:uuid", DeleteMapperHandler)
	}
	return router, nil
}

// Config is config setting for GinZap
type Config struct {
	TimeFormat string
	UTC        bool
	SkipPaths  []string
}

// GinZap returns a gin.HandlerFunc (middleware) that logs requests using uber-go/zap.
//
// Requests with errors are logged using zap.Error().
// Requests without errors are logged using zap.Info().
//
// It receives:
//  1. A time package format string (e.g. time.RFC3339).
//  2. A boolean stating whether to use UTC time zone or local.
func GinZap(logger *zap.Logger, timeFormat string, utc bool) gin.HandlerFunc {
	return GinZapWithConfig(logger, &Config{TimeFormat: timeFormat, UTC: utc})
}

// GinZapWithConfig returns a gin.HandlerFunc using configs
func GinZapWithConfig(logger *zap.Logger, conf *Config) gin.HandlerFunc {
	skipPaths := make(map[string]bool, len(conf.SkipPaths))
	for _, path := range conf.SkipPaths {
		skipPaths[path] = true
	}

	return func(c *gin.Context) {
		start := time.Now()
		// some evil middlewares modify this values
		path := c.Request.URL.Path
		query := c.Request.URL.RawQuery
		c.Next()

		if _, ok := skipPaths[path]; !ok {
			end := time.Now()
			latency := end.Sub(start)
			if conf.UTC {
				end = end.UTC()
			}

			if len(c.Errors) > 0 {
				// Append error field if this is an erroneous request.
				for _, e := range c.Errors.Errors() {
					logger.Error(e)
				}
			} else {
				logger.Sugar().Infof("| %-6s | %3d | %7.3f ms | %-15s | %s - %s",
					c.Request.Method,
					c.Writer.Status(),
					float64(latency.Microseconds())/1000.0,
					c.ClientIP(),
					path,
					query,
				)
			}
		}
	}
}

// RecoveryWithZap returns a gin.HandlerFunc (middleware)
// that recovers from any panics and logs requests using uber-go/zap.
// All errors are logged using zap.Error().
// stack means whether output the stack info.
// The stack info is easy to find where the error occurs but the stack info is too large.
func RecoveryWithZap(logger *zap.Logger, stack bool) gin.HandlerFunc {
	return func(c *gin.Context) {
		defer func() {
			if err := recover(); err != nil {
				// Check for a broken connection, as it is not really a
				// condition that warrants a panic stack trace.
				var brokenPipe bool
				if ne, ok := err.(*net.OpError); ok {
					if se, ok := ne.Err.(*os.SyscallError); ok {
						if strings.Contains(strings.ToLower(se.Error()), "broken pipe") || strings.Contains(strings.ToLower(se.Error()), "connection reset by peer") {
							brokenPipe = true
						}
					}
				}

				httpRequest, _ := httputil.DumpRequest(c.Request, false)
				if brokenPipe {
					logger.Error(c.Request.URL.Path,
						zap.Any("error", err),
						zap.String("request", string(httpRequest)),
					)
					// If the connection is dead, we can't write a status to it.
					_ = c.Error(err.(error))
					c.Abort()
					return
				}

				if stack {
					logger.Sugar().Errorf("[Recovery from panic]\nERROR: %s\nREQUEST:\n%s\nSTACK:\n%s", err, string(httpRequest), debug.Stack())
				} else {
					logger.Sugar().Errorf("[Recovery from panic]\nERROR: %s\nREQUEST:\n%s", err, string(httpRequest))
				}
				c.AbortWithStatus(http.StatusInternalServerError)
			}
		}()
		c.Next()
	}
}
