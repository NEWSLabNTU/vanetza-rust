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
        {}

        double as_square_meters() {
            return m_inner / boost::units::si::square_meter;
        }

    private:
        Area m_inner;
    };
}
}
