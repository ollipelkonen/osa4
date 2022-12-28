
extern crate glium;
use crate::f::FMesh::FMesh;
use crate::f::vertex::Vertex;
use crate::f::shader::load_shader;


pub struct Sdf {
  pub mesh: FMesh,
  pub shader: glium::Program,
}


pub fn create(display: &glium::Display, filename: &str) -> Sdf {
  let vertices: Vec<Vertex> = [
    Vertex::new( [-1.0f32,  1.0, 0.0], [1.0,0.0,0.0], [0.0, 0.0] ), // Top-left
    Vertex::new( [ 1.0f32,  1.0, 0.0], [0.0,1.0,0.0], [1.0, 0.0] ), // Top-right
    Vertex::new( [ 1.0f32, -1.0, 0.0], [0.0,0.0,1.0], [1.0, 1.0] ), // Bottom-right
    Vertex::new( [-1.0f32, -1.0, 0.0], [0.0,0.0,1.0], [0.0, 1.0] )
  ].to_vec();

  let indices = [ 0u32,1,3, 1,3,2];
  let vbuffer = glium::vertex::VertexBuffer::new(display, &vertices).unwrap();
  let ibuffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
  let mesh = FMesh{ vbuffer, ibuffer, material: None, vertices: None, indices: None, edges: None, bounds: None, matrix: nalgebra::Matrix4::<f32>::identity() };

  let vs = load_shader( "sdf-vert.glsl" );
  let fs = load_shader( &filename );
  let shader = glium::Program::from_source(display, &vs, &fs, None).unwrap();

  Sdf {
    mesh, shader
  }
}
