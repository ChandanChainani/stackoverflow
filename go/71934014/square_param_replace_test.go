package main

import (
	"testing"
)

func Benchmark_ReplaceSquareBracketsWithDot(b *testing.B) {
	// run the Fib function b.N times
	for n := 0; n < b.N; n++ {
		replaceSquareBracketsWithDot("person.addresses[0][name][1]")
	}
}
