extern crate glium;
use nalgebra::Matrix4;

////#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
#[derive(Debug)]
pub struct FMesh {
  pub matrix: Matrix4<f32>,
  pub vbuffer: glium::vertex::VertexBuffer<Vertex>,
  pub ibuffer: glium::index::IndexBuffer<u32>,
  pub material: Option<usize>,
  pub vertices: Option<Vec<Vertex>>,
  pub indices: Option<Vec<u32>>,
  pub edges: Option<Vec<FEdge>>,
  pub bounds: Option<[Vertex; 2]>
}

//TODO:
/*#[allow(dead_code)]
impl Default for FMesh {
  fn default() -> Self {
    FMesh {
      matrix. nalgebra::Matrix4::<f32>::identity(),
      vbuffer: glium::vertex::VertexBuffer::<Vertex>::empty(display, 0),
      ibuffer: None,
      material: None,
      vertices: None,
      indices: None,
      edges: None,
      bounds: None
    }
  }
}*/


impl FMesh {
  pub fn print_bounds(self) {
    println!("bounds.  center [{:?}, {:?}, {:?}]   size: [{:?}, {:?}, {:?}] ",
      (self.bounds.unwrap()[0].position[0]+self.bounds.unwrap()[1].position[0])/2.0,
      (self.bounds.unwrap()[0].position[1]+self.bounds.unwrap()[1].position[1])/2.0,
      (self.bounds.unwrap()[0].position[2]+self.bounds.unwrap()[1].position[2])/2.0,
      (self.bounds.unwrap()[1].position[0]-self.bounds.unwrap()[0].position[0]),
      (self.bounds.unwrap()[1].position[1]-self.bounds.unwrap()[0].position[1]),
      (self.bounds.unwrap()[1].position[2]-self.bounds.unwrap()[0].position[2])
    );
  }

}
