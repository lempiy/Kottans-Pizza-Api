package handlers

import (
	"github.com/labstack/echo"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/handlers/ws"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/room"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/utils"
)

//Run - inits and fills app router with handlers.
func Run(r *echo.Router, cluster *room.Cluster, storage *utils.Storage) {
	go room.NewPublisher(cluster, storage).Watch()
	r.Add("GET", "/ws", ws.Handle(cluster, storage))
}
