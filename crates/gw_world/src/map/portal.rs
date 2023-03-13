use bitflags::bitflags;
use gw_util::fl;
use std::fmt;

bitflags! {
    #[derive(Default)]
    pub struct PortalFlags: u32 {
        const ON_CLIMB = fl!(0);
        const ON_DESCEND = fl!(1);
        const ON_ENTER = fl!(2);
    }
}

impl fmt::Display for PortalFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct PortalInfo {
    pub(crate) flavor: Option<String>,
    pub(crate) flags: PortalFlags,
    pub(crate) map_id: String,
    pub(crate) map_location: String,
}

impl PortalInfo {
    pub fn new(map_id: &str, location: &str) -> Self {
        PortalInfo {
            flavor: None,
            flags: PortalFlags::ON_DESCEND,
            map_id: map_id.to_string(),
            map_location: location.to_string(),
        }
    }

    pub fn flavor(&self) -> &Option<String> {
        &self.flavor
    }

    pub fn set_flavor(&mut self, flavor: &str) {
        self.flavor = Some(flavor.to_string());
    }

    pub fn flags(&self) -> &PortalFlags {
        &self.flags
    }

    pub fn set_flags(&mut self, flag: PortalFlags) {
        self.flags = flag;
    }

    pub fn map_id(&self) -> &str {
        &self.map_id
    }

    pub fn location(&self) -> &str {
        &self.map_location
    }
}
