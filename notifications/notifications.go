package main

import (
	"github.com/labstack/echo"
	"github.com/labstack/echo/middleware"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/handlers"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/utils"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/room"
	"os"
	"log"
)

func main() {
	e := echo.New()
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	redisConn, err := utils.NewRedisConnection()
	if err != nil {
		log.Fatal(err)
	}
	keyStorage := utils.NewKeyStorage(redisConn)
	cluster := room.NewCluster()
	PORT := os.Getenv("PORT")
	r := e.Router()
	handlers.Run(r, cluster, keyStorage)
	e.Logger.Fatal(e.Start(":" + PORT))
}
