package ws

import (
	"github.com/gorilla/websocket"
	"github.com/labstack/echo"
	"net/http"
	"os"
	"log"
	"os/signal"
	"fmt"
)

var (
	upgrader = websocket.Upgrader{
		ReadBufferSize: 1024 * 4,
		WriteBufferSize: 1024 * 4,
		CheckOrigin: func(r *http.Request) bool {
			return true
		},
	}
	closeErrorCodes = []int{
		websocket.CloseAbnormalClosure,
		websocket.CloseNormalClosure,
		websocket.CloseInternalServerErr,
	}
)

func Handle(c echo.Context) error {
	interrupt := make(chan os.Signal, 1)
	signal.Notify(interrupt, os.Interrupt)
	ws, err := upgrader.Upgrade(c.Response(), c.Request(), nil)
	if err != nil {
		return err
	}

	send := make(chan []byte)

	go func() {
		defer ws.Close()
		for {
			_, message, err := ws.ReadMessage()
			if err != nil {
				if websocket.IsCloseError(err, closeErrorCodes...) {
					ws.Close()
					return
				}
				ws.Close()
				return
			}
			fmt.Println(message)
		}
	}()

	for {
		select {
		case data := <-send:
			err := ws.WriteMessage(websocket.TextMessage, data)
			if err != nil {
				log.Println(err)
			}
		case <-interrupt:
			log.Println("Websocket server disconnect...")
			err := ws.WriteMessage(websocket.CloseMessage,
				websocket.FormatCloseMessage(websocket.CloseNormalClosure,
					"Server's gone down"))
			ws.Close()
			os.Exit(0)
			return err
		}
	}
}
