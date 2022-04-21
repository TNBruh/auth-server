package routes

import (
	"auth-server/models"
	"auth-server/services"
	// "fmt"
	"net/http"

	"github.com/gin-gonic/gin"
)

func loginHandler(c *gin.Context) {
	//parse data
	var user models.User
	err := c.BindJSON(&user)
	if err != nil {
		c.IndentedJSON(http.StatusInternalServerError, gin.H{"message": err.Error()})
		return
	}

	//login
	userData, err := services.HostInstance.Login(&user)
	if err != nil {
		c.IndentedJSON(http.StatusInternalServerError, gin.H{"message": err.Error()})
		return
	}

	//remove password field
	delete(userData, "password")

	//write to response
	c.IndentedJSON(http.StatusOK, userData)
}

func refreshHandler(c *gin.Context) {

}

func logoutHandler(c *gin.Context) {

}

func registerHandler(c *gin.Context) {

}

func verificationHandler(c *gin.Context) {

}

func Setup() *gin.Engine {
	route := gin.Default()

	route.POST("/auth", loginHandler)

	return route
}
