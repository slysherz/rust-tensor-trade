extern crate uuid;

use crate::ttcore::clock::Clock;
use uuid::Uuid;

static GLOBAL_CLOCK: Clock = Clock { 
    start: 0, 
    step: 0 
};

/// A trait for objects that are indexed by time
pub trait TimeIndexed {
}

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

pub fn same_object<T>(a: &T, b: &T) -> bool {
    a as *const T == b as *const T
}