use crate::fl;
use bitflags::bitflags;
use std::convert::From;
use std::fmt;
use std::str::FromStr;

bitflags! {
    #[derive(Default)]
    pub struct PortalFlags: u32 {

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

        const ON_CLIMB = fl!(0);
        const ON_DESCEND = fl!(1);
        const ON_ENTER = fl!(2);

        // TODO

        // !!!!!!!!!!!!!!!!!!!!!
        // NOTE - If you add anything, you must add to FromStr impl below!!!!
        // !!!!!!!!!!!!!!!!!!!!!

    }
}

impl PortalFlags {
    pub fn apply(&mut self, flags: &str) {
        for val in flags.split("|") {
            if val.trim().starts_with("!") {
                match Self::from_str(&val[1..]) {
                    Ok(flag) => self.remove(flag),
                    Err(_) => {}
                }
            } else {
                match Self::from_str(val) {
                    Ok(flag) => self.insert(flag),
                    Err(_) => {}
                }
            }
        }
    }
}

impl fmt::Display for PortalFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for PortalFlags {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = PortalFlags::empty();
        for val in s.split("|") {
            match val.trim().to_uppercase().as_ref() {
                "ON_CLIMB" => result |= PortalFlags::ON_CLIMB,
                "ON_DESCEND" => result |= PortalFlags::ON_DESCEND,
                "ON_ENTER" => result |= PortalFlags::ON_ENTER,

                "" => {}
                _ => return Err(format!("Unknown PortalFlags: {}", s)),
            }
        }
        Ok(result)
    }
}

impl From<&str> for PortalFlags {
    fn from(s: &str) -> Self {
        match Self::from_str(s) {
            Ok(flag) => flag,
            Err(err) => panic!("{}", err),
        }
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
