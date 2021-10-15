package lake

import "container/list"

/**
 * @Description:
 *
 * LRU缓存淘汰策略
 * @Author: wumin2
 * @Date:  2021/10/14 19:19
 */

type Cache struct {
	maxBytes int64
	nbytes int64
	ll *list.List
	cache map[string] *list.Element
}