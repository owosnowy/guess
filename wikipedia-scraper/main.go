package main

import (
	"fmt"
	"os"
	"strings"

	"github.com/gocolly/colly/v2"
)

func main() {
	file, err := os.Create("description.txt")
	if err != nil {
		fmt.Println("Error creating file:", err)
		return
	}
	defer file.Close()
	c := colly.NewCollector()
	c.OnHTML("div.mw-parser-output > p", func(e *colly.HTMLElement) {
		description := e.Text
		description = strings.TrimSpace(description)
		if description != "" {
			file.WriteString(description + "\n")
		}
	})
	c.OnRequest(func(r *colly.Request) {
		fmt.Println("Visiting", r.URL)
	})
	err = c.Visit("https://en.wikipedia.org/wiki/Kanye_West")
	if err != nil {
		fmt.Println("Error visiting URL:", err)
	}
}
