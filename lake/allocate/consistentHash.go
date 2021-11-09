package allocate

import (
	"hash/crc32"
	"sort"
	"strconv"
)

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/11/5 10:44
 */
type Hash func(data []byte) uint32

type Node struct {
	addr string
}

func NewNode(addr string) *Node {
	return &Node{
		addr: addr,
	}
}

type Map struct {
	hashFunc Hash
	replicas int
	keys     []int
	nodeMap  map[int]string
}

func New(replicas int, hashFunc Hash) *Map {
	if hashFunc == nil {
		hashFunc = crc32.ChecksumIEEE
	}

	return &Map{
		hashFunc: hashFunc,
		replicas: replicas,
		nodeMap:  make(map[int]string),
	}
}

// Add 往hash环添加一个节点
func (m *Map) Add(addrs ...string) {
	for _, addr := range addrs {
		for i := 0; i < m.replicas; i++ {
			key := int(m.hashFunc([]byte(addr + strconv.Itoa(i))))
			m.keys = append(m.keys, key)
			m.nodeMap[key] = addr
		}
	}
	sort.Ints(m.keys)
}

func Remove() {

}

// 根据key从hash环上选取一个节点
func (m *Map) Get(key string) string {
	hash := m.hashFunc([]byte(key))

	index := sort.Search(len(m.keys), func(i int) bool {
		return m.keys[i] >= int(hash)
	})

	return m.nodeMap[m.keys[index%(len(m.keys))]]
}
