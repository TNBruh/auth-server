package utils

func Setup() {
	BcryptInstance = &bcryptInstance{
		8,
		14,
		10,
	}
}
