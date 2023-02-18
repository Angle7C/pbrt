
use super::{tool::interaction::Interaction, bxdf::TransportMode};
mod matte;
#[derive(Debug,Clone, Copy)]
pub enum Material{
   
}
pub trait MaterialAble {
    fn bump();
    fn compute_scattering(inter:&mut Interaction,mode:TransportMode,allow_lobe:bool);
}
