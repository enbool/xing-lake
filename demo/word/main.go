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
	doc, err := document.Open("input.docx")
	if err != nil {
		panic("open failed")
	}
	defer doc.Close()

	tables := doc.Tables()

	for _, talbe := range tables{
		rows := talbe.Rows()

		for i, row := range rows {
			if i == 0{
				continue
			}
			cells := row.Cells()
			for _, cell := range cells{
				fmt.Println(toText(cell.Paragraphs()))
			}
		}
	}
}

func toText(paragraphs []document.Paragraph) string {
	var text = ""
	for _, paragraph := range paragraphs{
		runs := paragraph.Runs()
		for _, run := range runs{
			text = text + run.Text()
		}
	}
	return text
}