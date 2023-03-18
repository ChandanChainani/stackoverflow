package main

import (
	"os"
	"strings"

	"github.com/PuerkitoBio/goquery"
)

var html = `
<section>
    <article>
        <h2>Article 1</h2>
        <p>Text for article #1</p>
    </article>
    <article>
        <h2>Article 2</h2>
        <p>Text for article #2</p>
    </article>
</section>
`

func main() {
	qHtml, err := goquery.NewDocumentFromReader(strings.NewReader(html))
	if err != nil {
		panic(err)
	}

	section := qHtml.Find(`section`)
	section.BeforeHtml(`
    <h1>Team Supreme</h1>
`)

	goquery.Render(os.Stdout, qHtml.Selection)
}
