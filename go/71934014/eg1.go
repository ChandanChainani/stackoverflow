package main

import (
	"reflect"

	"github.com/gofiber/fiber/v2"
	"log"

	"net/url"
	// "github.com/gorilla/schema"
	"github.com/go-playground/form/v4"
)

type Person struct {
	Name string `query:"name" json:"name" xml:"name" form:"name"`
	Pass string `query:"pass" json:"pass" xml:"pass" form:"pass"`
}

func GetMethods(T interface{}) {
	tmpType := reflect.TypeOf(T)
	for i := 0; i < tmpType.NumMethod(); i++ {
		method := tmpType.Method(i)
		log.Println(method.Name)
	}
}

// var decoder = schema.NewDecoder()
var decoder = form.NewDecoder()

func main() {
	app := fiber.New()

	app.Get("/", func(c *fiber.Ctx) error {
		log.Println(c.Request().URI().QueryArgs().PeekMulti("name"))
		log.Println(string(c.Request().URI().QueryString()))
		log.Println(c.Body())
		log.Println(c.AllParams())
		log.Println(c.MultipartForm())
		log.Println("Query:", c.Query("name[]"))
		log.Println("Form:", c.FormValue("name[]"))
		var tmp struct {
			Name string `query:"name"`
		}
		log.Println("Parser:", c.QueryParser(&tmp))
		persons := []Person{}
		// p := struct{
		//   Person []Person `query: "person"`
		// }{}

		log.Println(c)
		// if err := c.QueryParser(&p); err != nil {
		if err := c.BodyParser(&persons); err != nil {
			log.Println("HERE")
			return err
		}

		// log.Println(p[0].Name)     // john
		// log.Println(p[0].Pass)     // doe
		log.Println(persons)

		log.Println()
		return c.SendString("Get Called")
	})

	app.Post("/", func(c *fiber.Ctx) error {
		// GetMethods(c.Request())
		GetMethods(c.Request().URI())
		GetMethods(c)
		log.Println()
		log.Printf("%+v\n", string(c.Body()))
		log.Println(c.AllParams())
		log.Println(c.MultipartForm())

		log.Println(c.FormValue("name[]"))

		// log.Println(c.Request().URI().QueryArgs().PeekMulti("name"))
		test := url.Values{
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
		log.Println(test)

		persons := []Person{}
		// persons := struct{
		//   Person []Person `form:"person"`
		// }{}

		m, err := url.ParseQuery(string(c.Body()))
		if err != nil {
			return err
		}
		log.Println(m)
		err = decoder.Decode(&persons, m)
		if err != nil {
			return err
		}
		log.Printf("Result: %#v\n", persons)

		//  persons := struct{
		//    person []Person  `form:"person[]"`
		//  }{}
		// if err := c.BodyParser(&persons); err != nil {
		//   return err
		// }

		for _, person := range persons {
			log.Println(person.Name) // john
			log.Println(person.Pass) // doe
		}
		log.Println()
		return c.SendString("Post Called")
	})

	// curl -X POST -H "Content-Type: application/json" --data '[{"name":"john","pass":"doe"},{"name": "jack", "pass":"jill"}]' localhost:3000

	app.Listen(":3000")
}
