
extern crate glium;
//use nalgebra::Matrix4;
use rapier3d::prelude::*;
use rand::Rng;


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
    pub event_handler: ()
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
  
    
  
  
    pub fn add_force<'c>(&mut self) {
      let mut rng = rand::thread_rng();

      for body in self.rigid_body_set.iter_mut() {
        //body.1.add_force(vector![0.0, 1000.0, 0.0], true);
        /*let m:Vector<f32> = body.1.translation().normalize() * 1000.0;
        body.1.add_force( -m, true);*/
        //body.1.set_linvel(-m, true);
        //body.1.set_linvel(vector![0.2, 0.0, 0.0], true);
        let v = vector![rng.gen_range(0.0..1.0) as Real,
          rng.gen_range(0.0..1.0) as Real,
          rng.gen_range(0.0..1.0) as Real
          ];
        body.1.set_angvel( v, true );
        //break;
      }
    }
  
  }
  