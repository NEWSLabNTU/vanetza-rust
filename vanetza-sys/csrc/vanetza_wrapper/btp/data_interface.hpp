#pragma once

#include <vanetza/btp/data_indication.hpp>
#include <vanetza/btp/data_request.hpp>
#include <vanetza/common/byte_order.hpp>
#include <vanetza/net/packet.hpp>
#include <memory>
#include <vanetza_wrapper/btp/data_indication.hpp>
namespace vanetza_wrapper
{
namespace btp
{
    using vanetza_wrapper::btp::BtpDataIndicationWrapper;
    using vanetza::UpPacket;
    using vanetza::btp::DataRequestB;
    using vanetza::DownPacket;
class BTPIndicationInterfaceWrapper
{
public:
    virtual void indicate(const BtpDataIndicationWrapper&, std::unique_ptr<UpPacket>) = 0;
    virtual ~BTPIndicationInterfaceWrapper() {};
};

class BTPRequestInterfaceWrapper
{
public:
    virtual void request(const DataRequestB&, std::unique_ptr<DownPacket>) = 0;
    virtual ~BTPRequestInterfaceWrapper() {};
};

} // namespace btp
} // namespace vanetza


