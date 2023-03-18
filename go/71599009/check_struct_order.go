// https://stackoverflow.com/questions/18926303/iterate-through-the-fields-of-a-struct-in-go
// https://go.dev/play/p/gSnZ8mLll-
package main

import (
	"fmt"
	"reflect"
)

func main() {
	// check struct field order
	x := struct {
		Foo string
		Bar int
	}{"foo", 2}

	t := reflect.TypeOf(x)
	v := reflect.ValueOf(x)

	for i := 0; i < v.NumField(); i++ {
		key, value := t.Field(i).Name, v.Field(i).Interface()
		fmt.Printf("Key: %v\nValue: %v\n\n", key, value)
	}
}
