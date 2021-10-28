//pub use obj::*;
//mod obj;


extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
#[macro_use]
extern crate gfx;
extern crate shader_version;

extern crate collada;
extern crate serde_json;


use collada::*;

use std::path::Path;
use std::ffi::OsStr;


use piston_window::*;
use gfx::traits::*;
use shader_version::Shaders;
use shader_version::glsl::GLSL;

/*
pub struct Vector3 { x: f32, y: f32, z: f32, }

impl From<&collada::Vertex> for Vector3 {
  Vector3 {
      x: vertex.x as f32,
      y: vertex.y as f32,
      z: vertex.z as f32,
  }
}

*/


fn print_type_of<T>(_: &T, text: &str) {
  println!("{:?} {}", text, std::any::type_name::<T>())
}
/*fn print_type_of<T>(_: &T, text: Option<String>) {
  println!("{:?} {}", text, std::any::type_name::<T>())
}*/
/*fn print_type_of<T>(t: &T) {
  print_type_of(t, None);
  //println!("{}", std::any::type_name::<T>())
}*/



//impl create_object for

fn main() {
  println!("satan!");
//  let plague = collada::document::ColladaDocument::from_path( Path::new("./plague.dae"));

  let plague = match collada::document::ColladaDocument::from_path( Path::new("./data/plague.dae")) {
    Ok(p) => {
      p
    }
    Err(e) => {
      std::process::exit(1)
    }
  };

  let s = match plague.get_obj_set() {
    Some(p) => {
      p.objects.into_iter().map( |obu|
        obu.geometry.into_iter().map( |geo|
          geo.mesh.into_iter().map( |elem|
            match elem {
              PrimitiveElement::Triangles(v) => {
                v.vertices
              },
              _ => {
                println!("no");
                std::process::exit(1)
              }
            }
          )
        )
      )
    }
    None => {
      std::process::exit(1)
    }
  };


  print_type_of(&s, "obu__ ");
  //println!("__ dataa: {:?}", s.collect());
//  for obu in s.objects.iter() {

//      print_type_of(&obbu, "__ blaaa ");

/*  for obu in s.objects.iter() {
    println!("___ obj {:?} {}  tex: {}", obu.id, obu.vertices.len(), obu.tex_vertices.len() );

    //println!("    mesh {:?} {}", geo.vertices.len(), geo.vertices.len() );
    for geo in obu.geometry.iter() {
      //println!("    mesh {:?} {}", geo.mesh[0].vertices.len(), geo.mesh.len() );
      for elem in geo.mesh.iter() {
        let y = match elem {
          PrimitiveElement::Triangles(v) => {
            &v.vertices
          },
          _ => {
            println!("no");
            std::process::exit(1)
          }
        };

        let minx = y.iter().fold(0, |min_val, &val| min_val.min(val.0).min(val.1).min(val.2));
        let maxx = y.iter().fold(0, |min_val, &val| min_val.min(val.0).max(val.1).max(val.2));
        println!("   min. {:?}  max. {:?}", minx, maxx );
      }

      /*for elem in geo.mesh.iter() {
        let y = match elem {
          PrimitiveElement::Triangles(v) => {
            &v.vertices
          },
          _ => {
            println!("no");
            std::process::exit(1)
          }
        };
        println!("triangles {:?}", y);
      }*/
      //print_type_of(&geo.mesh[0]);
    }

  }*/

  println!("{:?}", plague.get_images());
  println!("grr");


  /*let events_loop = winit::EventsLoop::new();
  let monitor: winit::MonitorId = events_loop.get_primary_monitor();
  let (screen_width, screen_height) = monitor.get_dimensions();*/
  let screen_width = 800;
  let screen_height = 600;

  let opengl = OpenGL::V3_2;
  let mut window: PistonWindow = WindowSettings::new("f:", [screen_width,screen_height])
      .exit_on_esc(true)
      .graphics_api(opengl)
      .fullscreen(false)
      .build()
      .unwrap();


    let mut factory = window.factory.clone();

    while let Some(e) = window.next() {

      window.draw_3d(&e, |window| {
        let args = e.render_args().unwrap();

        window.encoder.clear(&window.output_color, [0.3, 0.3, 0.3, 1.0]);
        window.encoder.clear_depth(&window.output_stencil, 1.0);

        /*for obu in s.objects.iter() {
          let vertex_data = obu.vertices;
          let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);
          println!("___ obj {:?} {}", obu.id, obu.vertices.len() );
        }*/
              

        /*data.u_model_view_proj = model_view_projection(
          model,
          first_person.camera(args.ext_dt).orthogonal(),
          projection
        );
        window.encoder.draw(&slice, &pso, &data);*/
      });

      if e.resize_args().is_some() {
        /*projection = get_projection(&window);
        data.out_color = window.output_color.clone();
        data.out_depth = window.output_stencil.clone();*/
      }
    }
}
