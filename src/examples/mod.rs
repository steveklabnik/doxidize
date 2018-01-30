//! Examples of rendering
//!
//! This module and its submodules are purely for show. Check out each type of
//! item and see how Doxidize decides to render them!

/// An example of a module in another module
/// 
/// Nothing super exciting!
pub mod nested_module {
    /// A nested struct
    pub struct NestedStruct {
        pub x: i32,
        _y: i32,
    }

    /// This function should be a method...
    /// 
    /// ... but it isn't, so we can show off what functions look like.
    pub fn this_should_be_a_method(s: NestedStruct) {
        NestedStruct {
            x: 5,
            .. s
        };
    }

    /// another name for `NestedStruct`
    pub type StructAlias = NestedStruct;

    /// let's go deeper
    pub mod next_level1 {
        /// and deeper
        pub mod next_level2 {
            /// and deeper
            pub mod next_level3 {

            }
        }
    }

    /// an empty submodule
    pub mod empty {

    }
}

/// holds nothing, really.
pub static ALWAYS_NONE: Option<i32> = None;

/// holds a five
pub const ALWAYS_FIVE: Option<i32> = Some(5);

/// A point in space
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// create a new point
    /// 
    /// ```
    /// use doxidize::examples::Point;
    /// 
    /// let p = Point::new();
    /// ```
    pub fn new() -> Point {
        Point { x: 0, y: 0 }
    }
}

/// A traffic light
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/// A trait for things that can speak
pub trait Speak {
    /// Actually do the speech!
    fn speak(&self);
}