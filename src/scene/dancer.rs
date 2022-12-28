
use nalgebra::Matrix4;
use rapier3d::prelude::*;
use crate::glium::Surface;
use crate::f::fobject::FObject;
use crate::f::world::World;

//use crate::f::FObject;
//use crate::f::FMesh;
//use crate::f::World;


pub struct Dancer {
  pub balls: std::vec::Vec<RigidBodyHandle>,
  pub obj_sphere: Option<FObject>,
  pub shader: std::option::Option::<glium::Program>,
}



impl Dancer{

  pub fn new() -> Self {
    Self{
      balls: Vec::new(),
      obj_sphere: Option::None::<crate::f::fobject::FObject>,
      shader: Option::None::<glium::Program>
    }
  }



  pub fn create_ball(&mut self, pos: nalgebra::Vector3<f32>, radius: Real, previous: Option<RigidBodyHandle>, world: &mut World, dynamic: bool) -> RigidBodyHandle
  {

    let rigid_body = match dynamic {
      true => RigidBodyBuilder::dynamic(),
      false => RigidBodyBuilder::dynamic().gravity_scale(0.0)
      //false => RigidBodyBuilder::kinematic_position_based()
      //false => RigidBodyBuilder::kinematic_velocity_based()
      //false => RigidBodyBuilder::fixed()
    }.translation( pos )
      .lock_translations()
      .angular_damping(0.01)
      .linear_damping(0.01)
      //.lock_rotations()
      //.enabled_rotations(false, true, false)
      .ccd_enabled(true) 
      .build();

    let collider = ColliderBuilder::ball(radius).restitution(0.1).mass(10000.0).translation(pos).build();
    let ball_body_handle = world.rigid_body_set.insert(rigid_body);
    world.collider_set.insert_with_parent(collider, ball_body_handle, &mut world.rigid_body_set);

    //rigid_body.
    if let Some(prev) = previous {
      //let k1 = point![radius/2.0, radius/2.0, radius/2.0];
      //let k2 = point![radius/2.0, radius/2.0, radius/2.0];
      /*let (k1,k2) = (
        point![0.0, -radius / 2.0, 0.0],
        point![0.0, radius / 2.0, 0.0]
      );*/
      /*let joint = SphericalJointBuilder::new()
        .local_anchor1(k1)
        .local_anchor2(k2)*/
        /*.limits(JointAxis::X, [-1.0, 1.0])
        .limits(JointAxis::Y, [-1.0, 1.0])
        .limits(JointAxis::Z, [-1.0, 1.0])*/;
      let joint1 = GenericJointBuilder::new(JointAxesMask::empty())
        /*.limits(JointAxis::X, [-3.0, 3.0])
        .limits(JointAxis::Y, [0.0, 3.0])
        .limits(JointAxis::Z, [0.0, 3.0])*/
        .coupled_axes(JointAxesMask::Y | JointAxesMask::Z);

        //.local_anchor1(point![0.0, 0.0, -3.0])
        //.local_anchor2(point![0.0, 0.0, 1.0]);
      world.impulse_joint_set.insert(ball_body_handle, prev, joint1, true);
    }
    ball_body_handle
  }


  pub fn init_balls(&mut self, display: &glium::Display, world: &mut World) {
    self.shader = Some(crate::f::shader::create_shader_vf( &display, "test" ));
    let radius = 0.2;

    let first = Some(self.create_ball( nalgebra::Vector3::new(0.0,0.0,0.0), radius, None, world, false ));
    self.balls.push( first.unwrap() );

    for m in 1..10 {
      //let mut previous: Option<RigidBodyHandle> = None;
      let mut previous: Option<RigidBodyHandle> = first;
      for n in 1..10 {
        let dist = 5.0 + n as f64 / 2.0;
        let (x, z) = (
          (m as f64 / 9.0 * std::f64::consts::PI * 2.0).sin() * dist,
          (m as f64 / 9.0 * std::f64::consts::PI * 2.0).cos() * dist
        );
        let pos = nalgebra::Vector3::new( x as f32, n as f32 * -2.5 + 2.2, z as f32 );
        previous = Some(self.create_ball( pos, radius, previous, world, n>1 ));
        self.balls.push( previous.unwrap() );
      }
    }
    for m in 0..9 {
      for n in 0..9 {
        println!("{:?}", self.balls[m*9+n]);
      }
    }

    self.obj_sphere = Some(crate::f::fobject::FObject::load_gltf( "data/sphere.gltf", &display ));

    if let Some(obj) = &mut self.obj_sphere {
      obj.set_texture( "texture.jpg", display );
    }

  }
  

  pub fn render_balls<'b>(&mut self, world: &mut World, target: &mut glium::Frame, _time: f32, view_mat: [[f32;4];4], perspective_mat: [[f32;4];4]) {
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
