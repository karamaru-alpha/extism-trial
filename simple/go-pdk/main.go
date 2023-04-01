package main

import (
	"fmt"

	"github.com/extism/go-pdk"
)

//export count_vowels
func count_vowels() int32 {
	input := pdk.Input()

	count := 0
	for _, a := range input {
		switch a {
		case 'A', 'I', 'E', 'O', 'U', 'a', 'e', 'i', 'o', 'u':
			count++
		default:
		}
	}

	output := fmt.Sprintf(`{"count": %d, "input": "%s"}`, count, string(input))
	mem := pdk.AllocateString(output)

	// zero-copy output to host
	pdk.OutputMemory(mem)

	return 0
}

func main() {}
