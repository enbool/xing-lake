
#[cfg(test)]
pub mod test{
    use std::fs;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};

    #[test]
    fn test(){
        let filename = "C:/Users/wumin2/Desktop/fsdownload/message.txt";
        let file = File::open(filename).unwrap();

        let output_filename = "C:/Users/wumin2/Desktop/fsdownload/output.txt";
        let mut output = File::create(output_filename).unwrap();
        
        let buf = BufReader::new(file);
       let data: Vec<i32> = vec![];
        let e = data.get(1).unwrap();
        for line in buf.lines() {
            match line {
                Ok(mut line) => {
                    if line.contains("pokerSuit"){
                        line = line + "\n";
                        output.write_all(line.as_bytes());
                    }
                }
                Err(_) => continue
            }
        }
        
        
        output.flush();
    }
}