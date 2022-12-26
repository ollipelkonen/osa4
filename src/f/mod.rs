
extern crate glium;
use std::{fs, io};
use crate::f::Vertex;

//use crate::f::FMesh;
//use f::FMesh;
//use crate::f::Vertex;


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
  FMesh{ vbuffer: vbuffer, ibuffer: ibuffer, material, vertices: Some(vertices), indices: Some(indices.unwrap()),
    bounds: Some(bounds), edges: None, matrix: nalgebra::Matrix4::<f32>::identity() }
}




