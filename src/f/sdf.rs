
use crate::f;
extern crate glium;


pub struct Sdf {
  pub mesh: f::FMesh,
  pub shader: glium::Program,
}


pub fn create(display: &glium::Display, filename: &str) -> Sdf {
  let vertices: Vec<f::Vertex> = [
    f::Vertex::new( [-1.0f32,  1.0, 0.0], [1.0,0.0,0.0], [0.0, 0.0] ), // Top-left
    f::Vertex::new( [ 1.0f32,  1.0, 0.0], [0.0,1.0,0.0], [1.0, 0.0] ), // Top-right
    f::Vertex::new( [ 1.0f32, -1.0, 0.0], [0.0,0.0,1.0], [1.0, 1.0] ), // Bottom-right
    f::Vertex::new( [-1.0f32, -1.0, 0.0], [0.0,0.0,1.0], [0.0, 1.0] )
  ].to_vec();

  let indices = [ 0u32,1,3, 1,3,2];
  let vbuffer = glium::vertex::VertexBuffer::new(display, &vertices).unwrap();
  let ibuffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
  let mesh = f::FMesh{ vbuffer, ibuffer, material: None, vertices: None, indices: None, edges: None, bounds: None, matrix: nalgebra::Matrix4::<f32>::identity() };

  let vs = f::shader::load_shader( "sdf-vert.glsl" );
  let fs = f::shader::load_shader( &filename );
  let shader = glium::Program::from_source(display, &vs, &fs, None).unwrap();

  Sdf {
    mesh, shader
  }
}
