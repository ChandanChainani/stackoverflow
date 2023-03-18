package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/PuerkitoBio/goquery"
	"golang.org/x/net/html"
)

var doc = `
<section>
    <article>
        <h1>Article 1</h1>
        <p>Text for article #1</p>
    </article>
    <article>
        <h1>Article 2</h1>
        <p>Text for article #2</p>
    </article>
</section>
`

func main() {

	qHtml, err := goquery.NewDocumentFromReader(strings.NewReader(doc))
	if err != nil {
		panic(err)
	}

	searchStr := `article`
	// articles := qHtml.Find(`article:nth-child(2)`)
	// articles := qHtml.FindMatcher(goquery.Single(searchStr))
	// articles := qHtml.Add(searchStr)

	node := html.Node{
		Data: searchStr,
	}
	articles := qHtml.FindNodes(&node)

	fmt.Printf("%#v\n", articles)
	goquery.Render(os.Stdout, articles)
}
