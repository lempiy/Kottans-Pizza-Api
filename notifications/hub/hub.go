package hub

const (
	remove commandAction = iota
)

type commandAction int

type commandData struct {
	action  commandAction
	uuid    string
	storeId int
	data    []byte
}

type Hub struct {
	hubListener chan commandData
}
