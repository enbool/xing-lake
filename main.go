package main

import (
	"fmt"
	"mvc"
	"net/http"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/14 18:01
 */
func main()  {
	engine := mvc.New()

	engine.RegisterGetRequest("/", indexHandler)
	engine.RegisterPostRequest("/hello", helloHandler)

	engine.Run("8888")
}

func indexHandler(w http.ResponseWriter, req *http.Request) {
	fmt.Fprintf(w, "url: %q\n", req.URL.Path)
}

func helloHandler(w http.ResponseWriter, req *http.Request)  {
	for k, v := range req.Header{
		fmt.Fprintf(w, "Header[%q] = %q\n", k, v)
	}
}