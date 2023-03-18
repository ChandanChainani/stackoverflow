package main

import (
	"log"

	"github.com/gofiber/fiber/v2"
)

type Product struct {
	// Model
	Code        string  `json:"code"`
	Title       string  `json:"title"`
	Description string  `json:"description"`
	Image       string  `json:"image"`
	Price       float64 `json:"price"`
	Stocks      []Stock `json:"stock,omitempty" gorm:"many2many:product_stocks;"`
}

type Stock struct {
	// Model
	Size     string `json:"size"`
	Color    string `json:"color"`
	Quantity int    `json:"quantity"`
}

func main() {
	app := fiber.New()

	// app.Get("/", func (c *fiber.Ctx) error {
	//     return c.SendString("Hello, World!")
	// })

	app.Post("/", func(c *fiber.Ctx) error {
		p := new(Product)

		if err := c.BodyParser(&p); err != nil {
			return err
		}
		log.Printf("%#v\n", p)

		// Get raw body from POST request:
		return c.Send(c.Body()) // []byte("user=john")
	})

	log.Fatal(app.Listen(":3000"))
}
