package utils

import (
	"fmt"
	"github.com/garyburd/redigo/redis"
	"github.com/json-iterator/go"
	"log"
	"sync"
)

const RedisUrl = "redis://127.0.0.1:6379"

func NewRedisConnection() (redis.Conn, error) {
	conn, err := redis.DialURL(RedisUrl)
	if err != nil {
		return nil, err
	}
	return conn, err
}

type KeyHolder interface {
	GetValue(key string) (*UserData, error)
	RemoveValue(key string) error
}

type PubSub interface {
	On(channel string) chan []byte
	Off(channel string, ch chan []byte)
}

type Storage struct {
	pubSubMap map[string][]chan<- []byte
	conn      redis.Conn
	mx        *sync.Mutex
	lock      *sync.RWMutex
	pubSub    redis.PubSubConn
}

func NewStorage(conn redis.Conn, psConn redis.Conn) *Storage {
	s := Storage{
		pubSubMap: make(map[string][]chan<- []byte),
		conn:      conn,
		mx:        &sync.Mutex{},
		lock:      &sync.RWMutex{},
		pubSub:    redis.PubSubConn{Conn: psConn},
	}
	go s.listenPubSub()
	return &s
}

func (s *Storage) listenPubSub() error {
	for {
		switch v := s.pubSub.Receive().(type) {
		case redis.Message:
			s.lock.RLock()
			if channels, exist := s.pubSubMap[v.Channel]; exist {
				for _, ch := range channels {
					ch <- v.Data
				}
			}
			s.lock.RUnlock()
		case redis.Subscription:
			log.Printf("%s: %s %d\n", v.Channel, v.Kind, v.Count)
		case error:
			log.Println("listenPubSub err: ", v)
			return v
		}
	}
}

func (s *Storage) On(channel string) chan []byte {
	ch := make(chan []byte, 10)
	s.lock.RLock()
	if channels, exist := s.pubSubMap[channel]; exist {
		channels = append(channels, ch)
	} else {
		s.pubSubMap[channel] = []chan<- []byte{ch}
	}
	s.lock.RUnlock()
	s.mx.Lock()
	s.pubSub.Subscribe(channel)
	s.mx.Unlock()
	return ch
}

func (s *Storage) Off(channel string, ch chan []byte) {
	s.lock.RLock()
	if channels, exist := s.pubSubMap[channel]; exist {
		found := -1
		for i, c := range channels {
			if c == ch {
				found = i
			}
		}
		if found != -1 {
			channels = append(channels[:found], channels[found+1:]...)
		}
	}
	s.lock.RUnlock()
	s.mx.Lock()
	s.pubSub.Unsubscribe(channel)
	s.mx.Unlock()
}

type UserData struct {
	StoreId int    `json:"store_id"`
	UUID    string `json:"user_uuid"`
	Token   string `json:"token"`
}

func (s *Storage) GetValue(key string) (*UserData, error) {
	s.mx.Lock()
	data, err := s.conn.Do("GET", key)
	s.mx.Unlock()
	if err != nil {
		log.Println(err)
		return nil, err
	}
	bts, success := data.([]byte)
	if !success {
		log.Println("GetValue", "wrong data type returned from redis")
		return nil, fmt.Errorf("cannot convert value by key %s to string", key)
	}
	var ud UserData
	return &ud, jsoniter.Unmarshal(bts, &ud)
}

func (s *Storage) RemoveValue(key string) error {
	s.mx.Lock()
	_, err := s.conn.Do("DEL", key)
	s.mx.Unlock()
	if err != nil {
		log.Println(err)
		return err
	}
	return nil
}
