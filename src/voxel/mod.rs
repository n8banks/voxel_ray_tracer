use glam::{IVec3, U8Vec3};

/// Data associated with a single voxel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Voxel {
    pub color: U8Vec3,
}

/// An iterator that produces voxels.
#[derive(Clone)]
pub struct VoxelGenerator {}

impl VoxelGenerator {
    /// Create a new voxel generator.
    pub fn new() -> Self {
        todo!()
    }

    /// Lookup a voxel value at some position.
    pub fn lookup(&self, pos: IVec3) -> Option<Voxel> {
        todo!()
    }
}
