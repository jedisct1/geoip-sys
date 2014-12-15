
extern crate libc;

use libc::{c_void, c_char, c_int, c_ulong, c_float};

pub type RawGeoIp = *const c_void;
pub type In6Addr = [u8, ..16];

#[repr(C)]
pub struct GeoIpLookup {
    pub netmask: c_int
}

impl Copy for GeoIpLookup { }

impl GeoIpLookup {
    pub fn new() -> GeoIpLookup {
        GeoIpLookup {
            netmask: 0
        }
    }
}

#[link(name = "GeoIP")]
extern {
    pub fn GeoIP_open(dbtype: *const c_char, flags: c_int) -> RawGeoIp;
    pub fn GeoIP_delete(db: RawGeoIp);
    pub fn GeoIP_name_by_ipnum_gl(db: RawGeoIp, ipnum: c_ulong, gl: *mut GeoIpLookup) -> *const c_char;
    pub fn GeoIP_name_by_ipnum_v6_gl(db: RawGeoIp, ipnum: In6Addr, gl: *mut GeoIpLookup) -> *const c_char;
    pub fn GeoIP_record_by_ipnum(db: RawGeoIp, ipnum: c_ulong) -> *const GeoIpRecord;
    pub fn GeoIP_record_by_ipnum_v6(db: RawGeoIp, ipnum: In6Addr) -> *const GeoIpRecord;
    pub fn GeoIPRecord_delete(gir: *const GeoIpRecord);
    pub fn GeoIP_set_charset(db: RawGeoIp, charset: c_int) -> c_int;
}

#[repr(C)]
pub struct GeoIpRecord {
    pub country_code: *const c_char,
    pub country_code3: *const c_char,
    pub country_name: *const c_char,
    pub region: *const c_char,
    pub city: *const c_char,
    pub postal_code: *const c_char,
    pub latitude: c_float,
    pub longitude: c_float,
    pub dma_code: c_int,
    pub area_code: c_int,
    pub charset: c_int,
    pub continent_code: *const c_char,
    pub netmask: c_int
}

impl Copy for GeoIpRecord { }
