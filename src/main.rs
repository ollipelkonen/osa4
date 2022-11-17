#[macro_use]
extern crate glium;
extern crate nalgebra as nalgebra;

pub mod f;

extern crate serde_json;

use std::ffi::OsStr;
use std::time::{Duration, Instant};


#[allow(dead_code)]
fn print_type_of<T>(_: &T, text: &str) {
  println!("{:?} {:?}", text, std::any::type_name::<T>())
}




fn main() {
  //std::process::exit(1);

  #[allow(unused_imports)]
  let mut now = Instant::now();
  let start = Instant::now();
  use glium::{glutin, Surface};

  let event_loop = glutin::event_loop::EventLoop::new();
  let wb = glutin::window::WindowBuilder::new();
  let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
  let display = glium::Display::new(wb, cb, &event_loop).unwrap();

  println!("___ joku.glsl {:?}s -> ", now.elapsed().as_nanos() as f32/100000000.0);
  let sdf = f::sdf::create( &display, "joku.glsl" );
  println!("___ scne.gltf  {:?}s -> ", now.elapsed().as_nanos() as f32/100000000.0);
  let obj = f::FObject::load_gltf( "data/scene.gltf", &display );
  println!("___ shader  {:?}s -> ", now.elapsed().as_nanos() as f32/100000000.0);
  let shader = f::shader::create_shader_vf( &display, "test" );


  //println!("___ sphere.gltf  {:?}s -> ", now.elapsed().as_nanos() as f32/100000000.0);
  //TODO: center is in 0,1,0
  //let obj_sphere = Some(f::FObject::load_gltf( "data/sphere.gltf", &display ));

  println!("___ physics  {:?}s -> ", now.elapsed().as_nanos() as f32/100000000.0);
//  std::process::exit(1);


  let mut world: f::physics::World = f::physics::World::new();
  world.init_balls(&display);
  //world.obj_sphere = obj_sphere;
  //let mut balls: Vec<RigidBodyHandle>{>::new();


  println!("___ display  {:?}s -> ", now.elapsed().as_nanos() as f32/100000000.0);

  event_loop.run(move |event, _, control_flow| {
    //TODO: i don't want any events. fuck this.
    let next_frame_time = std::time::Instant::now();
    *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

    match event {
      glutin::event::Event::WindowEvent { event, .. } => match event {
        glutin::event::WindowEvent::CloseRequested => {
          *control_flow = glutin::event_loop::ControlFlow::Exit;
          return;
        },
        glutin::event::WindowEvent::KeyboardInput { input, .. } => if input.state == glutin::event::ElementState::Pressed {
          if let Some(key) = input.virtual_keycode {
            match key {
              glutin::event::VirtualKeyCode::Escape => {
                *control_flow = glutin::event_loop::ControlFlow::Exit;
                return
              },
              _ => {}
            }
          }
        },
        _ => return,
      },
      glutin::event::Event::NewEvents(cause) => match cause {
        glutin::event::StartCause::ResumeTimeReached { .. } => (),
        glutin::event::StartCause::Init => (),
        _ => return,
      },
      glutin::event::Event::RedrawRequested(_) => (),
      _ => return,
    }


    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);


    let draw_params = glium::DrawParameters {
      depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: true,
        .. Default::default()
      },
      .. Default::default()
    };

    let draw_params_sdf = glium::DrawParameters {
      backface_culling: glium::draw_parameters::BackfaceCullingMode::CullingDisabled,
      depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: false,
        .. Default::default()
      },
      .. Default::default()
    };

    let time = start.elapsed().as_nanos() as f32/1000000000.0f32;

    let light = [1.4, 0.4, 0.7f32];

    let model_mat: [[f32;4];4] = nalgebra::Matrix4::<f32>::identity().into();

    let perspective_mat: [[f32;4];4] = {
      let (width, height) = target.get_dimensions();
      //let aspect_ratio = height as f32 / width as f32;
      let aspect_ratio = width as f32 / height as f32;
      let fov: f32 = 3.141592 / 2.0;
      let znear = 0.1;
      let zfar = 1024.0;
      nalgebra::Matrix4::<f32>::new_perspective( aspect_ratio, fov, znear, zfar ).into()
      //(*nalgebra::Perspective3::new( aspect_ratio, fov, znear, zfar ).as_matrix() as nalgebra::Matrix4<f32>).into()
    };

    let cx = 13.0f32 * time.sin();
    let cy = 18.0f32 * time.cos();
    let view_mat: [[f32; 4]; 4] = (nalgebra::Matrix4::<f32>::look_at_rh(
      &nalgebra::Point3::new( 2.0+cx, cy, 3.0 ),
      &nalgebra::Point3::new( 2.0, 0.0, 3.0 ),
      &nalgebra::Vector3::new( 0.0, 0.0, 1.0 )
    )).into();


    target.draw(&sdf.mesh.vbuffer, &sdf.mesh.ibuffer, &sdf.shader,
        &uniform! { time: time
        }, &draw_params_sdf
      ).unwrap();

      obj.meshes.iter().for_each( |mesh| {
      target.draw(&mesh.vbuffer, &mesh.ibuffer, &shader,
        &uniform! { model: model_mat, view: view_mat, perspective: perspective_mat,
                  u_light: light,
                  diffuse_tex: &obj.textures[obj.materials[mesh.material.unwrap()].diffuse_texture.unwrap()],
                  normal_tex: &obj.textures[obj.materials[mesh.material.unwrap()].normal_texture.unwrap()]
        }, &draw_params
      ).unwrap();
    });


    /*let cx = 3.0f32 * time.sin();
    let cy = 8.0f32 * time.cos();
    let view_mat_2: [[f32; 4]; 4] = (nalgebra::Matrix4::<f32>::look_at_rh(
      &nalgebra::Point3::new( 2.0, 0.0, 3.0 ),
      &nalgebra::Point3::new( 2.0+cx, cy, 3.0 ),
      &nalgebra::Vector3::new( 0.0, 0.0, 1.0 )
    )).into();*/

    world.render_balls( &target, time, perspective_mat );
    //TODO: |n| may outlive borrowed value `world`
    /*balls.for_each( |b| {
      let ball = &world.rigid_body_set[b];
    } );*/
    //println!("ekeke {:?}", world.rigid_body_set.);

    /*for n in 1..10 {
      let pos = &nalgebra::Vector3::new( 0.0, 0.0, n as f32 * -2.5 + 1.0 );
      let model_mat_2: [[f32;4];4] = nalgebra::Matrix4::<f32>::new_translation(pos).append_scaling(0.4).into();
      obj_sphere.meshes.iter().for_each( |mesh| {
        target.draw(&mesh.vbuffer, &mesh.ibuffer, &shader,
          &uniform! { model: model_mat_2, view: view_mat, perspective: perspective_mat,
                    u_light: light,
                    diffuse_tex: &obj.textures[obj.materials[mesh.material.unwrap()].diffuse_texture.unwrap()],
                    normal_tex: &obj.textures[obj.materials[mesh.material.unwrap()].normal_texture.unwrap()]
          }, &draw_params
        ).unwrap();
      });
    }*/

    target.finish().unwrap();

    //println!("___ display  {:?}s -> ", now.elapsed().as_nanos() as f32/100000000.0);
    //now = Instant::now();
  });


}
