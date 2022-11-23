extern crate glium;
use nalgebra::Matrix4;
use rapier3d::prelude::*;
use crate::glium::Surface;

//pub mod f;
use crate::f::FObject;
use crate::f;


pub struct World {
  pub rigid_body_set: rapier3d::prelude::RigidBodySet,
  pub collider_set: ColliderSet,

  /* Create other structures necessary for the simulation. */
  //pub gravity: std::vec::Vec<f32>,
  //pub gravity: naglebra::Matrix,
  pub integration_parameters: IntegrationParameters,
  pub physics_pipeline: PhysicsPipeline,
  pub island_manager: IslandManager,
  pub broad_phase: BroadPhase,
  pub narrow_phase: NarrowPhase,
  pub impulse_joint_set: ImpulseJointSet,
  pub multibody_joint_set: MultibodyJointSet,
  pub ccd_solver: CCDSolver,
  pub physics_hooks: (),
  pub event_handler: (),

  pub balls: std::vec::Vec<RigidBodyHandle>,
  pub obj_sphere: Option<FObject>,
  pub shader: Option<glium::Program>,
}

impl<'a> World {
  pub fn new() -> Self {
    Self {
      rigid_body_set: RigidBodySet::new(),
      collider_set: ColliderSet::new(),
      //gravity: vector![0.0, -9.81, 0.0],
      integration_parameters: IntegrationParameters::default(),
      physics_pipeline: PhysicsPipeline::new(),
      island_manager: IslandManager::new(),
      broad_phase: BroadPhase::new(),
      narrow_phase: NarrowPhase::new(),
      impulse_joint_set: ImpulseJointSet::new(),
      multibody_joint_set: MultibodyJointSet::new(),
      ccd_solver: CCDSolver::new(),
      physics_hooks: (),
      event_handler: (),

      balls: std::vec::Vec::new(),
      obj_sphere: None::<FObject>,
      shader: None::<glium::Program>,
    }
    /* Create the ground. */
    /*let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
    w.collider_set.insert(collider);
    /* Create the bounding ball. */
    let mut rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 10.0, 0.0])
            .build();
    let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    let ball_body_handle = w.rigid_body_set.insert(rigid_body);
    w.collider_set.insert_with_parent(collider, ball_body_handle, &mut w.rigid_body_set);*/
  }

  
  pub fn create_ball(&mut self, pos: nalgebra::Vector3<f32>, radius: Real, previous: Option<RigidBodyHandle>, dynamic: bool) -> RigidBodyHandle{

    let rigid_body = match dynamic {
      true => RigidBodyBuilder::dynamic(),
      false => RigidBodyBuilder::kinematic_position_based()
    }.translation( vector![pos.x, pos.y, pos.z] )
      .build();

    let collider = ColliderBuilder::ball(radius).restitution(0.7).mass(10.0).build();
    let ball_body_handle = self.rigid_body_set.insert(rigid_body);
    self.collider_set.insert_with_parent(collider, ball_body_handle, &mut self.rigid_body_set);

    //rigid_body.
    if let Some(prev) = previous {
      let k1 = point![pos.x, pos.y, pos.z];
      let k2 = point![pos.x, pos.y-2.5, pos.z];
      let joint = SphericalJointBuilder::new()
        .local_anchor1(k1)
        .local_anchor2(k2);
        //.local_anchor1(point![0.0, 0.0, -3.0])
        //.local_anchor2(point![0.0, 0.0, 1.0]);
      self.impulse_joint_set.insert(ball_body_handle, prev, joint, true);
    }
    ball_body_handle
  }


  pub fn add_force<'c>(&mut self) {
    for body in self.rigid_body_set.iter_mut() {
      let m:Vector<f32> = body.1.translation().normalize() * 1000.0;
      //body.1.add_force(vector![0.0, 1000.0, 0.0], true);
      //body.1.add_force( -m, true);
      //body.1.set_linvel(-m, true);
      body.1.set_linvel(vector![0.0, 1000.0, 0.0], true);
      //break;
    }
  }

  pub fn init_balls(&mut self, display: &glium::Display) {
    self.shader = Some(f::shader::create_shader_vf( &display, "test" ));

    let mut previous: Option<RigidBodyHandle> = None;
    self.balls = (1..10).map( |n| {
      let pos = nalgebra::Vector3::new( 0.0, n as f32 * 2.5 - 1.0, 0.0 );
      previous = Some(self.create_ball( pos, 0.8, previous, n>1 ));
      previous.unwrap()
    } )
    .collect();
    self.obj_sphere = Some(f::FObject::load_gltf( "data/sphere.gltf", &display ));

    if let Some(obj) = &mut self.obj_sphere {
      obj.set_texture( "texture.jpg", display );
    }

  }

  pub fn render_balls<'b>(&mut self, target: &mut glium::Frame, time: f32, view_mat: [[f32;4];4], perspective_mat: [[f32;4];4]) {
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

    self.physics_pipeline.step(
      &gravity,
      &self.integration_parameters,
      &mut self.island_manager,
      &mut self.broad_phase,
      &mut self.narrow_phase,
      &mut self.rigid_body_set,
      &mut self.collider_set,
      &mut self.impulse_joint_set,
      &mut self.multibody_joint_set,
      &mut self.ccd_solver,
      &self.physics_hooks,
      &self.event_handler,
    );


    if let Some(obj) = &self.obj_sphere {
      //println!("EFKFE");
      if let Some(shader) = &self.shader {
        self.balls.iter().for_each( |b| {
          let ball = &self.rigid_body_set[*b];

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
