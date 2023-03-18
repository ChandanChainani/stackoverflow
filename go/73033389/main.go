package main

import (
	// "encoding/binary"
	"fmt"

	// "github.com/roberson-io/mmh3"
	"github.com/reusee/mmh3"
	// "github.com/datadog/mmh3"
)

func main() {
	key := []byte("foo")
	// var seed uint32 = 0
	h := mmh3.New32()
	h.Write(key)
	// hash := mmh3.Hashx86_32(key, seed)
	// fmt.Printf("%x\n", binary.LittleEndian.Uint32(hash))
	fmt.Printf("%x\n", h.Sum(nil))
	// fmt.Printf("%x\n", mmh3.Hash32(key))
}
