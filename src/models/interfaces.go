package models

import (
//"github.com/gin-gonic/gin"
)

type getResource[T interface{}, U interface{}] interface {
	get(T) U
}

type postResource[T interface{}, U interface{}] interface {
	post(T) U
}

type updateResource[T interface{}, U interface{}] interface {
	update(T) U
}

type deleteResource[T interface{}, U interface{}] interface {
	delete(T) U
}
