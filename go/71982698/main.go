package main

import (
	"fmt"

	"reflect"
	"strconv"
	"strings"
	"time"

	"github.com/gomodule/redigo/redis"
)

type Foo struct {
	// Number  int       `json:"number"  redis:"number"`
	Number  int64     `json:"number"  redis:"number"`
	ATime   time.Time `json:"atime"   redis:"atime"`
	AString string    `json:"astring" redis:"astring"`
}

func main() {
	c, err := redis.Dial("tcp", ":6379")
	if err != nil {
		fmt.Println(err)
		return
	}
	// c := redis.NewClient(&redis.Options{
	//   Addr:     "localhost:6379", // use default Addr
	//   Password: "",               // no password set
	//   DB:       0,                // use default DB
	// })
	defer c.Close()

	var foo Foo

	foo.Number = 10000000000
	foo.ATime = time.Now().UTC()
	foo.AString = "Hello"

	tmp := redis.Args{}.Add("id1").AddFlat(&foo)
	// fmt.Printf("SET %#v\n", tmp)
	if _, err := c.Do("HMSET", tmp...); err != nil {
		fmt.Println(err)
		return
	}

	t2 := time.Now().UTC()
	m := map[string]interface{}{
		"number":  99999999999,
		"atime":   t2,
		"astring": "Map",
	}

	if _, err := c.Do("HMSET", redis.Args{}.Add("id2").AddFlat(m)...); err != nil {
		fmt.Println(err)
		return
	}

	// newTime := time.Date(1980, time.April, 24, 18, 20, 15, 872233100, time.UTC)
	// fmt.Println(t2.Sub(newTime))
	if _, err := c.Do("HMSET", "id2", "atime", t2.Add(-time.Hour*(60*60*24))); err != nil {
		fmt.Println(err)
		return
	}

	for _, id := range []string{"id1", "id2"} {

		// fmt.Println(c.HGetAll(c.Context(), id))
		// v, err := redis.Values(c.Do("HGETALL", id))
		// if err != nil {
		//   fmt.Println(err)
		//   return
		// }
		// fmt.Printf("%#v\n", v)
		// fmt.Println(v.(map[string]interface{}))

		/// tmp := &Foo{}
		/// r := reflect.ValueOf(tmp).Elem()
		/// // var ti interface{}
		/// for i := 0; i < len(v); i = i+2 {
		///   // fmt.Printf("%T %v\n", v[i], v[i])
		///   // fmt.Printf("%T %v\n", v[i + 1], v[i + 1])
		///   key, _ := v[i].([]byte)
		///   key[0] -= 32
		///   val := v[i + 1].([]byte)
		///   ks := string(key)
		///   kv := string(val)

		///   // fmt.Println(ks, kv)
		///   rv := r.FieldByName(ks)

		///   // fmt.Printf("TYPE: %s %T %s\n", rv.Type(), rv.Type(), rv.Type().String())
		///   // fmt.Printf("RV: %#v\n", rv)
		///   // fmt.Println(reflect.ValueOf(val).Type())
		///   // var err error
		///   switch rv.Type().Name() {
		///     case "int", "int32", "int64":
		///       var x int64
		///       x, _ = strconv.ParseInt(kv, 10, rv.Type().Bits())
		///       rv.SetInt(x)
		///     case "string":
		///       rv.SetString(kv)
		///     case "Time":
		///       format := "2006-01-02 15:04:05 -0700 MST"
		///       // kv = kv.Format(time.RFC3339)
		///       var ti interface{}
		///       ti, _ = time.Parse(format, kv)
		///       // fmt.Println("TIME: ", ti)
		///       rv.Set(reflect.ValueOf(ti))
		///   }
		///   // fmt.Println("INTERF", rv.Type().Name(), err)
		///   // if ks == "atime" {
		///   //   // format := "2022-04-25 20:24:42.978943668 +0530 IST m=+0.000447455"
		///   //   // format := "2022-04-25 15:30:09.80449896 +0000 UTC"
		///   //   format := "2006-01-02 15:04:05 -0700 MST"
		///   //   t, _ := time.Parse(format, val.(string))
		///   //   ti = t
		///   //   // fmt.Println(t)
		///   //   // v[i + 1] = t
		///   //   // fmt.Println(time.Parse(time.RFC3339, string(val)))
		///   //   // fmt.Println(reflect.ValueOf(t))
		///   // } else if ks == "number" {
		///   //   ti = val.(int)
		///   // } else {
		///   //   ti = val.(string)
		///   // }
		///   // fmt.Printf("%#v\n", ti)
		///   // fmt.Println(reflect.ValueOf(ti).Type())
		///   // rv.Set(reflect.ValueOf(ti))
		/// }

		// fmt.Printf("%#v\n", tmp)
		// fmt.Println()

		// var foo2 Foo
		// if err := redis.ScanStruct(v, &foo2); err != nil {
		//   fmt.Println(err)
		//   return
		// }
		//foo2 := &Foo{}
		// if _, err := redis.Scan(v, &foo2.Number, &foo2.ATime, &foo2.String); err != nil {
		// // if _, err := redis.Scan(v, &foo2); err != nil {
		//   fmt.Println(err)
		//   return
		// }

		// fmt.Printf("%#v\n", foo2)

		// r := reflect.ValueOf(tmp).Elem()

		tmp2 := &Foo{}
		result, err := redis.StringMap(c.Do("HGETALL", id))
		if err != nil {
			fmt.Println(err)
			return
		}
		// mapToStruct(result, tmp2)
		structFromMap(result, tmp2)
		fmt.Printf("%#v\n", tmp2)
		// fmt.Println(result)
		// for k, v := range result {
		//   fmt.Println(k, v)
		// }

	}
}

