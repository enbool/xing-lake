package mvc

import (
	"net/http"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/15 11:07
 */
type router struct {
	handlers map[string]HandlerFunc
}

func newRouter() *router {
	return &router{handlers: make(map[string]HandlerFunc)}
}

// Add a path to the Engine router
func (router *router) addRoute(method string, pattern string, handlerFunc HandlerFunc)  {
	key := buildRouterKey(method, pattern)
	router.handlers[key] = handlerFunc
}


func (router *router) handle(c *Context) {
	key := buildRouterKey(c.Method, c.Path)

	if handler, ok := router.handlers[key]; ok{
		handler(c)
	}else {
		c.String(http.StatusNotFound, "404\n%s", c.Path)
	}
}

func buildRouterKey(method string, pattern string) string {
	return method + "-" + pattern
}