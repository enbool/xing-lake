package cache

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/10/27 19:28
 */
type ByteView struct {
	b []byte
}

func (v ByteView) Len() int  {
	return len(v.b)
}

func (v ByteView) ByteSlice() []byte {
	return cloneBytes(v.b)
}

func (v ByteView) String() string {
	return string(v.b)
}

func cloneBytes(b []byte) []byte {
	c := make([]byte, len(b))
	copy(c, b)
	return c
}


