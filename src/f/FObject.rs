extern crate glium;
use nalgebra::Matrix4;
use crate::f::FMesh::FMesh;
use crate::f::*;
use crate::f::FMaterial::FMaterial;


#[derive(Debug)]
pub struct FObject {
  pub matrix: Matrix4<f32>,
  pub meshes: Vec<FMesh>,
  pub materials: Vec<FMaterial>,
  pub textures: Vec<glium::texture::SrgbTexture2d>,
}

impl FObject {
    pub fn print_bounds(self) {
      for m in self.meshes {
        m.print_bounds();
      }
    }
  
    // set same texture for every object
  //  pub fn set_texture(&mut self, texture: glium::texture::SrgbTexture2d) {
    pub fn set_texture(&mut self, texture: &str, display: &glium::Display) {
  
      if let Some(image) = load_image( texture, &display) {
        self.textures = vec![image];
        self.materials = vec![
          FMaterial {
            diffuse_texture: Some(0),
            normal_texture:  None,
            occlusion_texture:  None,
            metallic_roughness_texture:  None
          }
        ];
        for mesh in self.meshes.iter_mut() {
          mesh.material = Some(0);
        }
      }
  
    }
  
    pub fn load_gltf( filename: &str, display: &glium::Display ) -> FObject {
      let (doc, buffers, images) = match gltf::import(filename) {
        Ok(tuple) => tuple,
        Err(err) => {
          println!("glTF import failed: {:?}", err);
          if let gltf::Error::Io(_) = err {
            println!("Hint: Are the .bin file(s) referenced by the .gltf file available?")
          }
          std::process::exit(1)
        },
      };
  
      let imp = ImportData { doc, buffers, images };
  
      let textures: Vec<glium::texture::SrgbTexture2d> = imp.doc.textures().map( |im| {
            load_image_from_source(im.source().source(), display)
        })
        .filter_map(|a| a)
        .collect();
  
  
      let meshes: Vec<FMesh> = imp.doc.nodes()
        .filter_map(|n| n.mesh() )
        .map( |node| {
          mesh_from_gltf(&node.primitives().next().unwrap(), &imp, display)
        })
        .collect::<Vec<_>>();
  
  
      let materials = imp.doc.materials()
        .map( |material| {
          let diffuse_texture = match material.pbr_metallic_roughness().base_color_texture() {
            Some(d) => Some(d.texture().index()),
            _ => None
          };
          let normal_texture = match material.normal_texture() {
            Some(d) => Some(d.texture().index()),
            _ => None
          };
          let occlusion_texture = match material.occlusion_texture() {
            Some(d) => Some(d.texture().index()),
            _ => None
          };
          let metallic_roughness_texture = None;
          FMaterial { diffuse_texture, normal_texture, occlusion_texture, metallic_roughness_texture }
        })
        .collect();
  
      FObject { matrix: Matrix4::<f32>::identity(), meshes, materials, textures }
    }
  
    pub fn print_tree(&self, node: &gltf::Node, depth: i32) {
      for _ in 0..(depth - 1) {
        print!("  ");
      }
      print!(" -");
      print!(" Node {}", node.index());
      #[cfg(feature = "names")]
      print!(" ({})", node.name().unwrap_or("<Unnamed>"));
      if let Some(mesh) = node.mesh() {
        print!(" mesh {:?}", mesh.primitives());
      }
      if let Some(_cam) = node.camera() {
        print!(" camera ");
      }
      if let Some(_skin) = node.skin() {
        print!(" skin ");
      }
      println!();
      for child in node.children() {
        self.print_tree(&child, depth + 1);
      }
    }
  
  }
  