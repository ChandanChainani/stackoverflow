package main

import (
	"fmt"
	"log"

	"net/url"

	"github.com/go-playground/form"
)

func main() {
	v := url.Values{}
	v.Add("friend", "Jess")
	v.Add("friend", "Sarah")
	v.Add("friend", "Zoe")
	fmt.Printf("%#v\n", v)
	e := v.Encode()
	fmt.Println(e)
	m, _ := url.ParseQuery(e)
	fmt.Printf("%+v\n\n", m)

	v = url.Values{}
	v.Add("friend[]", "Jess")
	v.Add("friend[]", "Sarah")
	v.Add("friend[]", "Zoe")
	fmt.Printf("%#v\n", v)
	e = v.Encode()
	fmt.Println(e)
	m, _ = url.ParseQuery(e)
	fmt.Printf("%+v\n\n", m)

	v = url.Values{}
	v.Add("friend[name]", "Jess")
	v.Add("friend[name]", "Sarah")
	v.Add("friend[name]", "Zoe")
	fmt.Printf("%#v\n", v)
	e = v.Encode()
	fmt.Println(e)
	m, _ = url.ParseQuery(e)
	fmt.Printf("%+v\n\n", m)

	v = url.Values{}
	v.Add("friend[][name]", "Jess")
	v.Add("friend[][name]", "Sarah")
	v.Add("friend[][name]", "Zoe")
	fmt.Printf("%#v\n", v)
	e = v.Encode()
	fmt.Println(e)
	m, _ = url.ParseQuery(e)
	fmt.Printf("%+v\n\n", m)

	v = url.Values{}
	v.Add("friend[0][name]", "Jess")
	v.Add("friend[1][name]", "Sarah")
	v.Add("friend[2][name]", "Zoe")
	fmt.Printf("%#v\n", v)
	e = v.Encode()
	fmt.Println(e)
	m, _ = url.ParseQuery(e)
	fmt.Printf("%+v\n\n", m)

	v = url.Values{}
	v.Add("friend[0]", "Jess")
	v.Add("friend[1]", "Sarah")
	v.Add("friend[2]", "Zoe")
	fmt.Printf("%#v\n", v)
	e = v.Encode()
	fmt.Println(e)
	m, _ = url.ParseQuery(e)
	fmt.Printf("%+v\n\n", m)

	v = url.Values{}
	v.Add("friend[0]", "Jess")
	v.Add("friend[1]", "Sarah")
	v.Add("friend[2]", "Zoe")
	fmt.Printf("%#v\n", v)
	e = v.Encode()
	fmt.Println(e)
	m, _ = url.ParseQuery(e)
	fmt.Printf("%+v\n\n", m)

	result := []byte{0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x73, 0x5b, 0x30, 0x5d, 0x2e, 0x6e, 0x61, 0x6d, 0x65, 0x3d, 0x6f, 0x6e, 0x65, 0x26, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x73, 0x5b, 0x30, 0x5d, 0x2e, 0x70, 0x61, 0x73, 0x73, 0x3d, 0x31, 0x26, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x73, 0x5b, 0x31, 0x5d, 0x2e, 0x6e, 0x61, 0x6d, 0x65, 0x3d, 0x74, 0x77, 0x6f, 0x26, 0x70, 0x65, 0x72, 0x73, 0x6f, 0x6e, 0x73, 0x5b, 0x31, 0x5d, 0x2e, 0x70, 0x61, 0x73, 0x73, 0x3d, 0x32}
	v = url.Values{}
	v.Add("persons[0].name", "one")
	v.Add("persons[0].pass", "1")
	v.Add("persons[1].name", "two")
	v.Add("persons[1].pass", "2")
	fmt.Printf("%#v\n", v)
	e = v.Encode()
	fmt.Println(e)
	m, _ = url.ParseQuery(e)
	fmt.Printf("%+v\n\n", m)
	b := []byte(e)
	fmt.Println("Required: ", result)
	fmt.Printf("Current  : %+v\n\n", b)
	b = []byte("persons[0].name=one&persons[0].pass=1&persons[1].name=two&persons[1].pass=2")
	fmt.Printf("Current 2: %+v\n\n", b)
	fmt.Println(string(result))
	// if result == b {
	//   fmt.Println("HERE")
	// }

	v = url.Values{}
	v.Add("friend[0].name", "Jess")
	v.Add("friend[1].name", "Sarah")
	v.Add("friend[2].name", "Zoe")
	fmt.Printf("%#v\n", v)
	e = v.Encode()
	fmt.Println(e)
	m, _ = url.ParseQuery(e)
	fmt.Printf("%+v\n\n", m)

	values := parseForm()
	fmt.Println(values)

	decoder = form.NewDecoder()
	var user User

	// must pass a pointer
	err := decoder.Decode(&user, values)
	if err != nil {
		log.Panic(err)
	}

	fmt.Printf("%#v\n", user)
}

type Address struct {
	Name  string
	Phone string
}

type User struct {
	Name        string
	Age         uint8
	Gender      string
	Address     []Address
	Active      bool `form:"active"`
	MapExample  map[string]string
	NestedMap   map[string]map[string]string
	NestedArray [][]string
}

var decoder *form.Decoder

func parseForm() url.Values {
	return url.Values{
		"Name":                []string{"joeybloggs"},
		"Age":                 []string{"3"},
		"Gender":              []string{"Male"},
		"Address[0].Name":     []string{"26 Here Blvd."},
		"Address[0].Phone":    []string{"9(999)999-9999"},
		"Address[1].Name":     []string{"26 There Blvd."},
		"Address[1].Phone":    []string{"1(111)111-1111"},
		"active":              []string{"true"},
		"MapExample[key]":     []string{"value"},
		"NestedMap[key][key]": []string{"value"},
		"NestedArray[0][0]":   []string{"value"},
	}
}
