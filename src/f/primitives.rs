

#[derive(Debug)]
pub struct FMaterial {
  pub diffuse_texture: Option<usize>,
  pub normal_texture: Option<usize>,
  pub occlusion_texture: Option<usize>,
  pub metallic_roughness_texture: Option<usize>  // unused
}

#[derive(Debug)]
pub struct FEdge
{
  pub i: [u32;2],
}
