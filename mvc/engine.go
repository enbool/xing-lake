package mvc

import (
	"fmt"
	"net/http"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/14 18:13
 */
type Engine struct {
	router map[string]http.HandlerFunc
}

// New is the Construct function for Engine
func New() *Engine {
	return &Engine{router: make(map[string]http.HandlerFunc)}
}

// Add a path to the Engine router
func (engine *Engine) addRouter(method string, pattern string, handlerFunc http.HandlerFunc)  {
	key := buildRouterKey(method, pattern)
	engine.router[key] = handlerFunc
}

// GetRequest Add a GET path to the Engine router
func (engine *Engine) RegisterGetRequest(pattern string, handlerFunc http.HandlerFunc)  {
	engine.addRouter("GET", pattern, handlerFunc)
}

// RegisterPostRequest Add a POST path to the Engine router
func (engine *Engine) RegisterPostRequest(pattern string, handlerFunc http.HandlerFunc)  {
	engine.addRouter("POST", pattern, handlerFunc)
}

func (engine *Engine)Run(port string) error {
	return http.ListenAndServe(":" + port, engine)
}
func (engine *Engine) ServeHTTP(w http.ResponseWriter, req *http.Request)  {
	key := buildRouterKey(req.Method, req.URL.Path)

	if handler, ok := engine.router[key]; ok{
		handler(w, req)
	}else {
		fmt.Fprintf(w, "404\n%s", req.URL)
	}
}

func buildRouterKey(method string, pattern string) string {
	return method + "-" + pattern
}