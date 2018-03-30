extern crate byteorder;

#[cfg(test)]
mod tests {
    use stl;
    use std::slice::Iter;

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
            Ok(model) => { 
                println!("Triangle count");
                (*model).iter().inspect(|triangle| println!("{:?}", triangle));
            }
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
            vertex:[Vertex; 3],
        }

        impl Triangle {

            fn new() -> Box<Triangle> {
                Box::new(Triangle{vertex: Triangle::new_vertex()})
            }

            fn new_vertex() -> [Vertex; 3] {
                [
                    Vertex{x:0.0, y:0.0, z:0.0},
                    Vertex{x:0.0, y:0.0, z:0.0},
                    Vertex{x:0.0, y:0.0, z:0.0},
                ]
            }
        }

        #[derive(Debug)]
        pub struct Model {
            triangles: Vec<Box<Triangle>>,
        }
        
        impl Model {
            fn iter(&self) -> TriangleIterator {
                TriangleIterator{index:0, model: self}
            }
        }

        pub struct TriangleIterator<'a> {
            index: usize, 
            model: &'a Model,
        }

        impl <'a>Iterator for TriangleIterator<'a> {
            
            type Item = Box<Triangle>;

            fn next(&mut self) -> Option<Box<Triangle>> {
                if self.index < self.model.triangles.len() {
                    match self.model.triangles.get(self.index) {
                        Some(triangle) => Some(*triangle),
                        None => None
                    }
                } else {
                    None
                }
            }
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
            let mut model = Box::new(Model{triangles: Vec::new()});
            const BUF32_SIZE: usize = 4;
            let mut stl_file = File::open(stl_file_path).map_err(Error::InvalidPath)?;
            let mut buf = [0; HEADER_SIZE];
            
            stl_file.read_exact(&mut buf).map_err(Error::InvalidRead)?;
            let header = String::from_utf8(buf.to_vec()).map_err(Error::InvalidFormat)?;
            
            let mut buf32 = [0; BUF32_SIZE];
            stl_file.read_exact(&mut buf32).map_err(Error::InvalidRead)?;
            let triangle_cnt = LittleEndian::read_u32(&mut buf32); 
           
            for i in 0..triangle_cnt {

                /* Normal Vector */
                for normal in 0..3 {
                    stl_file.read_exact(&mut buf32).map_err(Error::InvalidRead)?;
                }
                
                let mut triangle = Triangle::new();
                for v in 0..3 {
                    stl_file.read_exact(&mut buf32).map_err(Error::InvalidRead)?;
                    triangle.vertex[v].x = LittleEndian::read_f32(&mut buf32); 

                    stl_file.read_exact(&mut buf32).map_err(Error::InvalidRead)?;
                    triangle.vertex[v].y = LittleEndian::read_f32(&mut buf32); 
                    
                    stl_file.read_exact(&mut buf32).map_err(Error::InvalidRead)?;
                    triangle.vertex[v].z = LittleEndian::read_f32(&mut buf32); 
                }
                
                model.triangles.push(triangle);
            }

            Ok(model)
        }
    }
}
