extern crate byteorder;

#[cfg(test)]
mod tests {
    use stl;
    use std::slice::Iter;

    #[test]
    fn binary_format_check() {
        let fmt = stl::parser::get_format(&"/home/vpati011/Downloads/HalfDonut.stl".to_string());
        match fmt {
            Ok(format) => println!("Pass"),
            _ => panic!("Test failed to detect the binary format for the STL file"),
        }
    }

    #[test]
    fn binary_stl_load() {
        match stl::parser::load_file(&"/home/vpati011/Downloads/HalfDonut.stl".to_string()) {
            Ok(model) => { 
                println!("Triangle count");
                let mut idx = 0;
                for triangle in (*model).iter() {
                    println!("Triangle {}\n{}", idx, triangle);
                    idx += 1;
                }
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
        const VERTEX_CNT: usize = 3;

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
            normal: Vertex,
            vertex:[Vertex; VERTEX_CNT],
        }

        impl Triangle {

            fn new() -> Box<Triangle> {
                Box::new(Triangle{normal: Vertex{x:0.0, y:0.0, z:0.0}, vertex: Triangle::new_vertex()})
            }

            fn new_vertex() -> [Vertex; VERTEX_CNT] {
                [
                    Vertex{x:0.0, y:0.0, z:0.0},
                    Vertex{x:0.0, y:0.0, z:0.0},
                    Vertex{x:0.0, y:0.0, z:0.0},
                ]
            }
        }

        impl std::fmt::Display for Triangle {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:.2} {:.2} {:.2}\n{:.2} {:.2} {:.2}\n{:.2} {:.2} {:.2}\n{:.2} {:.2} {:.2}",
                       self.normal.x, self.normal.y, self.normal.z,
                       self.vertex[0].x, self.vertex[0].y, self.vertex[0].z,
                       self.vertex[1].x, self.vertex[1].y, self.vertex[1].z,
                       self.vertex[2].x, self.vertex[2].y, self.vertex[2].z )
            }
        }

        #[derive(Debug)]
        pub struct Model {
            triangles: Vec<Box<Triangle>>,
        }
        
        impl Model {
            pub fn iter(&self) -> TriangleIterator {
                TriangleIterator{index:0, model: self}
            }
        }

        pub struct TriangleIterator<'a> {
            index: usize, 
            model: &'a Model,
        }

        impl <'a>Iterator for TriangleIterator<'a> {
            
            type Item = &'a Box<Triangle>;

            fn next(&mut self) -> Option<&'a Box<Triangle>> {
                if self.index < self.model.triangles.len() {
                    match self.model.triangles.get(self.index) {
                        Some(triangle) => {
                                            self.index += 1;
                                            Some(triangle)
                                          },
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
            const F32_SIZE: usize = 4;

            let mut triangle_byte_vec = Vec::new();
            let mut stl_file = File::open(stl_file_path).map_err(Error::InvalidPath)?;
            stl_file.read_to_end(&mut triangle_byte_vec).map_err(Error::InvalidRead)?;

            let buf = triangle_byte_vec.as_slice();
            let mut offset = HEADER_SIZE;
            let triangle_cnt = LittleEndian::read_u32(&buf[offset..offset + F32_SIZE]); 
            offset += F32_SIZE; 
        
            for triangle_idx in 0..triangle_cnt {
                let mut triangle = Triangle::new();

                /* Normal Vector */
                let mut normal:Vertex = Vertex{x:0.0, y:0.0, z:0.0};
                triangle.normal.x = LittleEndian::read_f32(&buf[offset..offset + F32_SIZE]);
                offset += F32_SIZE;
                
                triangle.normal.y = LittleEndian::read_f32(&buf[offset..offset + F32_SIZE]);
                offset += F32_SIZE;
                
                triangle.normal.z = LittleEndian::read_f32(&buf[offset..offset + F32_SIZE]);
                offset += F32_SIZE;

                /* Triangle Side vertices */
                for v in 0..VERTEX_CNT {
                    triangle.vertex[v].x = LittleEndian::read_f32(&buf[offset..offset + F32_SIZE]);
                    offset += F32_SIZE;

                    triangle.vertex[v].y = LittleEndian::read_f32(&buf[offset..offset + F32_SIZE]);
                    offset += F32_SIZE;

                    triangle.vertex[v].z = LittleEndian::read_f32(&buf[offset..offset + F32_SIZE]);
                    offset += F32_SIZE;
                }

                offset += 2;

                model.triangles.push(triangle);
            }

            Ok(model)
        }
    }
}
