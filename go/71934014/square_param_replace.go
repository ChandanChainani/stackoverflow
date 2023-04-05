package main

import (
	"fmt"
	// "bytes"
)

func replaceSquareBracketsWithDot(s string) string {
	b := []byte(s)

	// fmt.Println(len(b))
	if b[len(b)-1] == ']' {
		b[len(b)-1] = 0
	}
	if b[0] == '[' {
		b[0] = 0
	}
	// fmt.Println(len(b))

	for i, j := 0, len(b)-1; i <= j; i, j = i+1, j-1 {
		// fmt.Println(i, j)
		// if b[j] == 91 || b[j] == 93 {
		//   b[j] = '.'
		// }
		if b[i] == ']' || b[i] == '[' {
			b[i] = '.'
		}

		if b[i+1] == ']' || b[i+1] == '[' {
			b[i+1] = '.'
		}

		if b[i] == '.' && b[i+1] == '.' {
			b[i] = 0
		}

		if b[j] == '[' || b[j] == ']' {
			b[j] = '.'
		}

		if b[j-1] == '[' || b[j-1] == ']' {
			b[j-1] = '.'
		}

		if b[j] == '.' && b[j-1] == '.' {
			b[j] = 0
		}
		// fmt.Printf("%v %v %v %v\n", i, i + 1, string(b[i]), string(b[i + 1]))
		// if b[i] == ']' && b[i + 1] == '.' {
		//   b[i] = 0
		// }

		// // fmt.Println(string(b))
		// fmt.Printf("%v %v %v %v\n", j, j + 1, string(b[j]), string(b[j - 1]))
		// if b[j] == '.' && b[j - 1] == ']' {
		//   b[j] = 0
		// }
		// fmt.Println(b[i], string(b[i]), b[j], string(b[j]))
	}

	// return string(bytes.ReplaceAll(b, []byte(".."), []byte(".")))
	return string(b)
}

func main() {
	arr := []string{
		"[0][name]",
		"[0].name",
		"person[0][name]",
		"person[0].name",
		"person.addresses[0].name",
		"person.addresses[0][name]",
		"person.addresses[0][name][1]",
	}
	for _, input := range arr {
		fmt.Printf("Input  : %v\nOutput : %v\n\n", input, replaceSquareBracketsWithDot(input))
	}
}
