package main

import (
	"fmt"

	"errors"
	"math/rand"
	// "strconv"
	"context"
	"github.com/orcaman/concurrent-map"
	"golang.org/x/sync/errgroup"
	"sync"
)

func main() {
	fmt.Println("Started")
	c := &customerConfig{}
	c.loadAtStartupAndProcessNewChanges()
	// time.Sleep(3000 * time.Millisecond)
	fmt.Println("Ended")
}

type customerConfig struct {
	deltaChan chan string
	err       chan error
	wg        sync.WaitGroup
	data      *cmap.ConcurrentMap
}

// this is called during server startup.
func (r *customerConfig) loadAtStartupAndProcessNewChanges() error {
	r.data = &cmap.ConcurrentMap{}
	fmt.Println("LoadAndProcess Start")
	// path, err := r.GetPath("...", "....")
	var err error
	path := "path"
	if err != nil {
		return err
	}

	r.wg.Add(1)
	go r.detectNewFiles(path)
	err = r.loadFiles(4, path)
	if err != nil {
		return err
	}
	r.wg.Add(1)
	go r.processNewFiles()
	fmt.Println("LoadAndProcess End")
	return nil
}

func (r *customerConfig) detectNewFiles(rootPath string) {
	defer r.wg.Done()
	fmt.Println("Detect Files")
}

func (r *customerConfig) loadFiles(workers int, path string) error {
	defer r.wg.Done()
	fmt.Println("Load Files")
	var files = []string{"one", "two", "three"}

	// reset the map so that it can have fresh state from new files.
	g, ctx := errgroup.WithContext(context.Background())
	sem := make(chan struct{}, workers)
	for _, file := range files {
		select {
		case <-ctx.Done():
			break
		case sem <- struct{}{}:
		}
		file := file
		g.Go(func() error {
			defer func() { <-sem }()
			data, err := r.read(file, "MyBucket")
			if err == nil {
				fmt.Println(data, err)
				r.data.Clear()
				for k, v := range data {
					r.data.Set(k, v)
				}
			}
			return err
		})
	}

	if err := g.Wait(); err != nil {
		return err
	}
	return nil
}

func (r *customerConfig) read(file string, bucket string) (map[string]int, error) {
	fmt.Println("Read Files")
	// read file and store it in "data" concurrent map
	// and if there is any error then return the error
	/// var err error
	/// fr, err := pars3.NewS3FileReader(context.Background(), bucket, file, r.s3Client.GetSession().Config)
	/// if err != nil {
	///   return errs.Wrap(err)
	/// }
	/// defer xio.CloseIgnoringErrors(fr)

	/// pr, err := reader.NewParquetReader(fr, nil, 8)
	/// if err != nil {
	///   return errs.Wrap(err)
	/// }

	/// if pr.GetNumRows() == 0 {
	///   log.Printf("Skipping %s due to 0 rows", file)
	///   // spn.Infof("Skipping %s due to 0 rows", file)
	///   return nil
	/// }

	/// for {
	///   rows, err := pr.ReadByNumber(r.cfg.RowsToRead)
	///   if err != nil {
	///     return errs.Wrap(err)
	///   }
	///   if len(rows) <= 0 {
	///     break
	///   }

	///   byteSlice, err := json.Marshal(rows)
	///   if err != nil {
	///     return errs.Wrap(err)
	///   }
	///   var invMods []CompModel
	///   err = json.Unmarshal(byteSlice, &invMods)
	///   if err != nil {
	///     return errs.Wrap(err)
	///   }

	///   for i := range invMods {
	///     key := strconv.FormatInt(invMods[i].ProductID, 10) + ":" + strconv.Itoa(int(invMods[i].Iaz))
	///     hasInventory := false
	///     if invMods[i].Available > 0 {
	///       hasInventory = true
	///     }
	///     r.data.Set(key, hasInventory)
	///   }
	/// }
	/// return nil
	var data = make(map[string]int)
	data[file] = 0
	data[bucket] = 0
	if rand.Int() > 0 {
		return data, nil
	}
	return nil, errors.New("Some Error")
}

// processNewFiles - load new files found by detectNewFiles
func (r *customerConfig) processNewFiles() {
	fmt.Println("processNewFiles")
	// find new files on delta channel
	// and call "loadFiles" method to read it
	// if there is any error, then it will add it to the error channel.
}

func (r *customerConfig) handleError() {
	fmt.Println("handleError")
	// read error from error channel if there is any
	// then log it
	fmt.Println("Logging Error")
}
