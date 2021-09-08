package main

import "fmt"

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/8 17:46
 */
func main()  {
	nums := [4]int{2,7,11,15}
	result := twoSum(nums[:], 8)
	fmt.Println(result)
}

func twoSum(nums []int, target int) []int {
	var result []int
	if len(nums) == 0{
		return result
	}
	var cache = make(map[int]int)

	for i, num := range nums{
		var remain = target - num
		index, presennt := cache[remain]
		if presennt{
			result = append(result, index)
			result = append(result, i)

			return result
		}
		cache[num] = i
	}
	return result
}