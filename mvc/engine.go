package mvc

import (
	"net/http"
)

// HandlerFunc define the request handler
type HandlerFunc func(ctx *Context)

type Engine struct {
	router *router
}

// New is the Construct function for Engine
func New() *Engine {
	return &Engine{router: newRouter()}
}

// RegisterGetRequest Add a GET path to the Engine router
func (engine *Engine) RegisterGetRequest(pattern string, handlerFunc HandlerFunc)  {
	engine.addRoute("GET", pattern, handlerFunc)
}

// RegisterPostRequest Add a POST path to the Engine router
func (engine *Engine) RegisterPostRequest(pattern string, handlerFunc HandlerFunc)  {
	engine.addRoute("POST", pattern, handlerFunc)
}

func (engine *Engine)Run(port string) error {
	return http.ListenAndServe(":" + port, engine)
}
func (engine *Engine) ServeHTTP(w http.ResponseWriter, req *http.Request)  {
	c := newConext(w, req)
	engine.router.handle(c)
}

func (engine *Engine) addRoute(method string, pattern string, handler HandlerFunc) {
	engine.router.addRoute(method, pattern, handler)
}