func structFromMap(src map[string]string, dst interface{}) error {
	dt := reflect.TypeOf(dst).Elem()
	dv := reflect.ValueOf(dst).Elem()
	// fmt.Printf("%#v\n", d.Name())
	// fmt.Printf("%#v\n", re.NumField())

	// iter := reflect.ValueOf(dst).MapRange()
	// for iter.Next() {
	//   k := iter.Key()
	//   v := iter.Value()
	//   fmt.Println(k)
	//   fmt.Println(v)
	// }

	for i := 0; i < dt.NumField(); i++ {
		sf := dt.Field(i)
		sv := dv.Field(i)
		// fmt.Printf("%#v\n", sf.Name)
		// fmt.Printf("%#v\n", sv)
		if v, ok := src[strings.ToLower(sf.Name)]; ok {
			// fmt.Println(sf.Name, sf.Type)
			switch sv.Interface().(type) {
			case time.Time:
				format := "2006-01-02 15:04:05 -0700 MST"
				// if ti, err := time.Parse(time.RFC3339, v); err == nil {
				ti, err := time.Parse(format, v)
				if err != nil {
					return err
				}
				sv.Set(reflect.ValueOf(ti))
			case int, int64:
				x, err := strconv.ParseInt(v, 10, sv.Type().Bits())
				if err != nil {
					return err
				}
				sv.SetInt(x)
			default:
				sv.SetString(v)
			}
		}
	}

	// re := reflect.ValueOf(dst).Elem()
	// for k, v := range src {
	//   key := []byte(k)
	//   key[0] -= 32
	//   // fmt.Println(k, string(key), v)
	//   rf := re.FieldByName(string(key))
	//   fmt.Println(rf.Set)
	//   switch rf.Interface().(type) {
	//   case time.Time:
	//     format := "2006-01-02 15:04:05 -0700 MST"
	//     // if ti, err := time.Parse(time.RFC3339, v); err == nil {
	//     if ti, err := time.Parse(format, v); err == nil {
	//       rf.Set(reflect.ValueOf(ti))
	//     }
	//   case int, int64:
	//     x, _ := strconv.ParseInt(v, 10, rf.Type().Bits())
	//     rf.SetInt(x)
	//   default:
	//     rf.SetString(v)
	//   }
	// }

	return nil
}

func mapToStruct(src map[string]string, dst interface{}) error {
	// rt := reflect.TypeOf(dst)
	re := reflect.ValueOf(dst).Elem()
	for k, v := range src {
		key := []byte(k)
		key[0] -= 32
		// fmt.Println(k, string(key), v)
		rf := re.FieldByName(string(key))
		// fmt.Println(rf.Interface())
		switch rf.Interface().(type) {
		case time.Time:
			format := "2006-01-02 15:04:05 -0700 MST"
			// if ti, err := time.Parse(time.RFC3339, v); err == nil {
			if ti, err := time.Parse(format, v); err == nil {
				rf.Set(reflect.ValueOf(ti))
			}
		case int, int64:
			x, _ := strconv.ParseInt(v, 10, rf.Type().Bits())
			rf.SetInt(x)
		default:
			rf.SetString(v)
		}
	}

	return nil
}
