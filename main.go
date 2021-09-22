package main

import (
	"fmt"
	"mvc"
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
	engine.RegisterGetRequest("/hello", helloHandler)

	engine.Run("8888")
}

func indexHandler(c *mvc.Context) {
	fmt.Fprintf(c.Writer, "url: %q\n", c.Path)
}

func helloHandler(c *mvc.Context)  {
	for k, v := range c.Request.Header{
		fmt.Fprintf(c.Writer, "Header[%q] = %q\n", k, v)
	}
}