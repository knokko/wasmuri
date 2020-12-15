mod rectangle;

pub use rectangle::*;

use std::fmt::Debug;

/// Represents a part of the domain of a `Component` and is used to indicate in
/// which part of its domain a component has actually drawn something.
///
/// ### Methods
/// The trait has an `is_inside` method that decides whether a given point lies
/// within this region, or not. Furthermore, this trait has methods to get the
/// `left_bound`, `right_bound`, `bottom_bound`, and `top_bound` of the region,
/// which should always be fast.
///
/// ### Coordinate definitions
/// An x-coordinate of 0.0 indicates the left border of the component domain and
/// an x-coordinate of 1.0 indicates the right border of the component domain. 
/// Similarly, a y-coordinate of 0.0 indicates the bottom border of the component
/// domain and a y-coordinate of 1.0 indicates the top border of the component.
/// 
/// ### Implementations
/// The simplest implementation of this trait is `RectangularDrawnRegion`. I am
/// planning to add more implementations in the future. You can also create
/// your own implementations to define more complex shapes.
pub trait DrawnRegion: Debug {

    /// Checks if the point (x, y) is inside this region and returns true if
    /// (and only if) so
    fn is_inside(&self, x: f32, y: f32) -> bool;

    /// Clones this drawn region. This method should normally return a new
    /// `DrawnRegion` of the same struct as self. Due to Rust rules, this
    /// trait can't simply require implementing structs to implement `Clone`.
    fn clone(&self) -> Box<dyn DrawnRegion>;

    /// Gets the left bound of this region. The `is_inside` method should return
    /// false for any point that is on the left of the left bound (whose
    /// x-coordinate is smaller than the result of this method).
    fn get_left(&self) -> f32;

    /// Gets the bottom bound of this region. The `is_inside` method should return
    /// false for any point that is below the bottom bound (whose y-coordinate
    /// is smaller than the result of this method).
    fn get_bottom(&self) -> f32;

    /// Gets the right bound of this region. The `is_inside` method should return
    /// false for any point that is on the right of the right bound (whose
    /// x-coordinate is larger than the result of this method).
    fn get_right(&self) -> f32;

    /// Gets the top bound of this region. The `is_inside` method should return
    /// false for any point that is above the right bound (whose
    /// y-coordinate is larger than the result of this method).
    fn get_top(&self) -> f32;

    /// Gets the width of this region. This is simply the result of subtracting
    /// the left bound from the right bound.
    fn get_width(&self) -> f32 {
        self.get_right() - self.get_left()
    }

    /// Gets the height of this region. This is simply the result of subtracting
    /// the bottom bound from the top bound.
    fn get_height(&self) -> f32 {
        self.get_top() - self.get_bottom()
    }
}