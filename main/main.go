package main

import (
	"fmt"
	cache "lake"
	"lake/httpwork"
	"log"
	"net/http"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/11/4 17:18
 */
var db2 = map[string]string{
	"Tom":  "630",
	"Jack": "390",
	"Sam":  "567",
}

func main() {
	cache.NewGroup("score", 1024, cache.GetterFunc(
		func(key string) ([]byte, error) {
			log.Println("[SLOW DB] search key", key)

			if v, ok := db2[key]; ok {
				return []byte(v), nil
			}
			return nil, fmt.Errorf("%s not exist", key)
		}))

	addr := "localhost:8000"

	peers := httpwork.NewHTTPPool(addr)
	log.Println("Lake Server is running at ", addr)
	http.ListenAndServe(addr, peers)
}
