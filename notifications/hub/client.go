package hub

import "log"

type Client struct {
	send        chan<- []byte
	read        <-chan []byte
	token       string
	StoreId     int
	UUID        string
	die         <-chan struct{}
	hub         *Hub
	hubListener chan<- commandData
}

func NewClient(send chan<- []byte, read <-chan []byte, die <-chan struct{}, token, uuid string, storeId int) *Client {
	c := &Client{
		send:    send,
		read:    read,
		token:   token,
		StoreId: storeId,
		UUID:    uuid,
		die:     die,
	}
	go c.watch()
	return c
}

func (c *Client) attachToHub(hub *Hub) {
	if c.hub != nil {
		c.hubListener <- commandData{
			action: remove,
			uuid:   c.UUID,
		}
	}
	c.hub = hub
	c.hubListener = hub.hubListener
}

func (c *Client) watch() {
	for {
		select {
		case <-c.die:
			c.Die()
			return
		case msg := <-c.read:
			// Do stuff
			log.Println("Client "+c.UUID+" read: ", string(msg))
		}
	}
}

func (c *Client) Send(msg []byte) {
	c.send <- msg
}

func (c *Client) Die() {
	if c.hubListener != nil {
		c.hubListener <- commandData{
			action: remove,
			uuid:   c.UUID,
		}
	}
}
