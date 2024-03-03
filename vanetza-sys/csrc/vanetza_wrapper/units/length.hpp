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

        LengthWrapper(double meters) :
            m_inner(meters * boost::units::si::meter)
        {}

        double as_meters() const {
            return m_inner / boost::units::si::meter;
        }

        Length m_inner;
    };
}
}
