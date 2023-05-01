package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"time"
)

func main() {
	client := &http.Client{Timeout: 10 * time.Second}
	req, err := http.NewRequest("GET", fmt.Sprintf("%s/projects", "http://localhost:8000"), nil)
	r, err := client.Do(req)
	if err != nil {
		log.Fatal(err)
	}
	defer r.Body.Close()
	// var tmp interface{}
	var projects map[string]interface{}
	err = json.NewDecoder(r.Body).Decode(&projects)
	if err != nil {
		log.Fatal(err)
	}

	// projects, ok := tmp.(map[string]interface{})
	// fmt.Println(ok)
	fmt.Printf("%#v\n", projects)
	fmt.Printf("%#v\n", projects["members"])
	c, ok := projects["members"].([]string)
	fmt.Println(ok)
	fmt.Printf("%#v\n", c)
}
