package room

const (
	remove commandAction = iota
	emit
	add
	get
	length
	die
)

type commandAction int

type commandData struct {
	action  commandAction
	key     string
	storeId int
	result  chan<- *Client
	length  chan<- int
	data    []byte
	client  *Client
}

type Hub struct {
	listener chan commandData
	pool     map[string]*Client
	ID       int
}

func NewHub(id int) *Hub {
	hub := Hub{
		listener: make(chan commandData),
		pool:     make(map[string]*Client),
		ID:       id,
	}
	go hub.run()
	return &hub
}

func (hub *Hub) run() {
	for command := range hub.listener {
		switch command.action {
		case add:
			hub.pool[command.client.Key] = command.client
			command.client.attachToHub(hub)
		case get:
			command.result <- hub.pool[command.key]
		case remove:
			delete(hub.pool, command.key)
		case length:
			command.length <- len(hub.pool)
		case emit:
			for _, client := range hub.pool {
				client.Send(command.data)
			}
		case die:
			return
		}
	}
}

func (hub *Hub) Add(client *Client) {
	hub.listener <- commandData{
		action: add,
		client: client,
	}
}

func (hub *Hub) Get(key string) *Client {
	result := make(chan *Client)
	hub.listener <- commandData{
		action: get,
		key:    key,
		result: result,
	}
	return <-result
}

func (hub *Hub) Length() int {
	length := make(chan int)
	hub.listener <- commandData{
		action: get,
		length: length,
	}
	return <-length
}

func (hub *Hub) Remove(key string) {
	hub.listener <- commandData{
		action: remove,
		key:    key,
	}
}

func (hub *Hub) Emit(msg []byte) {
	hub.listener <- commandData{
		action: emit,
		data:   msg,
	}
}

func (hub *Hub) Die() {
	hub.listener <- commandData{
		action: die,
	}
}
