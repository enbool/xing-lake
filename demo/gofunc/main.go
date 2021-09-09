package main

import "fmt"

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/9 16:45
 */
func main() {
	var countdown = make(chan int, 2)
	for i := 0; i < 10; i++ {
		go func(i int) {
			fmt.Println(i)
			countdown <- i
		}(i)
	}

	for {
		i, ok := <- countdown
		if !ok{
			break
		}
		fmt.Println("func ", i)
	}
	fmt.Println("finished")

}