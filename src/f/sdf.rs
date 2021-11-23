
use std::{fs, io};
use crate::f;


pub struct Sdf {
  pub mesh: f::FMesh,
}




pub fn create(display: &glium::Display) -> f::FMesh {
    let vertices: Vec<f::Vertex> = vec! [
      f::Vertex::new( [-1.0f32,  1.0, 0.0], [0.0,0.0,0.0], [0.0, 0.0] ), // Top-left
      f::Vertex::new( [ 1.0f32,  1.0, 0.0], [0.0,0.0,0.0], [1.0, 0.0] ), // Top-right
      f::Vertex::new( [ 1.0f32, -1.0, 0.0], [0.0,0.0,0.0], [1.0, 1.0] ), // Bottom-right
      f::Vertex::new( [-1.0f32, -1.0, 0.0], [0.0,0.0,0.0], [0.0, 1.0] )
    ];
  let indices: Vec<u32> = vec![ 0u32, 1, 3,  ];
  let vbuffer = glium::vertex::VertexBuffer::new(display, &vertices).unwrap();
  let ibuffer = glium::index::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

  f::FMesh{ vbuffer, ibuffer, material: None, vertices: None, indices: None }
}
