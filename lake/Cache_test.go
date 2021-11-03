package cache

import (
	"fmt"
	"log"
	"testing"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/11/2 18:31
 */

var db = map[string]string{
	"Tom": "630",
	"Jack": "390",
	"Sam": "567",
}

func TestGet(t *testing.T)  {
	loadCounts := make(map[string]int, len(db))

	lake := NewGroup("source", 1024, GetterFunc(
		func(key string) ([]byte, error) {
			log.Println("[SLOW DB] search key", key)

			if v, ok := db[key];ok{
				if _, ok := loadCounts[key]; ok{
					loadCounts[key] += 1
				}else {
					loadCounts[key] = 0
				}
				return []byte(v), nil
			}
			return nil, fmt.Errorf("%s not exist", key)
		}))
	for k, v := range db{
		view, err := lake.Get(k)
		if err != nil{
			t.Fatal("load failed")
		}
		if view.String() != v{
			t.Fatal("failed to get value of ", k)
		}
		if loadCounts[k] > 1 {
			t.Fatalf("cache %s miss", k)
		}
	}
	if view, err := lake.Get("unknown"); err == nil{
		t.Fatalf(" value of unknown should be empty, but %s got", view)
	}

}