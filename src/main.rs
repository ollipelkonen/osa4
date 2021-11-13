#[macro_use]
extern crate glium;
extern crate nalgebra as nalgebra;

pub mod object;

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



#[allow(dead_code)]
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





#[allow(dead_code)]
fn print_type_of<T>(_: &T, text: &str) {
  println!("{:?} {:?}", text, std::any::type_name::<T>())
}



fn load_shader_vf( filename: &str, display: &glium::Display ) -> glium::Program
{
  let pv = ["data/shaders/", filename, "-vert.glsl"].concat();
  let vs = fs::read_to_string(pv)
        .expect("Something went wrong reading the vert shader");
  let pf = ["data/shaders/", filename, "-frag.glsl"].concat();
  let fs = fs::read_to_string(pf)
        .expect("Something went wrong reading the vert shader");
  glium::Program::from_source(display, &vs, &fs, None).unwrap()
}


fn main() {
  #[allow(unused_imports)]
  let mut now = Instant::now();
  use glium::{glutin, Surface};

  let event_loop = glutin::event_loop::EventLoop::new();
  let wb = glutin::window::WindowBuilder::new();
  let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
  let display = glium::Display::new(wb, cb, &event_loop).unwrap();

  let obj = object::load_object( "data/scene.gltf", &display );
  let shader = load_shader_vf( "test", &display);
  println!("shader_vf {}s", now.elapsed().subsec_nanos()/10000000);

  println!("___ display  {:?}s -> ", now.elapsed().subsec_nanos() as f32/100000000.0);
//  std::process::exit(1);

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


    obj.meshes.iter().for_each( |mesh| {
      let mut uniforms = uniform! { model: model, view: view_mat, perspective: perspective,
        u_light: light,
      };
/*      if let mat = mesh.material.unwrap() {
        if let diffuse_texture = obj.materials[mat].diffuse_texture.unwrap() {
          uniforms = uniforms.add("diffuse_texture", &obj.textures[diffuse_texture]);
        }
      }*/
//      diffuse_tex: &obj.textures[obj.materials[mesh.material.unwrap()].diffuse_texture.unwrap()],
  //    normal_tex: &obj.textures[obj.materials[mesh.material.unwrap()].normal_texture.unwrap()]

      target.draw(&mesh.vbuffer, &mesh.ibuffer, &shader,
        &uniform! { model: model, view: view_mat, perspective: perspective,
                  u_light: light,
                  diffuse_tex: &obj.textures[obj.materials[mesh.material.unwrap()].diffuse_texture.unwrap()],
                  normal_tex: &obj.textures[obj.materials[mesh.material.unwrap()].normal_texture.unwrap()]
        }, &params
      ).unwrap();
    });


    target.finish().unwrap();
  });


}
