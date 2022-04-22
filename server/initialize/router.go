package initialize

import (
	"github.com/gin-gonic/gin"
	"wsrx/server/services"
)

func InitRouter() (*gin.Engine, error) {
	router := gin.Default()
	router.Use(gin.Recovery())

	// this handler will upgrade to a websocket connection
	// uuid is the key to find which address will be tunneled
	router.GET("/traffic/:uuid", services.TrafficHandler)

	adminRouter := router.Group("")
	adminRouter.Use(services.AdminRequired())
	{
		// get all the mappers
		adminRouter.GET("pool", services.GetMapperListHandler)
		// get logs of this mapper
		adminRouter.GET("pool/:uuid", services.GetMapperHandler)
		// create a new mapper
		adminRouter.POST("pool", services.CreateMapperHandler)
		// delete a mapper
		adminRouter.DELETE("pool/:uuid", services.DeleteMapperHandler)
	}
	return router, nil
}
