#[macro_use]
extern crate glium;
extern crate nalgebra as nalgebra;


use std::{fs, io};
use std::io::Cursor;
use std::boxed::Box;
use std::error::Error as StdError;

/*
extern crate vecmath;
extern crate camera_controllers;
extern crate shader_version;
*/

extern crate serde_json;

use std::path::Path;
use std::path::PathBuf;

use std::ffi::OsStr;
use std::time::{Duration, Instant};
use gltf::Gltf;


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

pub struct Root {

}

pub fn from_gltf(
  g_primitive: &gltf::Primitive<'_>,
//  primitive_index: usize,
//  mesh_index: usize,
//  root: &mut Root,
  imp: &ImportData,
//  base_path: &Path
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

  let g_material = g_primitive.material();

  let mut material = None;
  Primitive{ vertices, indices, mode, material }
}



//fn load_object(path: &str) -> Result<gltf::Gltf, Box<dyn StdError>> {
fn load_object(path: &str) -> Result<Gltf, Box<(dyn std::error::Error + 'static)>> {
  let file = fs::File::open(&path)?;
  let reader = io::BufReader::new(file);
  match gltf::Gltf::from_reader(reader) {
    Ok(p) => {
      Ok(p)
    }
    Err(e) => {
      std::process::exit(1)
    }
  }
}



fn print_tree(node: &gltf::Node, depth: i32) {
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
  if let Some(cam) = node.camera() {
    print!(" camera ");
  }
  if let Some(skin) = node.skin() {
    print!(" skin ");
  }
  println!();

  for child in node.children() {
    print_tree(&child, depth + 1);
  }
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

impl Default for Vertex {
  fn default() -> Self {
      Vertex {
          position: [0.0, 0.0, 0.0],
          normal: [0.0, 0.0, 1.0],
          tex_coords: [0.0, 0.0]
      }
  }
}





fn print_type_of<T>(_: &T, text: &str) {
  println!("{:?} {:?}", text, std::any::type_name::<T>())
}



fn load_shader_vf( filename: &str, display: &glium::Display ) -> glium::Program
{
  let pv = ["data/shaders/", filename, "-vert.glsl"].concat();
  println!("   read file {}", pv);
  let vs = fs::read_to_string(pv)
        .expect("Something went wrong reading the vert shader");
  let pf = ["data/shaders/", filename, "-frag.glsl"].concat();
  let fs = fs::read_to_string(pf)
        .expect("Something went wrong reading the vert shader");
  glium::Program::from_source(display, &vs, &fs, None).unwrap()
}


fn main() {
  #[allow(unused_imports)]
  use glium::{glutin, Surface};

  let event_loop = glutin::event_loop::EventLoop::new();
  let wb = glutin::window::WindowBuilder::new();
  let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
  let display = glium::Display::new(wb, cb, &event_loop).unwrap();

  println!("running!");
  let mut now = Instant::now();


  let source = "data/scene.gltf";
  let (doc, buffers, images) = match gltf::import(source) {
    Ok(tuple) => tuple,
    Err(err) => {
      println!("glTF import failed: {:?}", err);
      if let gltf::Error::Io(_) = err {
        println!("Hint: Are the .bin file(s) referenced by the .gltf file available?")
      }
      std::process::exit(1)
    },
  };
  println!("____ import: {}s", now.elapsed().subsec_nanos()/10000000);
  //now = Instant::now();
  let imp = ImportData { doc, buffers, images };
  //println!("___ doc : {:?}  meshes: {:?}", imp.doc, imp.doc.meshes());
  println!("___ imp.  doc: --  buffers: {:?} images: {:?} ", imp.buffers.len(), imp.images.len());


  let vals: Vec<Primitive> = imp.doc.nodes()
    .filter_map(|n| n.mesh() )
    .map( |node| {
      from_gltf(&node.primitives().next().unwrap(), &imp)
      })
    .collect::<Vec<_>>();


  print_type_of( &vals, "____ vals");

  println!("_____ vals vals:  {} ", vals.len());


  println!("from gltf {}s", now.elapsed().subsec_nanos()/10000000);
  //now = Instant::now();

  let shader = load_shader_vf( "test", &display);


  println!("shader_vf {}s", now.elapsed().subsec_nanos()/10000000);

  let obj = &vals[2];
  let vbuffer = glium::vertex::VertexBuffer::new(&display, &obj.vertices).unwrap();
  let ibuffer = glium::index::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &(obj.indices).as_ref().unwrap().as_slice()).unwrap();

  let file = fs::File::open("data/textures/aiStandardSurface1SG_baseColor.jpg").unwrap();
  let reader = io::BufReader::new(file);
  //let image__ = image::load_from_memory_with_format(reader.buffer(), image::ImageFormat::Jpeg);
  let image__ = image::load(reader, image::ImageFormat::Jpeg);
  let image_ = image__.unwrap();
  let image = image_.to_rgba8();
//    .unwrap()
//    .to_rgba8();
  let image_dimensions = image.dimensions();
  let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
  let diffuse_texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

  println!("textures loaded  {}", now.elapsed().subsec_nanos()/10000000);

  println!("___ display -> ");
  event_loop.run(move |event, _, control_flow| {
    let next_frame_time = std::time::Instant::now() +
        std::time::Duration::from_nanos(16_666_667);
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    match event {
        glutin::event::Event::WindowEvent { event, .. } => match event {
            glutin::event::WindowEvent::CloseRequested => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return;
            },
            _ => return,
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::ResumeTimeReached { .. } => (),
            glutin::event::StartCause::Init => (),
            _ => return,
        },
        _ => return,
    }

    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

    let model: [[f32;4];4] = nalgebra::Matrix4::<f32>::identity().into();

    let light = [1.4, 0.4, 0.7f32];
    let params = glium::DrawParameters {
      depth: glium::Depth {
          test: glium::draw_parameters::DepthTest::IfLess,
          write: true,
          .. Default::default()
      },
      .. Default::default()
    };


    let view_mat: [[f32; 4]; 4] = (nalgebra::Matrix4::<f32>::look_at_rh(
      &nalgebra::Point3::new( -3.0, -7.0, 3.0 ),
      &nalgebra::Point3::new( 0.0, 0.0, 3.0 ),
      &nalgebra::Vector3::new( 0.0, 0.0, 1.0 )
    )).into();

    let perspective: [[f32;4];4] = {
      let (width, height) = target.get_dimensions();
      //let aspect_ratio = height as f32 / width as f32;
      let aspect_ratio = width as f32 / height as f32;
      let fov: f32 = 3.141592 / 2.0;
      let znear = 0.1;
      let zfar = 1024.0;
      nalgebra::Matrix4::<f32>::new_perspective( aspect_ratio, fov, znear, zfar ).into()
      //(*nalgebra::Perspective3::new( aspect_ratio, fov, znear, zfar ).as_matrix() as nalgebra::Matrix4<f32>).into()
    };
    println!("____ persp: {:?}   size {:?}", perspective, target.get_dimensions());


    //for obj in &vals {
    //let obj = vals.first().unwrap();
      //target.draw(&vbuffer, glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip), &shader,
      target.draw(&vbuffer, &ibuffer, &shader,
        &uniform! { model: model, view: view_mat, perspective: perspective,
                  u_light: light, diffuse_tex: &diffuse_texture, normal_tex: &diffuse_texture },
        &params).unwrap();
    //};

    target.finish().unwrap();
  });


}
