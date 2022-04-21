package services

func Setup() {

}

type serverType uint

const (
	redis serverType = iota
	memcached
	mainServer
	localStorage
)

type Host struct {
	ip                string
	port              uint
	password          string
	mode              serverType
	getUserRoute      string
	registerUserRoute string
}
