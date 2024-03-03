use super::{
    DataRequest, GacDataRequest, GbcDataRequest, GucDataRequest, ShbDataRequest, TsbDataRequest,
};
use vanetza_sys::vanetza::geonet::DataRequest as CxxDataRequest;

pub enum DataRequestVariant {
    Guc(GucDataRequest),
    Gbc(GbcDataRequest),
    Gac(GacDataRequest),
    Shb(ShbDataRequest),
    Tsb(TsbDataRequest),
}

impl From<TsbDataRequest> for DataRequestVariant {
    fn from(v: TsbDataRequest) -> Self {
        Self::Tsb(v)
    }
}

impl From<ShbDataRequest> for DataRequestVariant {
    fn from(v: ShbDataRequest) -> Self {
        Self::Shb(v)
    }
}

impl From<GacDataRequest> for DataRequestVariant {
    fn from(v: GacDataRequest) -> Self {
        Self::Gac(v)
    }
}

impl From<GbcDataRequest> for DataRequestVariant {
    fn from(v: GbcDataRequest) -> Self {
        Self::Gbc(v)
    }
}

impl From<GucDataRequest> for DataRequestVariant {
    fn from(v: GucDataRequest) -> Self {
        Self::Guc(v)
    }
}

impl DataRequest for DataRequestVariant {
    fn as_cxx_ref(&self) -> &CxxDataRequest {
        match self {
            DataRequestVariant::Guc(req) => <_ as DataRequest>::as_cxx_ref(req),
            DataRequestVariant::Gbc(req) => <_ as DataRequest>::as_cxx_ref(req),
            DataRequestVariant::Gac(req) => <_ as DataRequest>::as_cxx_ref(req),
            DataRequestVariant::Shb(req) => <_ as DataRequest>::as_cxx_ref(req),
            DataRequestVariant::Tsb(req) => <_ as DataRequest>::as_cxx_ref(req),
        }
    }
}
