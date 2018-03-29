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
	return &conn, err
}

type KeyHolder interface {
	GetValue(key string) (*UserData, error)
	RemoveValue(key string) error
}

type Storage struct {
	conn redis.Conn
	mx   *sync.Mutex
}

func NewKeyStorage(conn redis.Conn) *Storage {
	return &Storage{
		conn: conn,
		mx:   &sync.Mutex{},
	}
}

type UserData struct {
	StoreId  int
	UUID     string
	Username string
	Token    string
}

func (ks *Storage) GetValue(key string) (*UserData, error) {
	ks.mx.Lock()
	data, err := ks.conn.Do("GET", key)
	ks.mx.Unlock()
	if err != nil {
		log.Println(err)
		return nil, err
	}
	str, success := data.(string)
	if !success {
		log.Println("GetValue", "wrong data type returned from redis")
		return nil, fmt.Errorf("cannot convert value by key %s to string", key)
	}
	var ud *UserData
	return ud, jsoniter.UnmarshalFromString(str, ud)
}

func (ks *Storage) RemoveValue(key string) error {
	ks.mx.Lock()
	_, err := ks.conn.Do("DEL", key)
	ks.mx.Unlock()
	if err != nil {
		log.Println(err)
		return err
	}
	return nil
}
