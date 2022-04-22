package utils

import (
	"fmt"
	"time"

	"github.com/golang-jwt/jwt"
)

var JwtInstance *jwtInstance

type jwtInstance struct {
	key          string
	refreshAge   uint
	accessAge    uint
	GlobalLogout bool
}

func (inst *jwtInstance) Validate(refreshToken string) (map[string]interface{}, error) {
	token, err := jwt.Parse(refreshToken, func(token *jwt.Token) (interface{}, error) {
		// Don't forget to validate the alg is what you expect:
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, fmt.Errorf("unexpected signing method: %v", token.Header["alg"])
		}

		// hmacSampleSecret is a []byte containing your secret, e.g. []byte("my_secret_key")
		return []byte(inst.key), nil
	})
	if err != nil {
		return map[string]interface{}{}, err
	}

	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok || !token.Valid {
		return map[string]interface{}{}, fmt.Errorf("unable to map or invalid token")
	}
	return claims, nil

}

func (inst *jwtInstance) Sign(bodyData map[string]interface{}) (string, error) {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims(bodyData))

	tokenString, err := token.SignedString(inst.key)
	if err != nil {
		return "", err
	}
	return tokenString, nil
}

func (inst *jwtInstance) Refresh(refreshToken string, validateAndRefresh func(subId string, rTkn string) bool) (string, string, error) {
	//body of old refresh token
	bodyData, err := inst.Validate(refreshToken)

	//current time
	currentTime := time.Now().Unix()
	if err != nil { //check if token had expired or not
		return "", "", err
	} else if bodyData["exp"].(int) < int(currentTime) {
		return "", "", fmt.Errorf("expired token")
	}
	//tokens' respective ages
	aExp, rExp := currentTime+int64(inst.accessAge), currentTime+int64(inst.refreshAge)

	//new refresh token's body
	refreshBody := map[string]interface{}{
		"sub":  bodyData["sub"].(string),
		"role": bodyData["role"].(byte),
		"exp":  rExp,
	}
	//new access token's body
	accessBody := bodyData
	accessBody["exp"] = aExp

	//signed refresh token
	tokenizedRefresh, err := inst.Sign(refreshBody)
	if err != nil {
		return "", "", err
	}
	//signed access token
	tokenizedAccess, err := inst.Sign(accessBody)
	if err != nil {
		return "", "", err
	}

	//if the logout is global, then we need to have a jwt cache and check it
	if inst.GlobalLogout {
		//use the custom function to validate the cache
		success := validateAndRefresh(bodyData["sub"].(string), refreshToken)
		if !success {
			return "", "", fmt.Errorf("user had logged out")
		}
	}

	return tokenizedAccess, tokenizedRefresh, nil
}

func (inst *jwtInstance) CreateAccessRefresh(bodyData map[string]interface{}) (string, string, error) {
	//current time
	currentTime := time.Now().Unix()

	//tokens' respective ages
	aExp, rExp := currentTime+int64(inst.accessAge), currentTime+int64(inst.refreshAge)

	refreshBody := map[string]interface{}{
		"sub":  bodyData["id"],
		"role": bodyData["role"],
		"exp":  rExp,
	}
	accessBody := bodyData
	accessBody["sub"] = bodyData["id"]
	accessBody["exp"] = aExp

	refreshToken, err := inst.Sign(refreshBody)
	if err != nil {
		return "", "", err
	}
	accessToken, err := inst.Sign(accessBody)
	if err != nil {
		return "", "", err
	}

	return refreshToken, accessToken, nil
}
