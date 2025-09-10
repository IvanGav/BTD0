use bevy::prelude::*;

#[derive(PartialEq)]
pub enum Upgradable {
    Crosspath520(u8,u8,u8),
    None,
    Hero20(u8),
}

impl Upgradable {
    /// Return true if path 1 is upgradable
    pub fn can_upgrade_1(&self)->bool {
        if let Upgradable::Crosspath520(p1, p2, p3) = *self {
            if p2 > 0 && p3 > 0 { return false; } // blocked path
            if p1 == 2 && (p2 > 2 || p3 > 2) { return false; } // maxed out crosspath
            if p1 == 5 { return false; } // maxed out
            return true;
        }
        return false;
    }
    /// Return true if path 2 is upgradable
    pub fn can_upgrade_2(&self)->bool {
        if let Upgradable::Crosspath520(p1, p2, p3) = *self {
            if p1 > 0 && p3 > 0 { return false; } // blocked path
            if p2 == 2 && (p1 > 2 || p3 > 2) { return false; } // maxed out crosspath
            if p2 == 5 { return false; } // maxed out
            return true;
        }
        return false;
    }
    /// Return true if path 3 is upgradable
    pub fn can_upgrade_3(&self)->bool {
        if let Upgradable::Crosspath520(p1, p2, p3) = *self {
            if p2 > 0 && p1 > 0 { return false; } // blocked path
            if p3 == 2 && (p2 > 2 || p1 > 2) { return false; } // maxed out crosspath
            if p3 == 5 { return false; } // maxed out
            return true;
        }
        return false;
    }
}