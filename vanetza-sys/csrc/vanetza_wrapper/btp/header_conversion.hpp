#pragma once


#include <vanetza/btp/header.hpp>
#include <vanetza/common/byte_buffer_convertible.hpp>
#include <vanetza/common/serialization_buffer.hpp>

namespace vanetza_wrapper
{
namespace convertible
{
    using vanetza::convertible::byte_buffer;
    using vanetza::ByteBuffer;
    using vanetza::btp::HeaderA;
    using vanetza::btp::HeaderB;
    using vanetza::serialize_into_buffer;
struct BTPHeaderAbyte_buffer_implWrapper : public byte_buffer
{
    BTPHeaderAbyte_buffer_implWrapper(const HeaderA& header) : m_header(header) {}
    void convert(ByteBuffer& buffer) const override
    {
        buffer.clear();
        serialize_into_buffer(m_header, buffer);
    }
    std::size_t size() const override { return HeaderA::length_bytes; }

    const HeaderA m_header;
};

struct BTPHeaderBbyte_buffer_implWrapper : public byte_buffer
{
    BTPHeaderBbyte_buffer_implWrapper(const HeaderB& header) : m_header(header) {}
    void convert(ByteBuffer& buffer) const override
    {
        buffer.clear();
        serialize_into_buffer(m_header, buffer);
    }
    std::size_t size() const override { return HeaderB::length_bytes; }

    const HeaderB m_header;
};

} // namespace convertible
} // namespace vanetza

