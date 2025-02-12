use dotrix_core::assets::{Mesh, Texture};
use dotrix_core::Id;

/// Widget
#[derive(Default)]
pub struct Widget {
    /// Widget mesh
    pub mesh: Mesh,
    /// Id of the widget texture
    pub texture: Id<Texture>,
}
