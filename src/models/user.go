package models

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strings"
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
