package models

import (
//"github.com/gin-gonic/gin"
)

type getResource[T interface{}] interface {
	Get(T) (map[string]interface{}, error)
}

type postResource[T interface{}] interface {
	Post(T) (map[string]interface{}, error)
}

type updateResource[T interface{}] interface {
	Update(T) (map[string]interface{}, error)
}

type deleteResource[T interface{}] interface {
	Delete(T) (map[string]interface{}, error)
}
