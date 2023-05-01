package main

import (
	"log"
	"net/http"
)

func main() {
	s := `
    {
      "members": [
        "test12",
        "test8800",
        "test0032",
        "test1234"
      ],
      "owners": [
        "test000",
        "test111",
        "test12",
        "test1234"
      ]
    }
    `

	http.HandleFunc("/projects", func(w http.ResponseWriter, _ *http.Request) {
		log.Println("Getting Projects")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(s))
	})

	log.Println("Listening...")
	log.Fatal(http.ListenAndServe(":8000", nil))
}
