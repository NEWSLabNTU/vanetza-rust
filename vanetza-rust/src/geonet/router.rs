use super::{
    location_table::LocationTableRef, mib::MIB, Address, DataConfirm, DataRequestVariant,
    GacDataRequest, GbcDataRequest, GucDataRequest, LongPositionVectorRef, ShbDataRequest,
    TsbDataRequest,
};
use crate::{utils::MacAddressExt, DownPacket, PositionFix, Runtime, UpPacket};
use autocxx::prelude::*;
use cxx::UniquePtr;
use mac_address::MacAddress;
use std::os;
use vanetza_sys::vanetza::geonet::Router as CxxRouter;

pub struct Router {
    ptr: UniquePtr<CxxRouter>,
}

impl Router {
    pub fn new(runtime: &impl Runtime, mib: &MIB) -> Self {
        let ptr = CxxRouter::new(runtime.as_cxx_pin_mut(), &mib.to_cxx_ptr()).within_unique_ptr();
        Self { ptr }
    }

    pub fn request(&mut self, request: &DataRequestVariant, packet: DownPacket) -> DataConfirm {
        match request {
            DataRequestVariant::Guc(req) => self.request_guc(req, packet),
            DataRequestVariant::Gbc(req) => self.request_gbc(req, packet),
            DataRequestVariant::Gac(req) => self.request_gac(req, packet),
            DataRequestVariant::Shb(req) => self.request_shb(req, packet),
            DataRequestVariant::Tsb(req) => self.request_tsb(req, packet),
        }
    }

    pub fn request_gbc(&mut self, request: &GbcDataRequest, packet: DownPacket) -> DataConfirm {
        self.ptr
            .pin_mut()
            .request1(request.ptr.as_ref().unwrap(), packet.ptr)
    }

    pub fn request_guc(&mut self, request: &GucDataRequest, packet: DownPacket) -> DataConfirm {
        self.ptr
            .pin_mut()
            .request2(request.ptr.as_ref().unwrap(), packet.ptr)
    }

    pub fn request_gac(&mut self, request: &GacDataRequest, packet: DownPacket) -> DataConfirm {
        self.ptr
            .pin_mut()
            .request3(request.ptr.as_ref().unwrap(), packet.ptr)
    }

    pub fn request_tsb(&mut self, request: &TsbDataRequest, packet: DownPacket) -> DataConfirm {
        self.ptr
            .pin_mut()
            .request4(request.ptr.as_ref().unwrap(), packet.ptr)
    }

    pub fn request_shb(&mut self, request: &ShbDataRequest, packet: DownPacket) -> DataConfirm {
        self.ptr
            .pin_mut()
            .request(request.ptr.as_ref().unwrap(), packet.ptr)
    }

    pub fn indicate(&mut self, packet: UpPacket, sender: MacAddress, destination: MacAddress) {
        // self.ptr.pin_mut().indicate(packet, &sender.to_cxx(), &destination.to_cxx())
        todo!();
    }

    pub fn update_position(&mut self, fix: &PositionFix) {
        self.ptr.pin_mut().update_position(&fix.to_cxx_ptr());
    }

    pub fn set_address(&mut self, address: &Address) {
        self.ptr.pin_mut().set_address(&address.to_cxx_ptr());
    }

    pub fn mib(&self) -> MIB {
        let mib = self.ptr.as_ref().unwrap().get_mib();
        MIB::from_cxx(mib)
    }

    pub fn location_table(&self) -> LocationTableRef<'_> {
        let ref_ = self.ptr.as_ref().unwrap().get_location_table();
        LocationTableRef { ref_ }
    }

    pub fn local_position_vector(&self) -> LongPositionVectorRef<'_> {
        let ref_ = self.ptr.as_ref().unwrap().get_local_position_vector();
        LongPositionVectorRef { ref_ }
    }

    pub fn outside_sectorial_contention_area(
        &self,
        sender: MacAddress,
        forwarder: MacAddress,
    ) -> bool {
        self.ptr
            .as_ref()
            .unwrap()
            .outside_sectorial_contention_area(&sender.to_cxx(), &forwarder.to_cxx())
    }

    pub fn set_random_seed(&mut self, seed: u32) {
        self.ptr
            .pin_mut()
            .set_random_seed(autocxx::c_ulong(seed as os::raw::c_ulong));
    }
}
