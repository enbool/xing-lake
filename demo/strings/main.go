package main

import (
	"fmt"
	"strings"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/9/13 14:26
 */

func main() {
	configPath := "./resource/TB_BB_6_B.txt"
	if strings.HasPrefix(configPath,"/") || strings.HasPrefix(configPath,"./"){
		// nothing
	}else {
		fmt.Println(true)
		//configPath = gamePath + configPath
	}
}