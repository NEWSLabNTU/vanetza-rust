#pragma once

#include <memory>
#include <vanetza/net/packet_variant.hpp>
#include <vanetza/net/chunk_packet.hpp>
#include <vanetza/net/cohesive_packet.hpp>

namespace vanetza_wrapper
{
    using vanetza::PacketVariant;
    using vanetza::ChunkPacket;
    using vanetza::CohesivePacket;

    class PacketVariantWrapper {
    public:
        // PacketVariantWrapper(PacketVariant&& ref) :
        //     m_inner(ref)
        // {}

        // PacketVariantWrapper(ChunkPacket&& ref) :
        //     m_inner(ref)
        // {}

        // PacketVariantWrapper(CohesivePacket&& ref) :
        //     m_inner(ref)
        // {}

        std::unique_ptr<PacketVariant> m_inner;
    };
}
