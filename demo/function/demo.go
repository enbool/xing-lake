package main

import "fmt"

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/7 17:44
 */

type Printer func(contents string) (n int, err error)

func printToStd(contents string) (bytesNum int, err error) {
	return fmt.Println(contents)
}

func main() {
	var p = printToStd
	p("someting")
}