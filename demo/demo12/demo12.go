package main

import "fmt"

/**
 * @Description: 数组和切片
 * 数组：长度固定。值类型。数组的容量(cap)等于长度(len)
 *
 * 切片：长度可变。切片的长度可以自动随元素增长而增长，但不会随元素减少而减少。引用类型
 *
 * @Author: wumin2
 * @Date:  2021/9/7 14:30
 */
func main() {
	var s1 = make([]int, 5)
	fmt.Println("len of s1: ", len(s1))
	fmt.Println("cap of s1: ", cap(s1))
	fmt.Printf("value of s1: %d", s1)

	var s2 = make([]int, 8)
	fmt.Println("len of s1: ", len(s2))
	fmt.Println("cap of s1: ", cap(s2))
	fmt.Println("value of s1: %d", s2)

	var s3 = []int{0, 1,2,3,4,5,6,7,8,9}
	fmt.Println("len of s3: ", len(s3))
	fmt.Println(s3[9])

	var s4 = s3[2:3]
	fmt.Println(s4)
	fmt.Println("len of s4: ", len(s4)) // 1
	fmt.Println("cap of s4: ", cap(s4)) // 8

	var s5 = s4[1:5]
	fmt.Println(s5) // [3 4 5 6]

	var s6 = s5[0:cap(s5)]
	fmt.Println(s6) // [3 4 5 6 7 8 9]
	fmt.Println("len of s6: ", len(s6)) // 7
	fmt.Println("cap of s6: ", cap(s6)) // 7
	s6 = append(s6, 10)
	fmt.Println(s6) // [3 4 5 6 7 8 9 10]
	fmt.Println("len of s6: ", len(s6)) // 8
	fmt.Println("cap of s6: ", cap(s6)) // 14
}