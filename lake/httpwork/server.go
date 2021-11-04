package httpwork

import (
	"fmt"
	cache "lake"
	"log"
	"net/http"
	"strings"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/11/3 18:02
 */
const defaultBasePath = "/lake/"

type HTTPPool struct {
	self string
	basePath string
}

func NewHTTPPool(self string) *HTTPPool {
	return &HTTPPool{
		self: self,
		basePath: defaultBasePath,
	}
}

// Log print log with server name
func (httpPool *HTTPPool) Log(format string, v ...interface{}) {
	log.Printf("[Server %s]", httpPool.self, fmt.Sprintf(format, v))
}

// ServerHTTP URL: /basePath/groupName/key
func (httpPool *HTTPPool) ServeHTTP(w http.ResponseWriter, r *http.Request) {

	path := r.URL.Path
	if strings.Contains(path, "/favicon.ico"){
		w.Write([]byte("hello"))
		return
	}
	if !strings.HasPrefix(path, httpPool.basePath){
		panic("HTTP serving unexpected path: " + path)
	}
	httpPool.Log("%s %s", r.Method, path)

	parts := strings.Split(path, "/")

	if len(parts) != 4{
		http.Error(w, "Bad Request", http.StatusBadRequest)
		return
	}

	groupName := parts[2]
	key := parts[3]

	group := cache.GetGroup(groupName)

	if group == nil{
		http.Error(w, "No such group: " + groupName, http.StatusNotFound)
		return
	}

	view, err := group.Get(key)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.Write(view.ByteSlice())
}
