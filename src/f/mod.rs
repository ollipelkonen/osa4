
extern crate glium;
use std::{fs, io};
pub mod shader;
pub mod sdf;
pub mod primitives;


#[derive(Debug)]
pub struct FObject {
  pub meshes: Vec<FMesh>,
  pub materials: Vec<primitives::FMaterial>,
  pub textures: Vec<glium::texture::SrgbTexture2d>,
}

impl FObject {
  pub fn print_bounds(self) {
    for m in self.meshes {
      m.print_bounds();
    }
  }
}


#[derive(Copy, Clone, Debug)]
pub struct Vertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);


impl Vertex {
  pub fn new(p: [f32; 3], n: [f32; 3], t: [f32; 2]) -> Vertex {
    Vertex {
      position: [p[0], p[1], p[2]],
      normal: [n[0], n[1], n[2]],
      tex_coords: [t[0], t[1]],
    }
  }
  pub fn zero() -> Vertex {
    Vertex {
      position: [0.0, 0.0, 0.0],
      normal: [0.0, 0.0, 0.0],
      tex_coords: [0.0, 0.0]
    }
  }
  pub fn min() -> Vertex {
    Vertex {
      position: [f32::MIN, f32::MIN, f32::MIN],
      normal: [f32::MIN, f32::MIN, f32::MIN],
      tex_coords: [f32::MIN, f32::MIN]
    }
  }
  pub fn max() -> Vertex {
    Vertex {
      position: [f32::MAX, f32::MAX, f32::MAX],
      normal: [f32::MAX, f32::MAX, f32::MAX],
      tex_coords: [f32::MAX, f32::MAX]
    }
  }
}

#[allow(dead_code)]
impl Default for Vertex {
  fn default() -> Self {
    Vertex {
      position: [0.0, 0.0, 0.0],
      normal: [0.0, 0.0, 1.0],
      tex_coords: [0.0, 0.0]
    }
  }
}



////#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
#[derive(Debug)]
pub struct FMesh {
  pub vbuffer: glium::vertex::VertexBuffer<Vertex>,
  pub ibuffer: glium::index::IndexBuffer<u32>,
  pub material: Option<usize>,
  pub vertices: Option<Vec<Vertex>>,
  pub indices: Option<Vec<u32>>,
  pub edges: Option<Vec<primitives::FEdge>>,
  pub bounds: Option<[Vertex; 2]>
}

//TODO
/*
#[allow(dead_code)]
impl Default for FMesh {
  fn default() -> Self {
    FMesh {
      vbuffer: None,
      ibuffer: None,
      material: None,
      vertices: None,
      indices: None,
      edges: None,
      bounds: None
    }
  }
}
*/

impl FMesh {
  pub fn print_bounds(self) {
    println!("bounds.  center [{:?}, {:?}, {:?}]   size: [{:?}, {:?}, {:?}] ",
      (self.bounds.unwrap()[0].position[0]+self.bounds.unwrap()[1].position[0])/2.0,
      (self.bounds.unwrap()[0].position[1]+self.bounds.unwrap()[1].position[1])/2.0,
      (self.bounds.unwrap()[0].position[2]+self.bounds.unwrap()[1].position[2])/2.0,
      (self.bounds.unwrap()[1].position[0]-self.bounds.unwrap()[0].position[0]),
      (self.bounds.unwrap()[1].position[1]-self.bounds.unwrap()[0].position[1]),
      (self.bounds.unwrap()[1].position[2]-self.bounds.unwrap()[0].position[2])
    );
  }
}


#[allow(dead_code)]
fn print_type_of<T>(_: &T, text: &str) {
  println!("{:?} {:?}", text, std::any::type_name::<T>())
}



pub fn get_mime(filename: &str) -> image::ImageFormat
{
  if filename.to_lowercase().ends_with("png") {
    image::ImageFormat::Png
  } else {
    image::ImageFormat::Jpeg
  }
}

pub fn load_image(uri: &str, display: &glium::Display) -> Option<glium::texture::SrgbTexture2d> {
  let filename = format!("data/{}", uri);
  let file = match fs::File::open( filename ) {
    Ok(f) => f,
    Err(e) => {
      println!("no file {:?}", e);
      std::process::exit(1);
    }
  };
  let reader = io::BufReader::new(file);
  let image = image::load(reader, get_mime(uri)).unwrap().to_rgba8();
  let image_dimensions = image.dimensions();
  let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
  Some(glium::texture::SrgbTexture2d::new(display, image).unwrap())
}

fn load_image_from_source(a: gltf::image::Source, display: &glium::Display) -> Option<glium::texture::SrgbTexture2d> {
  match a {
    gltf::image::Source::Uri{uri, mime_type: _} => {
      load_image(uri, display)
    },
    _ => None
  }
}


struct ImportData {
  pub doc: gltf::Document,
  pub buffers: Vec<gltf::buffer::Data>,
  pub images: Vec<gltf::image::Data>,
}



