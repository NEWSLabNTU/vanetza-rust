#pragma once


#include <vanetza/common/byte_order.hpp>
#include <vanetza/common/serialization.hpp>
#include <vanetza/net/packet_variant.hpp>
#include <cstdint>
#include <vanetza/btp/header.hpp>
namespace vanetza_wrapper
{
namespace btp
{
    using vanetza::uint16be_t;
    using std::size_t;
    using vanetza::OutputArchive;
    using vanetza::InputArchive;
    using vanetza::ChunkPacket;
    using vanetza::CohesivePacket;
    using vanetza::PacketVariant;
    using vanetza::btp::HeaderB;

    typedef uint16be_t port_type;

// interactive packet transport
// struct HeaderA
// {
//     static constexpr std::size_t length_bytes = 4;

//     port_type destination_port;
//     port_type source_port;
// };

// static_assert(sizeof(HeaderA) == HeaderA::length_bytes, "Wrong size");

// void serialize(OutputArchive&, const HeaderA&);
// void deserialize(InputArchive&, HeaderA&);
// HeaderA parse_btp_a(ChunkPacket&);
// HeaderA parse_btp_a(CohesivePacket&);
// HeaderA parse_btp_a(PacketVariant&);

// non-interactive packet transport
class HeaderB_wrapper
{   

    public:
        vanetza::btp::HeaderB headerB;
        uint16_t destination_port_16t;
        uint16_t destination_port_info_16t;
        HeaderB_wrapper(uint16_t destination_port_16t,uint16_t destination_port_info_16t) {
            this->destination_port_16t = destination_port_16t;
            this->destination_port_info_16t = destination_port_info_16t;
            this->headerB.destination_port = vanetza::host_cast<uint16_t>(this->destination_port_16t);
            this->headerB.destination_port_info = vanetza::host_cast<uint16_t>(this->destination_port_info_16t);
        }
        vanetza::btp::HeaderB GetHeaderB() {
            return this->headerB;
        }
};

// struct HeaderB_wrapper
// {   
//     uint16_t destination_port_16t;
//     uint16_t destination_port_info_16t;
// };

// static_assert(sizeof(HeaderB) == HeaderB::length_bytes, "Wrong size");

// void serialize(OutputArchive&, const HeaderB&);
// void deserialize(InputArchive&, HeaderB&);
// HeaderB parse_btp_b(ChunkPacket&);
// HeaderB parse_btp_b(CohesivePacket&);
// HeaderB parse_btp_b(PacketVariant&);

} // namepsace btp
} // namespace vanetza

