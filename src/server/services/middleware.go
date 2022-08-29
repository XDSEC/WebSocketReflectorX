package services

import (
	"github.com/gin-gonic/gin"
	"strings"
	"wsrx/src/config"
)

func AdminRequired() gin.HandlerFunc {
	return func(ctx *gin.Context) {
		if !config.ServerConfig.Auth.Enabled {
			ctx.Next()
			return
		}
		var tokenStr string
		if token := ctx.Request.Header.Get("Authorization"); token == "" {
			ctx.JSON(403, gin.H{
				"code":    403,
				"message": "Auth failed",
			})
			ctx.Abort()
			return
		} else {
			tokenStr = strings.ReplaceAll(token, "Bearer ", "")
			if tokenStr != config.ServerConfig.Auth.AuthToken {
				ctx.JSON(403, gin.H{
					"code":    403,
					"message": "Auth failed",
				})
				ctx.Abort()
				return
			}
		}
		ctx.Next()
	}
}
