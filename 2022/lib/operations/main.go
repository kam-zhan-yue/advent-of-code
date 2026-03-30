package operations

type Operation int
const (
	Add Operation = iota
	Subtract
	Multiply
	Divide
	Square
)

func Parse(s string) Operation {
	switch s {
		case "*": return Multiply
		case "-": return Subtract
		case "/": return Divide
		default: return Add
	}
}
