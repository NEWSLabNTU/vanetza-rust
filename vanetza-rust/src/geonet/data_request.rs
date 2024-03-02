use super::mib::MIB;
use crate::common::ItsAid;
use vanetza_sys as sys;

// traits

pub trait DataRequest {
    fn new(mib: &MIB, its_aid: ItsAid) -> Self;
}

pub trait DataRequestWithArea: DataRequest {}

pub trait DataRequestWithAddress: DataRequest {}

macro_rules! declare_data_request {
    ($r_name:ident, $c_name:ident, $new_fn:ident, $del_fn:ident) => {
        pub struct $r_name(pub(crate) sys::$c_name);

        impl DataRequest for $r_name {
            fn new(mib: &MIB, its_aid: ItsAid) -> Self {
                unsafe {
                    let ptr = sys::$new_fn(mib.0, its_aid);
                    Self(ptr)
                }
            }
        }

        impl Drop for $r_name {
            fn drop(&mut self) {
                unsafe {
                    sys::$del_fn(self.0);
                }
            }
        }
    };
}

// GucDataRequest
declare_data_request!(
    GucDataRequest,
    c_guc_data_request,
    guc_data_request_new,
    guc_data_request_del
);

impl DataRequestWithAddress for GucDataRequest {}

// GbcDataRequest
declare_data_request!(
    GbcDataRequest,
    c_gbc_data_request,
    gbc_data_request_new,
    gbc_data_request_del
);

impl DataRequestWithArea for GbcDataRequest {}

// GacDataRequest
declare_data_request!(
    GacDataRequest,
    c_gac_data_request,
    gac_data_request_new,
    gac_data_request_del
);

impl DataRequestWithArea for GacDataRequest {}

// ShbDataRequest
declare_data_request!(
    ShbDataRequest,
    c_shb_data_request,
    shb_data_request_new,
    shb_data_request_del
);

// TsbDataRequest
declare_data_request!(
    TsbDataRequest,
    c_tsb_data_request,
    tsb_data_request_new,
    tsb_data_request_del
);
