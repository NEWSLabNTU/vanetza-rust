#pragma once

#include <boost/units/systems/si/length.hpp>
#include <vanetza/units/length.hpp>


namespace vanetza_wrapper
{
namespace units
{
    using vanetza::units::Length;

    class LengthWrapper {
    public:
        LengthWrapper(Length src) :
            m_inner(src)
        {}

        double as_meters() {
            return m_inner / boost::units::si::meter;
        }

    private:
        Length m_inner;
    };
}
}
