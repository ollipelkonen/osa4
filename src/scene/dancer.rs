
use nalgebra::Matrix4;
use rapier3d::prelude::*;
use crate::glium::Surface;
use crate::f::FObject::FObject;
use crate::f::World::World;

//use crate::f::FObject;
//use crate::f::FMesh;
//use crate::f::World;


pub struct dancer {
  pub balls: std::vec::Vec<RigidBodyHandle>,
  pub obj_sphere: Option<FObject>,
  pub shader: std::option::Option::<glium::Program>,
}



impl dancer{

  pub fn new() -> Self {
    Self{
      balls: Vec::new(),
      obj_sphere: Option::None::<crate::f::FObject::FObject>,
      shader: Option::None::<glium::Program>
    }
  }



  pub fn create_ball(&mut self, pos: nalgebra::Vector3<f32>, radius: Real, previous: Option<RigidBodyHandle>, world: &mut World, dynamic: bool) -> RigidBodyHandle
  {

    let rigid_body = match dynamic {
      true => RigidBodyBuilder::dynamic(),
      false => RigidBodyBuilder::kinematic_position_based()
      }.translation( vector![pos.x, pos.y, pos.z] )
        .build();

    let collider = ColliderBuilder::ball(radius).restitution(0.7).mass(10.0).build();
    let ball_body_handle = world.rigid_body_set.insert(rigid_body);
    world.collider_set.insert_with_parent(collider, ball_body_handle, &mut world.rigid_body_set);

    //rigid_body.
    if let Some(prev) = previous {
      let k1 = point![pos.x, pos.y, pos.z];
      let k2 = point![pos.x, pos.y-2.5, pos.z];
      let joint = SphericalJointBuilder::new()
        .local_anchor1(k1)
        .local_anchor2(k2);
        //.local_anchor1(point![0.0, 0.0, -3.0])
        //.local_anchor2(point![0.0, 0.0, 1.0]);
      world.impulse_joint_set.insert(ball_body_handle, prev, joint, true);
    }
    ball_body_handle
  }


  pub fn init_balls(&mut self, display: &glium::Display, world: &mut World) {
    self.shader = Some(crate::f::shader::create_shader_vf( &display, "test" ));

    let mut previous: Option<RigidBodyHandle> = None;
    self.balls = (1..10).map( |n| {
      let pos = nalgebra::Vector3::new( 0.0, n as f32 * 2.5 - 1.0, 0.0 );
      previous = Some(self.create_ball( pos, 0.8, previous, world, n>1 ));
      previous.unwrap()
    } )
    .collect();
    self.obj_sphere = Some(crate::f::FObject::FObject::load_gltf( "data/sphere.gltf", &display ));

    if let Some(obj) = &mut self.obj_sphere {
      obj.set_texture( "texture.jpg", display );
    }

  }
  

  pub fn render_balls<'b>(&mut self, world: &mut World, target: &mut glium::Frame, time: f32, view_mat: [[f32;4];4], perspective_mat: [[f32;4];4]) {
    let light = [1.4, 0.4, 0.7f32];
    let draw_params = glium::DrawParameters {
      depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: true,
        .. Default::default()
      },
      .. Default::default()
    };

    let gravity = vector![0.0, -9.81, 0.0];

    world.physics_pipeline.step(
      &gravity,
      &world.integration_parameters,
      &mut world.island_manager,
      &mut world.broad_phase,
      &mut world.narrow_phase,
      &mut world.rigid_body_set,
      &mut world.collider_set,
      &mut world.impulse_joint_set,
      &mut world.multibody_joint_set,
      &mut world.ccd_solver,
      &world.physics_hooks,
      &world.event_handler,
    );


    if let Some(obj) = &self.obj_sphere {
      //println!("EFKFE");
      if let Some(shader) = &self.shader {
        self.balls.iter().for_each( |b| {
          let ball = &world.rigid_body_set[*b];

          let pos = ball.translation();
          for mesh in &obj.meshes {
            let m4: Matrix4<f32> = mesh.matrix * obj.matrix;
            let model_matrix: [[f32;4];4] = (m4 * nalgebra::Matrix4::<f32>::new_translation(pos).append_scaling(0.4)).into();
            target.draw(&mesh.vbuffer, &mesh.ibuffer, &shader,
              &uniform! { model: model_matrix, view: view_mat, perspective: perspective_mat,
                      u_light: light,
                      diffuse_tex: &obj.textures[obj.materials[mesh.material.unwrap()].diffuse_texture.unwrap()]//,
                      //normal_tex: &obj.textures[obj.materials[mesh.material.unwrap()].normal_texture.unwrap()]
              },
              &draw_params);
          }

        } );
      }
    }
  }



}
