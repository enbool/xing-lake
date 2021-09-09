package main

import "fmt"

/**
 * @Description: range表达式右边的值会在for循环第一次执行时赋值。如果是值类型，则for循环的元素不会被修改，如果是引用类型，则会被修改
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/9 17:44
 */
func main() {
	data := []int{1, 2, 3, 4, 5}
	for i, e := range data {
		if i == 3 {
			data[3] += e
		}
	}
	fmt.Println(data)

	arrayData := [...]int{1, 2, 3, 4, 5}

	for i, e := range arrayData {
		if i == 3 {
			arrayData[3] += e
		}
	}
	fmt.Println(arrayData)

	numbers2 := [...]int{1, 2, 3, 4, 5, 6}
	maxIndex2 := len(numbers2) - 1
	fmt.Println(numbers2)
	for i, e := range numbers2 {
		if i == maxIndex2 {
			numbers2[0] += e
		} else {
			numbers2[i+1] += e
		}
		fmt.Println(numbers2)
	}

}
