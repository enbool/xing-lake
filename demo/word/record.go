package main

/**
 * @Description:
 *
 *
 * @Author: wumin2
 * @Date:  2021/10/28 10:23
 */
type Record struct {
	Name        string
	Description string
	Percent     string
}

func New(name string, description string, percent string) *Record {
	return &Record{
		Name: name,
		Description: description,
		Percent: percent,
	}
}

func (r Record) String() string {
	return r.Name + ", " + r.Description + ", " + r.Percent + ";"
}