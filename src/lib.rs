
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod stl {

    pub mod parser {

        use std::fs::File;
        use std::io::Read;

        pub enum FileFormat {
            Invalid,
            Text,
            Binary,
        }

        pub fn get_format(stl_file_path: String) -> FileFormat {
            let mut stl_file = File::open(stl_file_path).expect("Unable to open the stl file");
            let mut buf = [0; 5];
            
            stl_file.read_exact(&mut buf).expect("Unable to read first 5 bytes");
            let header = String::from_utf8(buf.to_vec()).expect("Unable convert read buffer to string");
           
            if header.to_lowercase() == "solid" {
                FileFormat::Text
            } else {
                FileFormat::Binary
            }
        }
    }
}
