
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



  pub fn create_ball(&mut self, pos: nalgebra::Vector3<f32>, radius: Real, previous: Option<RigidBodyHandle>, previous_pos: nalgebra::Vector3<f32>, world: &mut World, dynamic: bool) -> RigidBodyHandle
  {

    let rigid_body = match dynamic {
      true => RigidBodyBuilder::dynamic(),
      false => RigidBodyBuilder::dynamic() //.gravity_scale(0.0)
      //false => RigidBodyBuilder::kinematic_position_based()
      //false => RigidBodyBuilder::kinematic_velocity_based()
      //false => RigidBodyBuilder::fixed()
    }.translation( pos )
      .angular_damping(0.1)
      .linear_damping(0.1)
      .ccd_enabled(true) 
      .build();

    //println!("{:?}",rigid_body);
    let ball_body_handle = world.rigid_body_set.insert(rigid_body);

    let collider = ColliderBuilder::ball(radius);//.restitution(0.1).mass(0.1).build();
    world.collider_set.insert_with_parent(collider, ball_body_handle, &mut world.rigid_body_set);

    //rigid_body.
    if let Some(prev) = previous {
      let joint = SphericalJointBuilder::new()
        .local_anchor2( point![previous_pos.x, previous_pos.y, previous_pos.z] );
        //.contacts_enabled(false)
        /*.limits(JointAxis::X, [-1.0, 1.0])
        .limits(JointAxis::Y, [-1.0, 1.0])
        .limits(JointAxis::Z, [-1.0, 1.0]);*/
      //let joint1 = GenericJointBuilder::new(JointAxesMask::empty())
        /*.limits(JointAxis::X, [-3.0, 3.0])
        .limits(JointAxis::Y, [0.0, 3.0])
        .limits(JointAxis::Z, [0.0, 3.0])*/
        //.coupled_axes(JointAxesMask::X | JointAxesMask::Y | JointAxesMask::Z);
        //.local_anchor1(point![0.0, 0.0, -3.0])
        //.local_anchor2(point![0.0, 0.0, 1.0]);
      world.impulse_joint_set.insert(ball_body_handle, prev, joint, false);
    }
    ball_body_handle
  }


  pub fn init_balls(&mut self, display: &glium::Display, world: &mut World) {
    self.shader = Some(crate::f::shader::create_shader_vf( &display, "test" ));
    let radius = 0.2;

    let first_pos = nalgebra::Vector3::new(0.0,2.2,0.0);
    let first = Some(self.create_ball( first_pos, 0.2, None, first_pos, world, false ));
    world.collider_set.iter_mut().nth(0).unwrap().1.set_mass(10000.0);
    self.balls.push( first.unwrap() );

    let x_count = 20;
    let y_count = 10;
    for m in 0..x_count {
      //let mut previous: Option<RigidBodyHandle> = None;
      //let mut previous_pos: Option<nalgebra::Vector3<f32>> = None;
      let mut previous: Option<RigidBodyHandle> = first;
      let mut previous_pos: Option<nalgebra::Vector3<f32>> = Some(first_pos);
      for n in 0..y_count {
        let dist = 5.0 + n as f64 / 1.0;
        let (x, z) = (
          (m as f64 / x_count as f64 * std::f64::consts::PI * 2.0).sin() * dist,
          (m as f64 / x_count as f64 * std::f64::consts::PI * 2.0).cos() * dist
        );
        let pos = nalgebra::Vector3::new( x as f32, n as f32 * -2.5 + 2.2, z as f32 );
        if previous_pos == None {
          previous = Some(self.create_ball( pos, radius, previous, pos, world, n>1 ));
        } else {
          previous = Some(self.create_ball( pos, radius, previous, pos-previous_pos.unwrap(), world, n>1 ));
        }
        self.balls.push( previous.unwrap() );
        previous_pos = Some(pos);
      }
    }

    for m in 0..(x_count-1) {
      for n in 0..(y_count-1) {
        //println!("{:?}", self.balls[m*y_count+n+1]);
        let b = self.balls[m*y_count+n+1];
        let b2 = self.balls[((m+1)%x_count)*y_count+n+1];
        let tr = world.rigid_body_set.get(b).unwrap().translation() - world.rigid_body_set.get(b2).unwrap().translation();
        let joint = SphericalJointBuilder::new()
          .local_anchor2( point![ tr.x, tr.y, tr.z ] );
        world.impulse_joint_set.insert(b, b2, joint, true);
      }
    }
    
    let middle = Some(self.create_ball( nalgebra::Vector3::new(0.0,-5.0,0.0), 2.0, None, first_pos, world, false ));
    self.balls.push( middle.unwrap() );

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
          if world.rigid_body_set.contains(*b) {
            let ball = &world.rigid_body_set[*b];

            //println!(" ball;: {:?} ", ball);

            if ball.colliders().len() > 0 {
              let radius = world.collider_set.get(ball.colliders()[0]).unwrap().shape().as_ball().unwrap().radius;
              //println!("{:?}", radius);
              let pos = ball.translation();
              for mesh in &obj.meshes {
                // center object
                let v1 = pos + nalgebra::Vector3::new( 0.0, -1.0, 0.0 );
                let scale = nalgebra::Matrix4::<f32>::new_scaling(radius*2.0);
                let position = nalgebra::Matrix4::<f32>::new_translation(&v1);
                let m4: Matrix4<f32> = mesh.matrix * obj.matrix * position.append_scaling(0.4) * scale;
                let model_matrix: [[f32;4];4] = m4.into();
                target.draw(&mesh.vbuffer, &mesh.ibuffer, &shader,
                  &uniform! { model: model_matrix, view: view_mat, perspective: perspective_mat,
                          u_light: light,
                          diffuse_tex: &obj.textures[obj.materials[mesh.material.unwrap()].diffuse_texture.unwrap()]//,
                          //normal_tex: &obj.textures[obj.materials[mesh.material.unwrap()].normal_texture.unwrap()]
                  },
                  &draw_params);
              }
            }
          }

        } );
      }
    }
  }



}
