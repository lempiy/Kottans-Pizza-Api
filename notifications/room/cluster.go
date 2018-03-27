package room

type commandPayload struct {
	action commandAction
	id     int
	result <-chan *Hub
	data   []byte
	hub    *Hub
}

type Cluster struct {
	listener chan commandPayload
	pool     map[int]*Hub
}

func NewCluster() *Cluster {
	cluster := Cluster{
		listener: make(chan commandPayload),
		pool:     make(map[int]*Hub),
	}
	go cluster.run()
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
				hub.Emit(command.data)
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

func (cluster *Cluster) Emit(msg []byte) {
	cluster.listener <- commandPayload{
		action: emit,
		data:   msg,
	}
}

func (cluster *Cluster) Die() {
	cluster.listener <- commandPayload{
		action: die,
	}
}
