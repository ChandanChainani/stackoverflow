package main

import (
	"fmt"

	"math/rand"
	"os"
	"os/signal"
	"time"
	// "strconv"
	// "sync"
	// "context"
	"errors"
)

func main() {
	// var s []string
	// // var s = []string{}
	// fmt.Println(s)
	// // s = append(s, []string{"1", "2", "3"}...)
	// s = append(s, s...)
	// fmt.Println(s)

	fmt.Println(">>>", center("STARTED", 30), "<<<")

	// New Version
	c := &Config{
		InitialPath:    "Old Path",
		DetectInterval: 3000,
	}
	c.start()
	// time.Sleep(3000 * time.Millisecond)
	fmt.Println(">>>", center("ENDED", 30), "<<<")
}

func center(s string, w int) string {
	// fmt.Sprintf("%[1]*s", -w, fmt.Sprintf("%[1]*s", (w + len(s))/2, s))
	return fmt.Sprintf("%*s", -w, fmt.Sprintf("%*s", (w+len(s))/2, s))
}

// fmt.Println(">>>", center("a", 110), "<<<")
// fmt.Println(">>>", center("ab", 110), "<<<")
// fmt.Println(">>>", center("abc", 110), "<<<")
// fmt.Println(">>>", center("in the middle", 110), "<<<")
// fmt.Println(">>>", center("short window", 20), "<<<")
// fmt.Println(">>>", center("a very short window that underruns the text", 20), "<<<")

type Config struct {
	deltaCh        chan string
	ticker         *time.Ticker
	stopSignal     chan os.Signal
	InitialPath    string
	DetectInterval time.Duration
}

func (c *Config) start() {
	c.stopSignal = make(chan os.Signal, 1)
	signal.Notify(c.stopSignal, os.Interrupt)

	c.ticker = time.NewTicker(c.DetectInterval * time.Millisecond)
	c.deltaCh = make(chan string, 1)

	go c.detect()
	go c.load()

	if c.InitialPath != "" {
		c.deltaCh <- c.InitialPath
	}

	<-c.stopSignal

	c.ticker.Stop()
	close(c.stopSignal)
}

// Detect New Files
func (c *Config) detect() {
	for {
		select {
		case <-c.stopSignal:
			close(c.deltaCh)
			return
		case <-c.ticker.C:
			fmt.Println(">>>", center("DETECT", 30), "<<<")
			c.deltaCh <- fmt.Sprintf("PATH %f", rand.Float64()*1.5)
		}
	}
	fmt.Println(">>>", center("DETECT END", 30), "<<<")
}

// Read Files
func read(path string) (map[string]int, error) {
	data := make(map[string]int)
	data[path] = 0
	fmt.Println(">>>", center("READ", 30), "<<<")
	fmt.Println(path)
	return data, nil
}

// Load Files
func (c *Config) load() {
	for {
		select {
		case <-c.stopSignal:
			return
		case path := <-c.deltaCh:
			fmt.Println(">>>", center("LOAD", 30), "<<<")
			data, err := read(path)
			if err != nil {
				fmt.Println("Log Error")
			} else {
				fmt.Println("Success", data)
			}
			fmt.Println()
		}
	}

	fmt.Println(">>>", center("LOAD END", 30), "<<<")
}
