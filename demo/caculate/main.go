package main

import (
	"errors"
	"fmt"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/7 17:52
 */

type Operate func(x int, y int) int

func main() {
	var x, y = 1, 2
	var e = "-"
	operator, err := getOperator(e)
	if err != nil{
		panic(err)

	}
	var result = operator(x, y)
	fmt.Println(result)
}

func getOperator(e string) (o Operate, err error) {
	switch e {
	case "+":
		return plus, nil
	case "-":
		return subtract, nil
	case "*":
		return divide, nil
	case "/":
		return multiple, nil
	}
	return nil, errors.New("operate not support")
}

func plus(x int, y int) int {
	return x + y
}

func subtract(x int, y int) int {
	return x - y
}

func divide(x int, y int) int {
	return x / y
}

func multiple(x int, y int) int {
	return x * y
}
