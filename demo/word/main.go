package main

/**
 * @Description:
 *
 * dbd902fb2c04a24a98790cffd6a1c249b88cae366dae9a211c01e7804193efce
 * @Author: wumin2
 * @Date:  2021/10/8 10:04
 */
import (
	"fmt"
	"github.com/unidoc/unioffice/common/license"
	"github.com/unidoc/unioffice/document"
	"io/fs"
	"io/ioutil"
	"path/filepath"
	"sort"
	"strconv"
	"strings"
	"time"
)

func init() {
	// Make sure to load your metered License API key prior to using the library.
	// If you need a key, you can sign up and create a free one at https://cloud.unidoc.io
	err := license.SetMeteredKey(`dbd902fb2c04a24a98790cffd6a1c249b88cae366dae9a211c01e7804193efce`)
	if err != nil {
		panic(err)
	}
}

func main() {
	fmt.Println("请输入要处理的文件夹路径: ")
	var root = "./input"
	fmt.Scanln(&root)

	fmt.Println("请输入每个表格截取多少行: ")
	var limit = 0
	fmt.Scanln(&limit)

	var files []string

	err := filepath.Walk(root, func(path string, info fs.FileInfo, err error) error {
		if !info.IsDir() {
			if strings.HasSuffix(path, ".docx") {
				files = append(files, path)
			}
		}
		return nil
	})

	if err != nil {
		panic(err)
	}
	var result = ""
	for _, file := range files {
		result += file + "\n"
		result += parseDoc(file, limit) + "\n"
	}
	writeTXT(result)
}

func parseDoc(filename string, limit int) string {
	doc, err := document.Open(filename)
	if err != nil {
		panic("open failed")
	}
	defer doc.Close()

	var result = ""
	tables := doc.Tables()
	for _, talbe := range tables {
		rows := talbe.Rows()
		var records = make([]*Record, len(rows)-1)
		for i, row := range rows {
			if i == 0 {
				continue
			}
			record := row2Record(row)
			records[i-1] = record
		}
		sort.Slice(records, func(i, j int) bool {
			if percentStr2Float(records[i].Percent) > percentStr2Float(records[j].Percent) {
				return true
			}
			return false
		})
		if limit > 0{
			records = records[:limit]
		}
		result += fmt.Sprint(records) + "\n"
	}
	return result
}

func writeTXT(s string) {
	currentTime := time.Now()
	filename := currentTime.Format("2006-01-02-15.04.05")
	var outputFilename = "./" + filename + ".txt"
	err := ioutil.WriteFile(outputFilename, []byte(s), 0644)
	if err != nil {
		panic(err)
	}
}

func percentStr2Float(source string) float64 {
	var percent = 0.0
	if len(source) != 0 {
		source = strings.TrimSuffix(source, "%")
		percentTmp, err := strconv.ParseFloat(source, 64)
		if err != nil {
			percentTmp = 0.0
		}
		percent = percentTmp
	}
	return percent
}

func row2Record(row document.Row) *Record {
	cells := row.Cells()
	name := toText(cells[0].Paragraphs())
	description := toText(cells[1].Paragraphs())
	percentStr := toText(cells[2].Paragraphs())

	return New(name, description, percentStr)
}
func toText(paragraphs []document.Paragraph) string {
	var text = ""
	for _, paragraph := range paragraphs {
		runs := paragraph.Runs()
		for _, run := range runs {
			text = text + run.Text()
		}
	}
	return text
}
