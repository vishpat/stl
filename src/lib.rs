extern crate byteorder;

#[cfg(test)]
mod tests {
    use stl;
    
    #[test]
    fn binary_format_check() {
        let fmt = stl::parser::get_format(&"/home/vpati011/Downloads/ship_v2_top.stl".to_string());
        match fmt {
            Ok(format) => println!("Pass"),
            _ => panic!("Test failed for detect the binary format for the STL file"),
        }
    }

    #[test]
    fn binary_stl_load() {
        match stl::parser::load_file(&"/home/vpati011/Downloads/ship_v2_top.stl".to_string()) {
            Ok(model) => println!("Triangle count {}", model.triangle_cnt),
            _ => panic!("Failed to parse the binary STL file"),
        }
    }


    #[test]
    fn text_format_check() {
        let fmt = stl::parser::get_format(&"/home/vpati011/Downloads/cube.stl".to_string());
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

        pub enum FileFormat {
            Text,
            Binary,
        }

        pub enum Error {
            InvalidPath(std::io::Error),
            InvalidRead(std::io::Error),
            InvalidFormat(std::string::FromUtf8Error),
        }

        pub fn get_format(stl_file_path: &String) -> Result<FileFormat, Error> {
            let mut stl_file = File::open(stl_file_path).map_err(Error::InvalidPath)?;
            let mut buf = [0; HEADER_SIZE];
            
            stl_file.read_exact(&mut buf).map_err(Error::InvalidRead)?;
            let header = String::from_utf8(buf.to_vec()).map_err(Error::InvalidFormat)?;
           
            if header.trim().to_lowercase().starts_with("solid") {
                Ok(FileFormat::Text)
            } else {
                Ok(FileFormat::Binary)
            }
        }

        #[derive(Debug)]
        pub struct Vertex {
            x: f32,
            y: f32,
            z: f32,
        }
        
        #[derive(Debug)]
        pub struct Triangle { 
            sides:[Vertex; 3],
        }

        #[derive(Debug)]
        pub struct Model {
            min_x: f32,
            max_x: f32,

            min_y: f32,
            max_y: f32,

            min_z: f32,
            max_z: f32,

            pub triangle_cnt: u32,
            triangles: Vec<Triangle>,
        }

        pub fn load_file(stl_file_path: &String) -> Result<Box<Model>, Error> {
            let stl_fmt = get_format(stl_file_path)?;
            match stl_fmt {
                Binary => load_bin_file(stl_file_path),
                Text => panic!("Not implemented")
            }
        }

        use byteorder::LittleEndian;
        use byteorder::ByteOrder;

        fn load_bin_file(stl_file_path: &String) -> Result<Box<Model>, Error> {
            let mut model = Box::new(Model{min_x: 0.0, min_y: 0.0, min_z: 0.0, 
                                           max_x: 0.0, max_y: 0.0, max_z: 0.0, 
                                           triangle_cnt: 0, triangles: Vec::new()});
            const U32_SIZE: usize = 4;
            let mut stl_file = File::open(stl_file_path).map_err(Error::InvalidPath)?;
            let mut buf = [0; HEADER_SIZE];
            
            stl_file.read_exact(&mut buf).map_err(Error::InvalidRead)?;
            let header = String::from_utf8(buf.to_vec()).map_err(Error::InvalidFormat)?;
            
            let mut u32_buf = [0; U32_SIZE];
            stl_file.read_exact(&mut u32_buf).map_err(Error::InvalidRead)?;
            model.triangle_cnt = LittleEndian::read_u32(&mut u32_buf); 
            
            Ok(model)
        }
    }
}
