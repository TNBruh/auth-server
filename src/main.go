package main

import (
	"auth-server/routes"
	"auth-server/services"
	"auth-server/utils"
)

func main() {
	ginEngine := routes.Setup()
	services.Setup()
	utils.Setup()

	ginEngine.Run()
}
