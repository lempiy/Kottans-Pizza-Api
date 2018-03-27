package room

const (
	remove commandAction = iota
	emit
	add
	get
	die
)

type commandAction int

type commandData struct {
	action  commandAction
	uuid    string
	storeId int
	result  <-chan *Client
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
			hub.pool[command.client.UUID] = command.client
		case get:
			command.result <- hub.pool[command.uuid]
		case remove:
			delete(hub.pool, command.uuid)
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

func (hub *Hub) Get(uuid string) *Client {
	result := make(chan *Client)
	hub.listener <- commandData{
		action: get,
		uuid:   uuid,
		result: result,
	}
	return <-result
}

func (hub *Hub) Remove(uuid string) {
	hub.listener <- commandData{
		action: remove,
		uuid:   uuid,
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
