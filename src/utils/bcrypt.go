package utils

import (
	"golang.org/x/crypto/bcrypt"
)

var BcryptInstance *bcryptInstance

type bcryptInstance struct {
	MinCost     uint
	MaxCost     uint
	DefaultCost uint
}

func (b *bcryptInstance) Compare(hash string, password string) bool {
	hashByte := []byte(hash)
	passwordByte := []byte(password)

	err := bcrypt.CompareHashAndPassword(hashByte, passwordByte)
	return err == nil
}
