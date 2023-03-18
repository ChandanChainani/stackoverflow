package main

import (
	"fmt"
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
	sectionChildren := section.Children().Clone()
	goquery.Render(os.Stdout, sectionChildren)
	fmt.Println()
	fmt.Println()
	section.ReplaceWithSelection(sectionChildren)

	goquery.Render(os.Stdout, qHtml.Contents())
	fmt.Println()
	fmt.Println()
	section = qHtml.Find(`section`)
	goquery.Render(os.Stdout, section)
}
