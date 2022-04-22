package models

import (
	"auth-server/services"
	"auth-server/utils"
	"encoding/json"
	"fmt"
	"net/http"
	"strings"

	"github.com/gomodule/redigo/redis"
)

type User struct {
	Identifier string `json:"identifier"`
	Password   string `json:"password"`
}

func (u *User) Get(input interface{}, rawRoute string, authorizationHeader string) (map[string]interface{}, error) {
	// hostIns := services.HostInstance
	// unspecifiedRoute := hostIns.CraftUserRoute()

	route := strings.ReplaceAll(rawRoute, ":id", u.Identifier)

	req, err := http.NewRequest("GET", route, nil)
	if err != nil {
		return map[string]interface{}{}, fmt.Errorf("somehow unable to create request")
	}

	req.Header.Set("Authorization", authorizationHeader)
	client := &http.Client{}
	res, err := client.Do(req)
	if err != nil {
		return map[string]interface{}{}, fmt.Errorf("unable to send request")
	}

	var body map[string]interface{}
	json.NewDecoder(res.Body).Decode(&body)

	return body, nil

}

func (u *User) Login() (map[string]interface{}, error) {
	userData, err := u.Get(nil, services.HostInstance.GetUserRoute(), services.HostInstance.Password)
	if err != nil {
		return map[string]interface{}{}, err
	}

	pwHash, ok := userData["password"].(string)
	if !ok {
		return map[string]interface{}{}, fmt.Errorf("unable to parse password from userData")
	}

	isUser := utils.BcryptInstance.Compare(pwHash, u.Password)
	if isUser {
		//create token here, then append
		accessToken, refreshToken, err := utils.JwtInstance.CreateAccessRefresh(userData)
		if err != nil {
			return map[string]interface{}{}, err
		}
		userData["access_token"] = accessToken
		userData["refresh_token"] = refreshToken

		//stores refresh token in cache
		if utils.JwtInstance.GlobalLogout {
			r := *(services.RedisInstance)
			_, err := r.Do("SET", userData["id"].(string), refreshToken)
			if err != nil {
				return map[string]interface{}{}, err
			}
		}
		return userData, nil
	} else {
		return map[string]interface{}{}, fmt.Errorf("sussy impostor")
	}
}

func (u *User) GlobalCacheCheck(subId string, rToken string) bool {
	r := *(services.RedisInstance)

	_, err := redis.String(r.Do("GET", subId))
	if err != nil {
		return false
	}

	_, e := r.Do("SET", subId, rToken)
	if e != nil {
		return false
	}

	return true
}
