use embedded_graphics::{
    geometry::{Dimensions, Point},
    prelude::Size,
    primitives::Rectangle,
};

pub struct Config {
    pub size: Size,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
    pub fill_center: bool,
}

impl Config {
    pub(crate) fn bounding_box_top_left<T: Dimensions>(&self, template: &T) -> Rectangle {
        Rectangle::new(
            template.bounding_box().top_left,
            Size::new(self.left, self.top),
        )
    }

    pub(crate) fn offset_top_left(&self) -> Point {
        Point::zero()
    }

    pub(crate) fn bounding_box_top_right<T: Dimensions>(&self, template: &T) -> Rectangle {
        let t_bb = template.bounding_box();
        let top_left = Point::new(
            t_bb.top_left.x + t_bb.size.width as i32 - self.right as i32,
            t_bb.top_left.y,
        );

        Rectangle::new(top_left, Size::new(self.right, self.top))
    }

    pub(crate) fn offset_top_right(&self) -> Point {
        Point::new(self.size.width as i32 - self.right as i32, 0)
    }

    pub(crate) fn bounding_box_bottom_left<T: Dimensions>(&self, template: &T) -> Rectangle {
        let t_bb = template.bounding_box();
        let top_left = Point::new(
            t_bb.top_left.x,
            t_bb.top_left.y + t_bb.size.height as i32 - self.bottom as i32,
        );

        Rectangle::new(top_left, Size::new(self.left, self.bottom))
    }

    pub(crate) fn offset_bottom_left(&self) -> Point {
        Point::new(0, self.size.height as i32 - self.bottom as i32)
    }

    pub(crate) fn bounding_box_bottom_right<T: Dimensions>(&self, template: &T) -> Rectangle {
        let t_bb = template.bounding_box();
        let top_left = Point::new(
            t_bb.top_left.x + t_bb.size.width as i32 - self.right as i32,
            t_bb.top_left.y + t_bb.size.height as i32 - self.bottom as i32,
        );

        Rectangle::new(top_left, Size::new(self.right, self.bottom))
    }

    pub(crate) fn offset_bottom_right(&self) -> Point {
        Point::new(
            self.size.width as i32 - self.right as i32,
            self.size.height as i32 - self.bottom as i32,
        )
    }

    pub(crate) fn bounding_box_top<T: Dimensions>(&self, template: &T) -> Rectangle {
        let t_bb = template.bounding_box();
        let top_left = Point::new(t_bb.top_left.x + self.left as i32, t_bb.top_left.y);

        Rectangle::new(
            top_left,
            Size::new(t_bb.size.width - self.right - self.left, self.top),
        )
    }

    pub(crate) fn offset_top(&self) -> Point {
        Point::new(self.left as i32, 0)
    }

    pub(crate) fn bounding_box_right<T: Dimensions>(&self, template: &T) -> Rectangle {
        let t_bb = template.bounding_box();
        let top_left = Point::new(
            t_bb.top_left.x + t_bb.size.width as i32 - self.right as i32,
            t_bb.top_left.y + self.top as i32,
        );

        Rectangle::new(
            top_left,
            Size::new(self.right, t_bb.size.height - self.top - self.bottom),
        )
    }

    pub(crate) fn offset_right(&self) -> Point {
        Point::new(self.size.width as i32 - self.right as i32, self.top as i32)
    }

    pub(crate) fn bounding_box_bottom<T: Dimensions>(&self, template: &T) -> Rectangle {
        let t_bb = template.bounding_box();
        let top_left = Point::new(
            t_bb.top_left.x + self.left as i32,
            t_bb.top_left.y + t_bb.size.height as i32 - self.bottom as i32,
        );

        Rectangle::new(
            top_left,
            Size::new(t_bb.size.width - self.right - self.left, self.bottom),
        )
    }

    pub(crate) fn offset_bottom(&self) -> Point {
        Point::new(
            self.left as i32,
            self.size.height as i32 - self.bottom as i32,
        )
    }

    pub(crate) fn bounding_box_left<T: Dimensions>(&self, template: &T) -> Rectangle {
        let t_bb = template.bounding_box();
        let top_left = Point::new(t_bb.top_left.x, t_bb.top_left.y + self.top as i32);

        Rectangle::new(
            top_left,
            Size::new(self.left, t_bb.size.height - self.top - self.bottom),
        )
    }

    pub(crate) fn offset_left(&self) -> Point {
        Point::new(0, self.top as i32)
    }

    pub(crate) fn bounding_box_center<T: Dimensions>(&self, template: &T) -> Rectangle {
        let t_bb = template.bounding_box();
        let top_left = Point::new(
            t_bb.top_left.x + self.left as i32,
            t_bb.top_left.y + self.top as i32,
        );

        Rectangle::new(
            top_left,
            Size::new(
                t_bb.size.width - self.right - self.left,
                t_bb.size.height - self.top - self.bottom,
            ),
        )
    }

    pub(crate) fn offset_center(&self) -> Point {
        Point::new(self.left as i32, self.top as i32)
    }
}
