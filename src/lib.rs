extern crate byteorder;



#[cfg(test)]
mod tests {
    use parser;
    use parser::Vertex;

    const BIN_STL_FILE: &str = "/Users/vpatil3/tmp/Cube_3d_printing_sample.stl";
    const TXT_STL_FILE: &str = "/Users/vpatil3/tmp/bottle.stl";

    #[test]
    fn binary_format_check() {
        let fmt = parser::get_format(&BIN_STL_FILE.to_string());
        match fmt {
            Ok(parser::FileFormat::Binary) => println!("Pass: Binary format"),
            _ => panic!("Test failed to detect the binary format for the STL file"),
        }
    }

    #[test]
    fn text_format_check() {
        let fmt = parser::get_format(&TXT_STL_FILE.to_string());
        match fmt {
            Ok(parser::FileFormat::Text) => println!("Pass: Text format"),
            _ => panic!("Test failed for detect the text format for the STL file"),
        }
    }

    #[test]
    fn binary_stl_load() {
        match parser::load_file(&BIN_STL_FILE.to_string()) {
            Ok(model) => {
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
    fn text_stl_load() {
        match parser::load_file(&TXT_STL_FILE.to_string()) {
            Ok(model) => {
                let mut idx = 0;
                for triangle in (*model).iter() {
                    println!("Triangle {}\n{}", idx, triangle);
                    idx += 1;
                }
            }
            _ => panic!("Failed to parse the text STL file"),
        }
    }

    #[test]
    fn triangle_count() {
        match parser::load_file(&TXT_STL_FILE.to_string()) {
            Ok(model) => {
                println!("{}", model.triangle_count());
            }
            _ => panic!("Failed to parse the text STL file"),
        }
    }

    #[test]
    fn triangle_vertices() {
        match parser::load_file(&TXT_STL_FILE.to_string()) {
            Ok(model) => {
                let mut idx = 0;
                for triangle in (*model).iter() {
                    println!("Triangle {}\n{:?}", idx, triangle.vertices());
                    idx += 1;
                }
            }
            _ => panic!("Failed to parse the text STL file"),
        }
    }

    #[test]
    fn vertex_array() {
        match parser::load_file(&BIN_STL_FILE.to_string()) {
            Ok(model) => {
                for triangle in (*model).iter() {
                    let vertices = triangle.vertices();
                    let v1: Vertex = vertices[0];
                    let v2: Vertex = vertices[1];
                    let v3: Vertex = vertices[2];
                    println!("{:?} {:?} {:?}", v1, v2, v3);
                }
            }
            _ => panic!("Failed to parse the text STL file"),
        }
    }
}

/// A RUST module to parse STL files. The format of the STL
/// file can be found at https://en.wikipedia.org/wiki/STL_(file_format)
pub mod parser {

    use std::fs::File;
    use std::io::Read;
    use std;

    const HEADER_SIZE: usize = 80;
    const VERTEX_CNT: usize = 3;
    const F32_SIZE: usize = 4;

    /// Types of STL file
    #[derive(Debug)]
    pub enum FileFormat {
        Text,
        Binary,
    }

    /// Possible Errors while parsing the STL file
    pub enum Error {
        InvalidPath(std::io::Error),
        InvalidRead(std::io::Error),
        InvalidFormat(std::string::FromUtf8Error),
    }

    /// Determines if an STL file is in text or a binary format
    pub fn get_format(stl_file_path: &str) -> Result<FileFormat, Error> {
        let mut stl_file = File::open(stl_file_path).map_err(self::Error::InvalidPath)?;
        let mut buf = [0; HEADER_SIZE];

        stl_file
            .read_exact(&mut buf)
            .map_err(self::Error::InvalidRead)?;
        let header = String::from_utf8(buf.to_vec()).map_err(self::Error::InvalidFormat)?;

        if header.trim().to_lowercase().starts_with("solid") {
            Ok(FileFormat::Text)
        } else {
            Ok(FileFormat::Binary)
        }
    }

    /// Represents a 3D vertex of a triangle
    #[derive(Debug, Copy, Clone)]
    pub struct Vertex {
        x: f32,
        y: f32,
        z: f32,
    }

    impl Vertex {
        pub fn get_x(&self) -> f32 {
            self.x
        }

        pub fn get_y(&self) -> f32 {
            self.y
        }

        pub fn get_z(&self) -> f32 {
            self.z
        }
    }

    /// Represents a triangle that makes up the 3D object
    #[derive(Debug, Copy, Clone)]
    pub struct Triangle {
        normal: Vertex,
        vertex: [Vertex; VERTEX_CNT],
    }

    impl Triangle {
        fn new() -> Triangle {
            Triangle {
                normal: Vertex {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                vertex: Triangle::new_vertices(),
            }
        }

        pub fn vertices(&self) -> &[Vertex; VERTEX_CNT] {
            return &self.vertex;
        }

        fn new_vertices() -> [Vertex; VERTEX_CNT] {
            [
                Vertex {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vertex {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vertex {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            ]
        }

        pub fn calculate_normal(&mut self) {
            let mut u = Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let mut v = Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };

            u.x = self.vertex[1].x - self.vertex[0].x;
            u.y = self.vertex[1].y - self.vertex[0].y;
            u.z = self.vertex[1].z - self.vertex[0].z;

            v.x = self.vertex[2].x - self.vertex[0].x;
            v.y = self.vertex[2].y - self.vertex[0].y;
            v.z = self.vertex[2].z - self.vertex[0].z;

            self.normal.x = u.y * v.z - u.z * v.y;
            self.normal.y = u.z * v.x - u.x * v.z;
            self.normal.z = u.x * v.y - u.y * v.x;

            let mut length = self.normal.x * self.normal.x + self.normal.y * self.normal.y
                + self.normal.z * self.normal.z;
            length = length.sqrt();

            self.normal.x = self.normal.x / length;
            self.normal.y = self.normal.y / length;
            self.normal.z = self.normal.z / length;
        }
    }

    impl std::fmt::Display for Triangle {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                f,
                "{:.2} {:.2} {:.2}\n{:.2} {:.2} {:.2}\n{:.2} {:.2} {:.2}\n{:.2} {:.2} {:.2}",
                self.normal.x,
                self.normal.y,
                self.normal.z,
                self.vertex[0].x,
                self.vertex[0].y,
                self.vertex[0].z,
                self.vertex[1].x,
                self.vertex[1].y,
                self.vertex[1].z,
                self.vertex[2].x,
                self.vertex[2].y,
                self.vertex[2].z
            )
        }
    }

    /// Representation of the 3D object in terms of triangles
    #[derive(Debug)]
    pub struct Model {
        triangles: Vec<Triangle>,
    }

    impl Model {
        pub fn iter(&self) -> TriangleIterator {
            TriangleIterator {
                index: 0,
                model: &self,
            }
        }

        pub fn triangle_count(&self) -> usize {
            self.triangles.len()
        }

        pub fn calculate_normals(&mut self) {
            for triangle in self.triangles.iter_mut() {
                triangle.calculate_normal();
            }
        }
    }

    impl<'a> IntoIterator for &'a Model {
        type Item = &'a Triangle;
        type IntoIter = TriangleIterator<'a>;

        /// Iterator to iterate over all the triangles that
        /// make up the 3D object
        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    /// The iterator for all the triangles making up the 3D
    /// object
    pub struct TriangleIterator<'a> {
        index: usize,
        model: &'a Model,
    }

    impl<'a> Iterator for TriangleIterator<'a> {
        type Item = &'a Triangle;

        fn next(&mut self) -> Option<&'a Triangle> {
            if self.index < self.model.triangles.len() {
                match self.model.triangles.get(self.index) {
                    Some(triangle) => {
                        self.index += 1;
                        Some(triangle)
                    }
                    None => None,
                }
            } else {
                None
            }
        }
    }

    /// Load a STL file and return the Model struct
    pub fn load_file(stl_file_path: &str) -> Result<Box<Model>, Error> {
        let stl_fmt = get_format(stl_file_path)?;
        println!("format {:?}", stl_fmt);
        match stl_fmt {
            FileFormat::Binary => load_bin_file(stl_file_path),
            FileFormat::Text => load_txt_file(stl_file_path),
        }
    }

    use std::io::BufReader;
    use std::io::BufRead;

    fn load_txt_file(stl_file_path: &str) -> Result<Box<Model>, Error> {
        let stl_file = File::open(stl_file_path).map_err(self::Error::InvalidPath)?;
        let mut file = BufReader::new(&stl_file);

        let mut model = Box::new(Model {
            triangles: Vec::new(),
        });
        let mut triangle = Triangle {
            normal: Vertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            vertex: Triangle::new_vertices(),
        };
        let mut vertex: usize = 0;

        loop {
            let mut line = String::new();
            file.read_line(&mut line).map_err(self::Error::InvalidRead)?;

            if line.is_empty() {
                break;
            }

            let tokens: Vec<&str> = line.trim().split_whitespace().collect();

            if tokens[0] == "facet" {
                vertex = 0;
                if tokens.len() == 5 {
                    triangle.normal.x = tokens[2].parse::<f32>().unwrap();
                    triangle.normal.y = tokens[3].parse::<f32>().unwrap();
                    triangle.normal.z = tokens[4].parse::<f32>().unwrap();
                }
            }

            if tokens[0] == "vertex" && tokens.len() == 4 {
                triangle.vertex[vertex].x = tokens[1].parse::<f32>().unwrap();
                triangle.vertex[vertex].y = tokens[2].parse::<f32>().unwrap();
                triangle.vertex[vertex].z = tokens[3].parse::<f32>().unwrap();
                vertex += 1;
            }

            if tokens[0] == "endfacet" {
                model.triangles.push(triangle);
            }
        }

        model.calculate_normals();

        Ok(model)
    }

    use byteorder::LittleEndian;
    use byteorder::ByteOrder;

    fn load_vertex(buf: &[u8], vertex: &mut Vertex, offset: &mut usize) {
        vertex.x = LittleEndian::read_f32(&buf[*offset..*offset + F32_SIZE]);
        *offset += F32_SIZE;

        vertex.y = LittleEndian::read_f32(&buf[*offset..*offset + F32_SIZE]);
        *offset += F32_SIZE;

        vertex.z = LittleEndian::read_f32(&buf[*offset..*offset + F32_SIZE]);
        *offset += F32_SIZE;
    }

    fn load_bin_file(stl_file_path: &str) -> Result<Box<Model>, Error> {
        let mut stl_file = File::open(stl_file_path).map_err(self::Error::InvalidPath)?;

        let mut triangle_byte_vec = Vec::new();
        stl_file
            .read_to_end(&mut triangle_byte_vec)
            .map_err(self::Error::InvalidRead)?;
        let buf = triangle_byte_vec.as_slice();

        let mut offset = HEADER_SIZE;
        let triangle_cnt = LittleEndian::read_u32(&buf[offset..offset + F32_SIZE]);
        offset += F32_SIZE;

        let mut model = Box::new(Model {
            triangles: Vec::new(),
        });
        for _ in 0..triangle_cnt {
            let mut triangle = Triangle::new();

            /* Normal Vector */
            load_vertex(buf, &mut triangle.normal, &mut offset);

            /* Triangle Side vertices */
            for v in 0..VERTEX_CNT {
                load_vertex(buf, &mut triangle.vertex[v], &mut offset);
            }

            offset += 2;

            model.triangles.push(triangle);
        }

        model.calculate_normals();

        Ok(model)
    }
}
