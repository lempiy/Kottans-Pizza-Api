package handlers

import (
	"github.com/labstack/echo"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/handlers/ws"
)

//Run - inits and fills app router with handlers.
func Run(r *echo.Router) {
	r.Add("GET", "/ws", ws.Handle)
}
