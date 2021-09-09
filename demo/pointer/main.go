package main

import "fmt"

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/9 10:39
 */
type Dog struct {
	Name string
}

func New(name string) Dog {
	return Dog{ name}
}

func (dog *Dog) SetName(name string) {
	dog.Name = name
	// return *dog
}
func main() {
	dog := New("big")
	dog.SetName("xiaoyu")
	fmt.Println(dog)
	p := &(dog.Name)
	*p = "23"
	fmt.Println(dog)

}