package main

import (
	"github.com/gomodule/redigo/redis"
	"reflect"
	"time"
	// "strings"
	"strconv"

	"fmt"
)

type Foo struct {
	Number  int64     `json:"number"  redis:"number"`
	Atime   time.Time `json:"atime"   redis:"atime"`
	Astring string    `json:"astring" redis:"astring"`
}

func main() {
	c, err := redis.Dial("tcp", ":6379")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer c.Close()

	t1 := time.Now().UTC()
	var foo Foo
	foo.Number = 10000000000
	foo.Atime = t1
	foo.Astring = "Hello"

	tmp := redis.Args{}.Add("id1").AddFlat(&foo)
	if _, err := c.Do("HMSET", tmp...); err != nil {
		fmt.Println(err)
		return
	}

	v, err := redis.StringMap(c.Do("HGETALL", "id1"))
	if err != nil {
		fmt.Println(err)
		return
	}
	fmt.Printf("%#v\n", v)

	if _, err := c.Do("HMSET", "id1", "atime", t1.Add(-time.Hour*(60*60*24))); err != nil {
		fmt.Println(err)
		return
	}

	var foo2 Foo
	mapToStruct(v, &foo2)
	fmt.Printf("%#v\n", foo2)
}

func mapToStruct(src map[string]string, dst interface{}) error {
	re := reflect.ValueOf(dst).Elem()
	for k, v := range src {
		key := []byte(k)
		key[0] -= 32
		rf := re.FieldByName(string(key))
		switch rf.Interface().(type) {
		case time.Time:
			format := "2006-01-02 15:04:05 -0700 MST"
			ti, err := time.Parse(format, v)
			if err != nil {
				return err
			}
			rf.Set(reflect.ValueOf(ti))
		case int, int64:
			x, err := strconv.ParseInt(v, 10, rf.Type().Bits())
			if err != nil {
				return err
			}
			rf.SetInt(x)
		default:
			rf.SetString(v)
		}
	}

	return nil
}
