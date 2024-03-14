#pragma once

#include <boost/units/systems/si/area.hpp>
#include <vanetza/units/area.hpp>


namespace vanetza_wrapper
{
namespace units
{
    using vanetza::units::Area;

    class AreaWrapper {
    public:
        AreaWrapper(Area src) :
            m_inner(src)
        {
        }

        AreaWrapper(double square_meters) :
            m_inner(square_meters * boost::units::si::square_meter)
        {}

        double as_square_meters() const {
            return m_inner / boost::units::si::square_meter;
        }

        Area m_inner;
    };
}
}
