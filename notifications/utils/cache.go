package utils

import (
	"fmt"
	"github.com/garyburd/redigo/redis"
	"github.com/json-iterator/go"
	"log"
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

type KeyStorage struct {
	conn redis.Conn
}

func NewKeyStorage(conn redis.Conn) *KeyStorage {
	return &KeyStorage{
		conn: conn,
	}
}

type UserData struct {
	StoreId  int
	UUID     string
	Username string
	Token    string
}

func (ks *KeyStorage) GetValue(key string) (*UserData, error) {
	data, err := ks.conn.Do("GET", key)
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

func (ks *KeyStorage) RemoveValue(key string) error {
	_, err := ks.conn.Do("DEL", key)
	if err != nil {
		log.Println(err)
		return err
	}
	return nil
}
