mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// like symlink for submodule hosting

// abs path
// pub is reexport from this module
pub use crate::front_of_house::hosting;

/// relative path ex.
// use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}