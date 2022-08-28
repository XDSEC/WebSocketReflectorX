package services

import (
	"github.com/gin-gonic/gin"
	"strings"
	"wsrx/server/global"
)

func AdminRequired() gin.HandlerFunc {
	return func(ctx *gin.Context) {
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
			if tokenStr != global.Config.Auth.Secret {
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
