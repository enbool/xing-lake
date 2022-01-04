package allocate

import (
	"strconv"
	"testing"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/11/11 23:56
 */

func TestHash(t *testing.T) {
	hash := New(100, func(key []byte) uint32 {
		i, _ := strconv.Atoi(string(key))
		return uint32(i)
	})

	// Given the above hash function, this will give replicas with "hashes":
	// 2, 4, 6, 12, 14, 16, 22, 24, 26
	hash.Add("6", "4", "2")

	testCases := map[string]string{
		"2":  "2",
		"11": "2",
		"23": "4",
		"27": "2",
	}

	for k, v:=
}