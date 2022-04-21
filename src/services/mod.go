package services

func Setup() {
	HostInstance = &Host{
		"http",
		"localhost",
		9000,
		"sussy",
		mainServer,
		"auth/user/:id",
		"auth/user",
	}
}
