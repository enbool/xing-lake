package main

import (
	"fmt"
	"sync"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/10 17:09
 */
func main() {
	var data = 0
	var sign = make(chan int, 5)
	var mu sync.Mutex
	for i := 0; i < 10; i++ {
		go func() {
			mu.Lock()
			defer mu.Unlock()
			sign <- i
			data = data + 1
		}()
	}

	for i := 0; i < 10; i++ {
		<-sign
	}
	fmt.Println(data)

}
