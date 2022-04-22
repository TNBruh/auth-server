package services

import "github.com/gomodule/redigo/redis"

func Setup() {
	HostInstance = &Host{
		"http",
		"localhost",
		9000,
		"sussy",
		mainServer,
		"auth/user/:id",
		"auth/user",
	}

	cRedis, err := redis.Dial("tcp", "localhost:6379", redis.DialPassword("amogus"))
	if err != nil {
		panic("cannot connect to redis")
	}
	RedisInstance = &cRedis
}
