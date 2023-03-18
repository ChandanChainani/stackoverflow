package main

import (
	"flag"
	"fmt"
	"os"
	"testing"
	// "log"
	// "io/ioutil"
)

var p = flag.Bool("p", false, "Enable Local Logging")

func MyLog(args ...interface{}) {
	if *p {
		fmt.Fprintln(os.Stdout, args...)
	}
}

//  func TestMain(m *testing.M) {
//      log.SetOutput(ioutil.Discard)
//      // setup()
//      // code := m.Run()
//      m.Run()
//      // shutdown()
//      // os.Exit(code)
//  }

func IntMin(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func TestIntMinBasic(t *testing.T) {
	ans := IntMin(2, -2)
	if ans != -2 {

		t.Errorf("IntMin(2, -2) = %d; want -2", ans)
	}
}

func TestIntMinTableDriven(t *testing.T) {
	var tests = []struct {
		a, b int
		want int
	}{
		{0, 1, 0},
		{1, 0, 0},
		{2, -2, -2},
		{0, -1, -1},
		{-1, 0, -1},
	}

	MyLog("Printing")

	for _, tt := range tests {

		testname := fmt.Sprintf("%d,%d", tt.a, tt.b)
		t.Run(testname, func(t *testing.T) {
			ans := IntMin(tt.a, tt.b)
			if ans != tt.want {
				t.Errorf("got %d, want %d", ans, tt.want)
			}
		})
	}
}

func BenchmarkIntMin(b *testing.B) {
	for i := 0; i < b.N; i++ {
		IntMin(1, 2)
	}
}
