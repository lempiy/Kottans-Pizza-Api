package main

import (
	"github.com/labstack/echo"
	"github.com/labstack/echo/middleware"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/handlers"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/room"
	"os"
)

func main() {
	e := echo.New()
	e.Use(middleware.Logger())
	e.Use(middleware.Recover())
	cluster := room.NewCluster()
	PORT := os.Getenv("PORT")
	r := e.Router()
	handlers.Run(r, cluster)
	e.Logger.Fatal(e.Start(":" + PORT))
}
