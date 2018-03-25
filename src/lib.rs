#[cfg(test)]
mod tests {
    use stl;

    #[test]
    fn binary_format_check() {
        let fmt = stl::parser::get_format("/home/vpati011/Downloads/ship_v2_top.stl".to_string());
        match fmt {
            stl::parser::FileFormat::Binary => println!("Pass"),
            _ => panic!("Test failed for detect the binary format for the STL file"),
        }
    }

    #[test]
    fn text_format_check() {
        let fmt = stl::parser::get_format("/home/vpati011/Downloads/cube.stl".to_string());
        match fmt {
            stl::parser::FileFormat::Text => println!("Pass"),
            _ => panic!("Test failed for detect the text format for the STL file"),
        }
    }
}

pub mod stl {

    pub mod parser {

        use std::fs::File;
        use std::io::Read;

        #[derive(Debug)]
        pub enum FileFormat {
            Invalid,
            Text,
            Binary,
        }

        pub fn get_format(stl_file_path: String) -> FileFormat {
            let mut stl_file = File::open(stl_file_path).expect("Unable to open the stl file");
            let mut buf = [0; 80];
            
            stl_file.read_exact(&mut buf).expect("Unable to the STL file header");
            let header = String::from_utf8(buf.to_vec()).expect("Unable convert read buffer to string");
           
            if header.trim().to_lowercase().starts_with("solid") {
                FileFormat::Text
            } else {
                FileFormat::Binary
            }
        }
    }
}
