package main

import (
    "github.com/labstack/echo"
    "github.com/labstack/echo/middleware"
    "github.com/lempiy/Kottans-Pizza-Api/notifications/handlers"
    "os"
)

func main() {
    e := echo.New()
    e.Use(middleware.Logger())
    e.Use(middleware.Recover())
    PORT := os.Getenv("PORT")
    r := e.Router()
    handlers.Run(r)
    e.Logger.Fatal(e.Start(":" + PORT))
}
