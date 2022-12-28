
extern crate glium;
//use nalgebra::Matrix4;
use rapier3d::prelude::*;
use rand::Rng;


#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
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
    }

    pub fn create_ground(&mut self) {
      /* Create the ground. */
      let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0)
        .translation(vector![0.0, -30.0, 0.0])
        .build();
      self.collider_set.insert(collider);
      /* Create the bounding ball. */
      /*let mut rigid_body = RigidBodyBuilder::fixed()
              .translation(vector![0.0, -20.0, 0.0])
              .build();
      let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
      let ball_body_handle = self.rigid_body_set.insert(rigid_body);
      self.collider_set.insert_with_parent(collider, ball_body_handle, &mut self.rigid_body_set);*/
    }
  
    
  

  
    pub fn add_force<'c>(&mut self) {
      let mut rng = rand::thread_rng();

      //let i = rng.gen_range(0..self.rigid_body_set.len());
      let i = 0;
      if let Some(x) = self.rigid_body_set.iter_mut().nth(i) {

        x.1.reset_forces(true);
        x.1.reset_torques(true);

        //x.1.add_force(vector![0.0, 2.5, 0.0], true);
        let v = vector![
          rng.gen_range(0.0..0.1) as Real,
          rng.gen_range(0.0..0.1) as Real,
          rng.gen_range(0.0..0.1) as Real
          ];
        x.1.add_torque(vector![1.0, 0.0, 0.0], true);
        /*
        x.1.add_force_at_point(vector![0.0, 1.0, 0.0], point![1.0, 2.0, 3.0], true);
*/
        //x.1.apply_impulse(vector![0.0, 0.5, 0.0], true);
/*        x.1.apply_torque_impulse(vector![1.0, 0.0, 0.0], true);
        x.1.apply_impulse_at_point(vector![0.0, 1.0, 0.0], point![1.0, 2.0, 3.0], true);
*/
        //x.1.set_angvel( v, true );

      }
      return;
      

      for body in self.rigid_body_set.iter_mut() {
        //body.1.add_force(vector![0.0, 1000.0, 0.0], true);
        /*let m:Vector<f32> = body.1.translation().normalize() * 1000.0;
        body.1.add_force( -m, true);*/
        //body.1.set_linvel(-m, true);
        //body.1.set_linvel(vector![0.2, 0.0, 0.0], true);
        let v = vector![rng.gen_range(-1.0..1.0) as Real,
          rng.gen_range(-1.0..1.0) as Real,
          rng.gen_range(-1.0..1.0) as Real
          ];
        body.1.set_angvel( v, true );
        //break;
      }
    }
  
  }
  