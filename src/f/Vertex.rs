#[derive(Copy, Clone, Debug)]
pub struct Vertex {
  pub position: [f32; 3],
  pub normal: [f32; 3],
  pub tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, normal, tex_coords);

#[allow(dead_code)]
impl Default for Vertex {
  fn default() -> Self {
    Vertex {
      position: [0.0, 0.0, 0.0],
      normal: [0.0, 0.0, 1.0],
      tex_coords: [0.0, 0.0]
    }
  }
}



impl Vertex {
  pub fn new(p: [f32; 3], n: [f32; 3], t: [f32; 2]) -> Vertex {
    Vertex {
      position: [p[0], p[1], p[2]],
      normal: [n[0], n[1], n[2]],
      tex_coords: [t[0], t[1]],
    }
  }
  pub fn zero() -> Vertex {
    Vertex {
      position: [0.0, 0.0, 0.0],
      normal: [0.0, 0.0, 0.0],
      tex_coords: [0.0, 0.0]
    }
  }
  pub fn min() -> Vertex {
    Vertex {
      position: [f32::MIN, f32::MIN, f32::MIN],
      normal: [f32::MIN, f32::MIN, f32::MIN],
      tex_coords: [f32::MIN, f32::MIN]
    }
  }
  pub fn max() -> Vertex {
    Vertex {
      position: [f32::MAX, f32::MAX, f32::MAX],
      normal: [f32::MAX, f32::MAX, f32::MAX],
      tex_coords: [f32::MAX, f32::MAX]
    }
  }
}
