mod its_aid;
mod manual_runtime;
mod runtime;
mod station_type;

pub use its_aid::*;
pub use manual_runtime::*;
pub use runtime::*;
pub use station_type::*;

pub mod aid {
    use super::ItsAid;

    pub const CA: ItsAid = 36;
    pub const DEN: ItsAid = 37;
    pub const TLM: ItsAid = 137;
    pub const RLT: ItsAid = 138;
    pub const IVI: ItsAid = 139;
    pub const TLC_R: ItsAid = 140;
    pub const TLC_S: ItsAid = 637;
    pub const GN_MGMT: ItsAid = 141;
    pub const CRL: ItsAid = 622;
    pub const SCR: ItsAid = 623;
    pub const CTL: ItsAid = 624;
    pub const VRU: ItsAid = 638;
    pub const CP: ItsAid = 639;
    pub const IMZ: ItsAid = 640;
    pub const SA: ItsAid = 540801;
    pub const GPC: ItsAid = 540802;
    pub const IPV6_ROUTING: ItsAid = 270549118;
}
