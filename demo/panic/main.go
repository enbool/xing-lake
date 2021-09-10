package main

import (
	"errors"
	"fmt"
)

/**
 * @Description:
 * defer关键字: 延迟执行代码，延迟到函数即将执行结束的那一刻
 *
 * @Author: wumin2
 * @Date:  2021/9/10 11:40
 */
func main()  {
	// 错误示例
	/*fmt.Println("Enter func main")
	panic(errors.New("something wrong"))
	p := recover()
	fmt.Println("panic: ", p)
	fmt.Println("exit main func")*/

	fmt.Println("Enter func main")
	res := forDefer();
	fmt.Println(res)

	fmt.Println("exit main func")
}

func forDefer() string{
	for i := 0; i < 5; i++ {
		defer func() {
			if r := recover(); r != nil {
				fmt.Println(i, " panic: ", r)
			}
			fmt.Println("recover fin")
		}()
		if i == 4 {
			panics()
		}
	}
	return "fin loop"
}
func doSomething() string {
	defer func() string {
		if r := recover(); r != nil {
			fmt.Println("panic: ", r)
			return "error"
		}
		fmt.Println("recover fin")
		return "fin"
	}()

	panics()
	return "hello"
}
func panics()  {
	panic(errors.New("something wrong"))
}