//#[allow(dead_code)]
fn mesh_from_gltf( g_primitive: &gltf::Primitive<'_>, imp: &ImportData, display: &glium::Display ) -> FMesh
{
  let buffers = &imp.buffers;
  let reader = g_primitive.reader(|buffer| Some(&buffers[buffer.index()]));
  let mut vertices: Vec<Vertex> = reader
        .read_positions()
        .unwrap_or_else(||
          panic!("primitives must have the POSITION attribute (mesh: , primitive: )")
        ).map(|position| {
            Vertex::new( position, [0.0,0.0,0.0], [0.0,0.0])
        }).collect();

  if let Some(normals) = reader.read_normals() {
    for (i, normal) in normals.enumerate() {
      vertices[i].normal = normal;
    }
  }

  let mut tex_coord_set = 0;
  while let Some(tex_coords) = reader.read_tex_coords(tex_coord_set) {
      if tex_coord_set > 1 {
          println!("Ignoring texture coordinate set {}, \
                  only supporting 2 sets at the moment. (mesh: , primitive: )", tex_coord_set);
          tex_coord_set += 1;
          continue;
      }
      for (i, tex_coord) in tex_coords.into_f32().enumerate() {
          match tex_coord_set {
              0 => vertices[i].tex_coords = tex_coord,
              //1 => vertices[i].tex_coord_1 = Vector2::from(tex_coord),
              _ => unreachable!()
          }
      }
      tex_coord_set += 1;
  }

  let indices = reader
    .read_indices()
    .map(|read_indices| {
      read_indices.into_u32().collect::<Vec<_>>()
    });


  let bounds: [Vertex; 2] = vertices.iter().fold( [Vertex::max(), Vertex::min()], |st, elem| {
      [
        Vertex {
          position: [
            st[0].position[0].min( elem.position[0] ),
            st[0].position[1].min( elem.position[1] ),
            st[0].position[2].min( elem.position[2] )
          ],
            ..Vertex::default()
        },
        Vertex {
          position: [
            st[1].position[0].max( elem.position[0] ),
            st[1].position[1].max( elem.position[1] ),
            st[1].position[2].max( elem.position[2] )
          ],
          ..Vertex::default()
        }
      ]
  } );

  let vbuffer = glium::vertex::VertexBuffer::new(display, &vertices).unwrap();
  let ibuffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &(indices).as_ref().unwrap().as_slice()).unwrap();
  let material = g_primitive.material().index();
  FMesh{ vbuffer: vbuffer, ibuffer: ibuffer, material, vertices: Some(vertices), indices: Some(indices.unwrap()), bounds: Some(bounds), edges: None }
}



impl FObject {
  pub fn load_gltf( filename: &str, display: &glium::Display ) -> FObject {
    let (doc, buffers, images) = match gltf::import(filename) {
      Ok(tuple) => tuple,
      Err(err) => {
        println!("glTF import failed: {:?}", err);
        if let gltf::Error::Io(_) = err {
          println!("Hint: Are the .bin file(s) referenced by the .gltf file available?")
        }
        std::process::exit(1)
      },
    };

    let imp = ImportData { doc, buffers, images };

    let textures: Vec<glium::texture::SrgbTexture2d> = imp.doc.textures().map( |im| {
        load_image_from_source(im.source().source(), display)
      })
      .filter_map(|a| a)
      .collect();


    let meshes: Vec<FMesh> = imp.doc.nodes()
      .filter_map(|n| n.mesh() )
      .map( |node| {
        mesh_from_gltf(&node.primitives().next().unwrap(), &imp, display)
      })
      .collect::<Vec<_>>();


    let materials = imp.doc.materials()
      .map( |material| {
        let diffuse_texture = match material.pbr_metallic_roughness().base_color_texture() {
          Some(d) => Some(d.texture().index()),
          _ => None
          };
        let normal_texture = match material.normal_texture() {
          Some(d) => Some(d.texture().index()),
          _ => None
          };
        let occlusion_texture = match material.occlusion_texture() {
          Some(d) => Some(d.texture().index()),
          _ => None
          };
        let metallic_roughness_texture = None;
        primitives::FMaterial { diffuse_texture, normal_texture, occlusion_texture, metallic_roughness_texture }
      })
      .collect();

    FObject { meshes, materials, textures }
  }

  pub fn print_tree(&self, node: &gltf::Node, depth: i32) {
    for _ in 0..(depth - 1) {
      print!("  ");
    }
    print!(" -");
    print!(" Node {}", node.index());
    #[cfg(feature = "names")]
    print!(" ({})", node.name().unwrap_or("<Unnamed>"));
    if let Some(mesh) = node.mesh() {
      print!(" mesh {:?}", mesh.primitives());
    }
    if let Some(_cam) = node.camera() {
      print!(" camera ");
    }
    if let Some(_skin) = node.skin() {
      print!(" skin ");
    }
    println!();
    for child in node.children() {
      self.print_tree(&child, depth + 1);
    }
  }

}

