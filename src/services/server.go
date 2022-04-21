package services

import (
	"auth-server/models"
	"auth-server/utils"
	"fmt"
)

var HostInstance *Host

type serverType uint

const (
	redis serverType = iota
	memcached
	mainServer
	localStorage
)

type Host struct {
	protocol          string
	ip                string
	port              uint
	Password          string
	Mode              serverType
	getUserRoute      string
	registerUserRoute string
}

func (h *Host) GetUserRoute() string {
	return fmt.Sprintf("%v://%v:%v/%v", h.protocol, h.ip, h.port, h.getUserRoute)
}

func (h *Host) Login(data *models.User) (map[string]interface{}, error) {

	userData, err := data.Get(nil, HostInstance.GetUserRoute(), HostInstance.Password)
	if err != nil {
		return map[string]interface{}{}, err
	}

	pwHash, ok := userData["password"].(string)
	if !ok {
		return map[string]interface{}{}, fmt.Errorf("unable to parse password from userData")
	}

	isUser := utils.BcryptInstance.Compare(pwHash, data.Password)

	if isUser {
		return userData, nil
	} else {
		return map[string]interface{}{}, fmt.Errorf("sussy impostor")
	}
}
