package main

import (
	"fmt"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

type User struct {
	ID   int
	Name string
}

func main() {
	// github.com/mattn/go-sqlite3
	db, err := gorm.Open(
		sqlite.Open("file::memory:?cache=shared"),
		&gorm.Config{},
	)
	if err != nil {
		fmt.Println("Something went wrong", err)
		return
	}
	db.AutoMigrate(&User{})

	db.Create(&[]User{{
		1, "John",
	}, {
		2, "James",
	}})

	u := db.FirstOrCreate(&User{
		3, "Jack",
	})
	// fmt.Printf("%+v\n", u)
	fmt.Printf("RowsAffected: %d\n", u.RowsAffected)

	u = db.FirstOrCreate(&User{
		Name: "Jack",
	})

	// fmt.Printf("%+v\n", u)
	// fmt.Println(u.RowsAffected)
	fmt.Printf("RowsAffected: %d\n", u.RowsAffected)
}
