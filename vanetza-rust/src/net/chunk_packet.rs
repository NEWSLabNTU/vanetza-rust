use cxx::UniquePtr;
use vanetza_sys::vanetza::ChunkPacket as CxxChunkPacket;

pub struct ChunkPacket {
    pub(crate) ptr: UniquePtr<CxxChunkPacket>,
}
