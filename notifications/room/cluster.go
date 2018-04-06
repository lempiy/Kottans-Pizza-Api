package room

import (
	"os"
	"os/signal"
	"time"
)

type commandPayload struct {
	action commandAction
	id     int
	result chan<- *Hub
	data   []byte
	hub    *Hub
}

type Cluster struct {
	isDying  bool
	listener chan commandPayload
	pool     map[int]*Hub
}

func NewCluster() *Cluster {
	cluster := Cluster{
		listener: make(chan commandPayload),
		pool:     make(map[int]*Hub),
	}
	go cluster.run()
	go func() {
		interrupt := make(chan os.Signal, 1)
		signal.Notify(interrupt, os.Interrupt)
		<-interrupt
		time.Sleep(time.Second * 2)
		os.Exit(0)
	}()
	return &cluster
}

func (cluster *Cluster) run() {
	for command := range cluster.listener {
		switch command.action {
		case add:
			cluster.pool[command.hub.ID] = command.hub
		case get:
			command.result <- cluster.pool[command.id]
		case remove:
			delete(cluster.pool, command.id)
		case emit:
			for _, hub := range cluster.pool {
				if hub.ID == command.id {
					hub.Emit(command.data)
					break
				}
			}
		case die:
			return
		}
	}
}

func (cluster *Cluster) Add(hub *Hub) {
	cluster.listener <- commandPayload{
		action: add,
		hub:    hub,
	}
}

func (cluster *Cluster) Get(id int) *Hub {
	result := make(chan *Hub)
	cluster.listener <- commandPayload{
		action: get,
		id:     id,
		result: result,
	}
	return <-result
}

func (cluster *Cluster) Remove(id int) {
	cluster.listener <- commandPayload{
		action: remove,
		id:     id,
	}
}

func (cluster *Cluster) Emit(msg []byte, hubID int) {
	cluster.listener <- commandPayload{
		action: emit,
		id:     hubID,
		data:   msg,
	}
}

func (cluster *Cluster) Die() {
	cluster.listener <- commandPayload{
		action: die,
	}
}
