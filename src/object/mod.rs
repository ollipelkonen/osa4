
extern crate glium;
use gltf::Gltf;
use std::{fs, io};


#[allow(dead_code)]
pub struct FObject {
  pub meshes: Vec<FMesh>,
  pub materials: Vec<FMaterial>,
  pub textures: Vec<glium::texture::SrgbTexture2d>,
}

pub struct FMesh {
  pub vbuffer: glium::vertex::VertexBuffer<Vertex>,
  pub ibuffer: glium::index::IndexBuffer<u32>,
  pub material: Option<usize>
}

pub struct FMaterial {
  pub diffuse_texture: Option<usize>,
  pub normal_texture: Option<usize>,
  pub occlusion_texture: Option<usize>,
  pub metallic_roughness_texture: Option<usize>  // unused
}


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

impl Vertex {
  fn new(p: [f32; 3], n: [f32; 3], t: [f32; 2]) -> Vertex {
      Vertex {
          position: [p[0], p[1], p[2]],
          normal: [n[0], n[1], n[2]],
          tex_coords: [t[0], t[1]],
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



pub struct ImportData {
  pub doc: gltf::Document,
  pub buffers: Vec<gltf::buffer::Data>,
  pub images: Vec<gltf::image::Data>,
}


pub fn get_mime(filename: &str) -> image::ImageFormat
{
  if filename.to_lowercase().ends_with("png") {
    image::ImageFormat::Png
  } else {
    image::ImageFormat::Jpeg
  }
}

pub fn load_image(a: gltf::image::Source, display: &glium::Display) -> Option<glium::texture::SrgbTexture2d> {
  match a {
    gltf::image::Source::Uri{uri, mime_type} => {
      let filename = format!("data/{}", uri);
      let file = fs::File::open( filename ).unwrap();
      let reader = io::BufReader::new(file);
      let image = image::load(reader, get_mime(uri)).unwrap().to_rgba8();
      let image_dimensions = image.dimensions();
      let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
      Some(glium::texture::SrgbTexture2d::new(display, image).unwrap())
    },
    _ => None
  }
}


#[allow(dead_code)]
pub fn from_gltf( g_primitive: &gltf::Primitive<'_>, imp: &ImportData, display: &glium::Display )-> FMesh
{
  let buffers = &imp.buffers;
  let reader = g_primitive.reader(|buffer| Some(&buffers[buffer.index()]));
  let positions = {
      let iter = reader
          .read_positions()
          .unwrap_or_else(||
              panic!("primitives must have the POSITION attribute (mesh: , primitive: )")
          );
      iter.collect::<Vec<_>>()
  };

  //TODO: this should be created immediately in earlier step
  let mut vertices: Vec<Vertex> = positions
      .into_iter()
      .map(|position| {
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


  let vbuffer = glium::vertex::VertexBuffer::new(display, &vertices).unwrap();
  let ibuffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &(indices).as_ref().unwrap().as_slice()).unwrap();
  let material = g_primitive.material().index();

  FMesh{ vbuffer, ibuffer, material }
}




pub fn load_object( filename: &str, display: &glium::Display ) -> FObject
{
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
      load_image(im.source().source(), display)
    })
    .filter_map(|a| a)
    .collect();


  let meshes: Vec<FMesh> = imp.doc.nodes()
    .filter_map(|n| n.mesh() )
    .map( |node| {
      from_gltf(&node.primitives().next().unwrap(), &imp, display)
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
      FMaterial { diffuse_texture, normal_texture, occlusion_texture, metallic_roughness_texture }
    })
    .collect();

  FObject { meshes, materials, textures }
}
