use std::collections::HashMap;
use super::{Chunk, ChunkIndex, HexPillar};
use math::*;

/// Represents a whole game world consisting of multiple `Chunk`s.
///
/// Chunks are parallelograms (roughly) that are placed next to each other
/// in the world.
pub struct World {
    // TODO: make it private after we can access it immutable via a method! (see #7)
    pub chunks: HashMap<ChunkIndex, Chunk>,
}

impl World {
    /// Creates an empty world without any chunks.
    pub fn empty() -> Self {
        World { chunks: HashMap::new() }
    }

    /// Returns a dummy world with one dummy chunk for early testing.
    /// FIXME: remove
    pub fn dummy() -> Self {
        let mut chunks = HashMap::new();
        chunks.insert(ChunkIndex(AxialPoint::new(0, 0)), Chunk::dummy());
        World { chunks: chunks }
    }

    /// Returns the hex pillar at the given world position, iff the
    /// corresponding chunk is loaded.
    pub fn pillar_at(&self, pos: ChunkIndex) -> Option<&HexPillar> {
        // TODO: use `/` operator once it's implemented
        // let chunk_pos = pos / (super::CHUNK_SIZE as i32);
        let chunk_pos = AxialPoint::new(pos.0.q / (super::CHUNK_SIZE as i32),
                                        pos.0.r / (super::CHUNK_SIZE as i32));

        let out = self.chunks.get(&ChunkIndex(chunk_pos)).map(|chunk| {
            // TODO: use `%` operator once it's implemented
            // let inner_pos = pos % (super::CHUNK_SIZE as i32);
            let inner_pos = AxialPoint::new(pos.0.q % (super::CHUNK_SIZE as i32),
                                            pos.0.r % (super::CHUNK_SIZE as i32));
            &chunk[inner_pos]
        });

        if out.is_none() {
            debug!("chunk {:?} is not loaded (position request {:?})",
                   chunk_pos,
                   pos);
        }

        out
    }
}
