package main

import (
	"fmt"
)

/**
 * @Description: 通道（channel）是一个FIFO队列。关键字chan，操作符<-
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/7 18:37
 */
func main() {
	var ch1 = make(chan int, 1)
	ch1 <- 1
	// ch1 <- 2 //fatal error: all goroutines are asleep - deadlock!
	var ch2 = make(chan int, 1)
	// ele, ok := <-ch2 //fatal error: all goroutines are asleep - deadlock!
	// if ok {
	//	fmt.Println(ele)
	//}
	ch2 <- 1

	var ch3 chan int
	// ch3 <- 1 //fatal error: all goroutines are asleep - deadlock!
	_ = ch3

	var ch4 = make(chan int, 2)
	go func() {
		for i := 0; i < 10; i++ {
			fmt.Println("send element ", i)
			ch4 <- i
			// time.Sleep(1 * time.Microsecond)
		}
		fmt.Println("close channel")
		close(ch4)
	}()

	for {
		elem, ok := <-ch4
		if !ok{
			fmt.Println("channel closed")
			break
		}
		fmt.Println("received element: ", elem)
	}
	fmt.Println("end")

}
