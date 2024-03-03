use crate::{utils::IntoCxxUniquePtr, ChunkPacket, CohesivePacket};
use cxx::UniquePtr;
use vanetza_sys::vanetza_wrapper::PacketVariantWrapper as CxxPacketVariantWrapper;

pub enum PacketVariant {
    Chunk(ChunkPacket),
    Cohesive(CohesivePacket),
}

impl IntoCxxUniquePtr<CxxPacketVariantWrapper> for PacketVariant {
    fn into_cxx_unique_ptr(self) -> UniquePtr<CxxPacketVariantWrapper> {
        todo!()
    }
}
