package room

import (
	"github.com/json-iterator/go"
	"github.com/lempiy/Kottans-Pizza-Api/notifications/utils"
	"log"
)

const NotificationThreadName = "NOTIFICATION"

type Publisher struct {
	*Cluster
	ps  utils.PubSub
	die chan struct{}
}

type Message struct {
	StoreID int                 `json:"store_id"`
	Payload jsoniter.RawMessage `json:"payload"`
}

func NewPublisher(cluster *Cluster, ps utils.PubSub) *Publisher {
	return &Publisher{
		Cluster: cluster,
		ps:      ps,
		die:     make(chan struct{}),
	}
}

func (p *Publisher) Watch() {
	onmessage := p.ps.On(NotificationThreadName)
	go func() {
		for {
			select {
			case msg := <-onmessage:
				var message Message
				if err := jsoniter.Unmarshal(msg, &message); err != nil {
					log.Println("Publisher.Watch err: ", err)
				}
				p.Emit(message.Payload, message.StoreID)
			case <-p.die:
				return
			}
		}
	}()
}

func (p *Publisher) Die() {
	p.die <- struct{}{}
}
