#[cfg(test)]
mod tests {
    use stl;

    #[test]
    fn binary_format_check() {
        let fmt = stl::parser::get_format("/home/vpati011/Downloads/ship_v2_top.stl".to_string());
        match fmt {
            Ok(format) => println!("Pass"),
            _ => panic!("Test failed for detect the binary format for the STL file"),
        }
    }

    #[test]
    fn text_format_check() {
        let fmt = stl::parser::get_format("/home/vpati011/Downloads/cube.stl".to_string());
        match fmt {
            Ok(format) => println!("Pass"),
            _ => panic!("Test failed for detect the text format for the STL file"),
        }
    }
}

pub mod stl {

    pub mod parser {

        use std::fs::File;
        use std::io::Read;
        use std;

        const HEADER_SIZE: usize = 80;

        #[derive(Debug)]
        pub enum FileFormat {
            Text,
            Binary,
        }

        pub enum FileFormatError {
            InvalidPath(std::io::Error),
            InvalidRead(std::io::Error),
            InvalidFormat(std::string::FromUtf8Error),
        }

        pub enum ModelError {
            STLFileInvalid,
        }

        pub struct Vertex {
            x: f32,
            y: f32,
            z: f32,
        }

        pub struct Triangle { 
            sides:[Vertex; 3],
        }

        pub struct Model {
            triangles: Vec<Triangle>,
        }

        pub fn get_format(stl_file_path: String) -> Result<FileFormat, FileFormatError> {
            let mut stl_file = File::open(stl_file_path).map_err(FileFormatError::InvalidPath)?;
            let mut buf = [0; HEADER_SIZE];
            
            stl_file.read_exact(&mut buf).map_err(FileFormatError::InvalidRead)?;
            let header = String::from_utf8(buf.to_vec()).map_err(FileFormatError::InvalidFormat)?;
           
            if header.trim().to_lowercase().starts_with("solid") {
                Ok(FileFormat::Text)
            } else {
                Ok(FileFormat::Binary)
            }
        }
    }
}
