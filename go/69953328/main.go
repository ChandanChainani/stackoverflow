package main

import (
	"html/template"
	"log"
	"net/http"
	"strings"
)

type playerType struct {
	Fname string
	Lname string
	// Men are true and Women are false
	Gender bool
}

var players []playerType

var templates *template.Template

func handleFilePath() {
	http.Handle("/templates/", http.StripPrefix("/templates/", http.FileServer(http.Dir("templates"))))
	http.Handle("/assets/", http.StripPrefix("/assets/", http.FileServer(http.Dir("assets"))))

	//templates = template.Must(template.ParseFiles("./templates/index.html", "./templates/displayPlayer.html"))

	templates = template.Must(template.New("").ParseGlob("./templates/*.html"))
}

func landingPage(w http.ResponseWriter, r *http.Request) {
	err := templates.ExecuteTemplate(w, "home", nil)
	if err != nil {
		log.Panicln(err)
		return
	}
}

func addPlayerHandler(w http.ResponseWriter, r *http.Request) {
	Fname := r.FormValue("firstName")
	Lname := r.FormValue("lastName")
	Gender := r.FormValue("gender")

	if addPlayer(Fname, Lname, Gender) {
		log.Printf("Player added: %v, %v, %v", Fname, Lname, Gender)
		err := templates.ExecuteTemplate(w, "displayPlayer", players[len(players)-1])
		if err != nil {
			log.Panicln(err)
			return
		}
	}
}

func addPlayer(Fname string, Lname string, Gender string) bool {

	if Fname == "" || Lname == "" || Gender == "" {
		return false
	}

	var playerToBeAdded playerType

	// Set men to true and women to false
	if strings.ToLower(Gender) == "male" {
		playerToBeAdded = playerType{Fname, Lname, true}
	} else {
		playerToBeAdded = playerType{Fname, Lname, false}
	}

	players = append(players, playerToBeAdded)

	return true
}

func main() {
	handleFilePath()
	http.HandleFunc("/", landingPage)

	log.Println("Listening...")

	http.HandleFunc("/addPlayer", addPlayerHandler)

	err := http.ListenAndServe(":8081", nil)
	if err != nil {
		log.Panicln(err)
	}
}
