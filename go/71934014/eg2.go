package main

import (
	"log"
	"reflect"

	"github.com/gofiber/fiber/v2"
)

// query: curl -g -X GET "http://localhost:3000/?persons[0][name]=john&persons[0][pass]=doe"

// recommendation -> name of the api and parameters
type ApiParameters struct {
	Persons []Person `query:"persons" json:"persons" xml:"persons" form:"persons"`
}

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

func main() {
	app := fiber.New()
	app.Get("/", func(c *fiber.Ctx) error {
		parameters := ApiParameters{}

		if err := c.QueryParser(&parameters); err != nil {
			return err
		}

		log.Printf("GET: %#v\n", parameters)
		// log.Println(parameters.Persons[0].Name) // john
		// log.Println(parameters.Persons[0].Pass) // doe

		return c.SendString("Get Called")
	})

	app.Post("/", func(c *fiber.Ctx) error {
		log.Println(string(c.Body()))
		// GetMethods(c)
		log.Println(c.GetReqHeaders())
		log.Println(c.AcceptsCharsets())
		log.Println(c.AcceptsEncodings())
		log.Println(c.AcceptsLanguages())
		log.Println(c.AllParams())
		// 2022/04/22 13:12:27 Accepts
		// 2022/04/22 13:12:27 AcceptsCharsets
		// 2022/04/22 13:12:27 AcceptsEncodings
		// 2022/04/22 13:12:27 AcceptsLanguages
		// 2022/04/22 13:12:27 AllParams

		// log.Println(string(c.ContentType()))

		parameters := ApiParameters{}

		if err := c.BodyParser(&parameters); err != nil {
			return err
		}

		log.Printf("POST: %#v\n", parameters)
		// log.Println(parameters.Persons[0].Name) // john
		// log.Println(parameters.Persons[0].Pass) // doe

		return c.SendString("Post Called")
	})

	log.Fatalln(app.Listen(":3000"))
}
