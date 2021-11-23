
use crate::f;


pub struct Sdf {
  pub mesh: f::FMesh,
  pub shader: glium::Program,
}


pub fn create(display: &glium::Display, filename: &str) -> Sdf {
  let vertices: Vec<f::Vertex> = vec! [
    f::Vertex::new( [-1.0f32,  1.0, 0.0], [0.0,0.0,0.0], [0.0, 0.0] ), // Top-left
    f::Vertex::new( [ 1.0f32,  1.0, 0.0], [0.0,0.0,0.0], [1.0, 0.0] ), // Top-right
    f::Vertex::new( [ 1.0f32, -1.0, 0.0], [0.0,0.0,0.0], [1.0, 1.0] ), // Bottom-right
    f::Vertex::new( [-1.0f32, -1.0, 0.0], [0.0,0.0,0.0], [0.0, 1.0] )
  ];
  let indices: Vec<u32> = vec![ 0u32,1,3, 0,1,2, 1,2,3, 3,2,1, 2,1,3, 2,3,1, 0,3,2, 0,2,1 ];
  let vbuffer = glium::vertex::VertexBuffer::new(display, &vertices).unwrap();
  let ibuffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

  let fs = f::Shader::load_shader( filename );
  let vs = f::Shader::load_shader( "sdf-vert.glsl" );

  Sdf {
    mesh: f::FMesh{ vbuffer, ibuffer, material: None, vertices: None, indices: None },
    shader: glium::Program::from_source(display, &vs, &fs, None).unwrap()
  }
}
