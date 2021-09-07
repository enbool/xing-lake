package main

import "fmt"

/**
 * @Description:
 * 怎么判断一个变量类型 x.(T)
 *  x表示要被判断类型的值，T表示类型
 *  value, ok := x.(T)
 *
 * @Author: wumin2
 * @Date:  2021/9/7 14:10
 */

var container = []string{"zero", "one", "two"}

func main() {
	container := map[int]string{0: "zero", 1: "one", 2: "two"}
	value, ok := interface{}(container).(map[int]string)
	if ok{
		fmt.Print(value)
	}
	fmt.Printf("The element is %q.\n\n", container[1])

	var null = string(-1)
	fmt.Print(null)
}
