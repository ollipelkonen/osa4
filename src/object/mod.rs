
extern crate glium;
use gltf::Gltf;
use std::{fs, io};


#[allow(dead_code)]
pub struct Obu {
  pub materials: Vec<Material>,
  //pub textures: Vec<gltf::texture::Texture<a§>>,
  pub meshes: Vec<Mesh>
}

pub struct Material {
  pub diffuse_texture: Option<u32>,
  pub normal_texture: Option<u32>,
  pub occlusion_texture: Option<u32>,
  pub metallic_roughness_texture: Option<u32>  // unused
}

pub struct Mesh {
  pub vertex_buffer: glium::vertex::VertexBuffer<f32>,
  pub index_buffer: glium::index::IndexBuffer<u32>,
  pub material: Material
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

pub struct Primitive {
  pub vertices: Vec<Vertex>,
  pub indices: Option<Vec<u32>>,
  pub mode: u32,
  pub material: Option<u32>
}



pub fn load_image(a: gltf::image::Source, display: &glium::Display) -> Option<glium::texture::SrgbTexture2d> {
  match a {
    gltf::image::Source::Uri{uri, mime_type} => {
      println!("____ texture found {}   mime: {:?}", uri, mime_type);
      let file = fs::File::open("data/textures/aiStandardSurface1SG_baseColor.jpg").unwrap();
      let reader = io::BufReader::new(file);
      let image = image::load(reader, image::ImageFormat::Jpeg).unwrap().to_rgba8();
      let image_dimensions = image.dimensions();
      let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
      Some(glium::texture::SrgbTexture2d::new(display, image).unwrap())
    },
    _ => None
  }
}


#[allow(dead_code)]
pub fn from_gltf(
  g_primitive: &gltf::Primitive<'_>,
//  primitive_index: usize,
//  mesh_index: usize,
//  root: &mut Root,
  imp: &ImportData,
//  base_path: &Path
  display: &glium::Display
  )-> Primitive
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

  let mut vertices: Vec<Vertex> = positions
      .into_iter()
      .map(|position| {
          /*Vertex {
              position: position,
              ..Vertex::default()
          }*/
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

  /*let minx = vertices.iter().fold(1.0f32, |min_val, &val| min_val.min(val.tex_coords[0]).min(val.tex_coords[1]));
  let maxx = vertices.iter().fold(0.0f32, |max_val, &val| max_val.max(val.tex_coords[0]).max(val.tex_coords[1]));
  let minu = vertices.iter().fold(1.0f32, |min_val, &val| min_val.min(val.tex_coords[0]));
  let maxu = vertices.iter().fold(0.0f32, |max_val, &val| max_val.max(val.tex_coords[0]));
  let minv = vertices.iter().fold(1.0f32, |min_val, &val| min_val.min(val.tex_coords[1]));
  let maxv = vertices.iter().fold(0.0f32, |max_val, &val| max_val.max(val.tex_coords[1]));
  println!("____ uv moin/max {} / {}", minx,maxx);
  println!("____ uv moin/max U {} / {}", minu,maxu);
  println!("____ uv moin/max U {} / {}", minv,maxv);*/

  let indices = reader
    .read_indices()
    .map(|read_indices| {
      read_indices.into_u32().collect::<Vec<_>>()
    });

  let mode = g_primitive.mode().as_gl_enum();

  let g_material: gltf::Material = g_primitive.material();

  println!("____ g_material? {:?}", g_primitive.material().index().unwrap());
  //normal_texture(), occlusion_texture()
  /*println!("__ json: {:?}", g_primitive.material().extras());
  let occl = match g_primitive.material().occlusion_texture().unwrap().texture().source().source() {
    gltf::image::Source::View{view, mime_type} => mime_type,
    gltf::image::Source::Uri{uri, mime_type} => uri
  };
  println!("___ material {:?}  : normal? {:?}   occlusion?   {:?}   {:?} ",
    g_primitive.material().index().unwrap(), g_primitive.material().normal_texture().unwrap().texture().extras(), occl, g_primitive.material().name().unwrap()
  );*/

  //TODO: is this always correct? wtf?
  //let im = imp.images[];

  // base color texture
/*  let texture_diffuse = match g_primitive.material().pbr_metallic_roughness().base_color_texture() {
    Some(d) => {
      match d.texture().source().source() {
        gltf::image::Source::Uri{uri, mime_type: _} => {
          let file = fs::File::open("data/textures/aiStandardSurface1SG_baseColor.jpg").unwrap();
          let reader = io::BufReader::new(file);
          //let image__ = image::load_from_memory_with_format(reader.buffer(), image::ImageFormat::Jpeg);
          let image = image::load(reader, image::ImageFormat::Jpeg).unwrap().to_rgba8();
          let image_dimensions = image.dimensions();
          let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
          Some(glium::texture::SrgbTexture2d::new(&display, image).unwrap())
        },
        _ => None
      },
      _ => None
    }
  };*/

  let texture_diffuse = match g_primitive.material().pbr_metallic_roughness().base_color_texture() {
    Some(d) => {
      println!("   diffuse found. id: {}", d.texture().index());
      load_image(d.texture().source().source(), display)
    },
    _ => None
    };
  let texture_occlusion = match g_primitive.material().occlusion_texture() {
    Some(d) => load_image(d.texture().source().source(), display),
    _ => None
    };


  println!("___ texture_diffuse: {:?}", texture_diffuse);

//  gltf::texture::Info i
  let mut material = None;
  Primitive{ vertices, indices, mode, material }
}




type Import = (gltf::Document, Vec<gltf::buffer::Data>, Vec<gltf::image::Data>);

//pub fn load_object( filename: &str ) -> Result<(gltf::Document, Vec<gltf::buffer::Data>, Vec<gltf::buffer::Data>)>
pub fn load_object( filename: &str, display: &glium::Display ) -> ImportData
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
  println!("___ imp.  doc: --  buffers: {:?} images: {:?} ", imp.buffers.len(), imp.images.len());

/*  for m in doc.materials() {
    println!("____ mat {:?} ", m);
  };*/

  let textures: Vec<glium::texture::SrgbTexture2d> = imp.doc.textures().map( |im| {
      load_image(im.source().source(), display)
    })
    .filter_map(|a| a)
    .collect();

  println!("___ found some textures: {:?}", textures);

  let vals: Vec<Primitive> = imp.doc.nodes()
    .filter_map(|n| n.mesh() )
    .map( |node| {
      from_gltf(&node.primitives().next().unwrap(), &imp, display)
      })
    .collect::<Vec<_>>();


//  let images =
  let obj = &vals[2];
  let vbuffer = glium::vertex::VertexBuffer::new(display, &obj.vertices).unwrap();
  let ibuffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &(obj.indices).as_ref().unwrap().as_slice()).unwrap();

  let file = fs::File::open("data/textures/aiStandardSurface1SG_baseColor.jpg").unwrap();
  let reader = io::BufReader::new(file);
  //let image__ = image::load_from_memory_with_format(reader.buffer(), image::ImageFormat::Jpeg);
  let image = image::load(reader, image::ImageFormat::Jpeg).unwrap().to_rgba8();


  let image_dimensions = image.dimensions();
  let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
//  let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();



//  (doc, buffers, images)
  imp
}
