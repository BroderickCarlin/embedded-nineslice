use embedded_graphics::{
    draw_target::DrawTargetExt,
    geometry::Point,
    prelude::{DrawTarget, ImageDrawable, OriginDimensions, Size},
    primitives::Rectangle,
};

pub use config::Config;

mod config;

pub struct NineSlice<'a, I> {
    template_image: &'a I,
    config: Config,
}

impl<'a, I> NineSlice<'a, I> {
    pub fn new(template_image: &'a I, config: Config) -> Self {
        Self {
            template_image,
            config,
        }
    }

    fn draw_region<D>(
        &self,
        area: &Rectangle,
        position: Point,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        I: ImageDrawable,
        D: DrawTarget<Color = I::Color>,
    {
        self.template_image
            .draw_sub_image(&mut target.translated(position), &area)
    }

    fn tile_region<D>(&self) -> Result<(), D::Error>
    where
        I: ImageDrawable,
        D: DrawTarget<Color = I::Color>,
    {
        unimplemented!()
    }

    // fn draw_edge<D>(&self, edge: Edge, area: &Rectangle, target: &mut D) -> Result<(), D::Error>
    // where
    //     I: ImageDrawable,
    //     D: DrawTarget<Color = I::Color>,
    // {
    //     let (mut edge_len, block_len) = match edge {
    //         Edge::Top | Edge::Bottom => (
    //             self.config.size.width - self.config.left - self.config.right,
    //             self.template_image.size().width - self.config.left - self.config.right,
    //         ),
    //         Edge::Right | Edge::Left => (
    //             self.config.size.height - self.config.top - self.config.bottom,
    //             self.template_image.size().height - self.config.top - self.config.bottom,
    //         ),
    //     };

    //     let (mut translation, top_left) = match edge {
    //         Edge::Top => (
    //             Point::new(self.config.left as i32, 0),
    //             Point::new(self.config.left as i32, 0),
    //         ),
    //         Edge::Right => (
    //             Point::new(
    //                 self.config.size.width as i32 - self.config.right as i32,
    //                 self.config.top as i32,
    //             ),
    //             Point::new(
    //                 self.template_image.size().width as i32 - self.config.right as i32,
    //                 self.config.top as i32,
    //             ),
    //         ),
    //         Edge::Bottom => (
    //             Point::new(
    //                 self.config.left as i32,
    //                 self.config.size.height as i32 - self.config.bottom as i32,
    //             ),
    //             Point::new(
    //                 self.config.left as i32,
    //                 self.template_image.size().height as i32 - self.config.bottom as i32,
    //             ),
    //         ),
    //         Edge::Left => (
    //             Point::new(0, self.config.top as i32),
    //             Point::new(0, self.config.top as i32),
    //         ),
    //     };

    //     while edge_len > 0 {
    //         let paint_len = block_len.min(edge_len);
    //         self.template_image.draw_sub_image(
    //             &mut target.translated(translation),
    //             &Rectangle::new(
    //                 top_left,
    //                 match edge {
    //                     Edge::Top => Size::new(paint_len, self.config.top),
    //                     Edge::Right => Size::new(self.config.right, paint_len),
    //                     Edge::Bottom => Size::new(paint_len, self.config.bottom),
    //                     Edge::Left => Size::new(self.config.left, paint_len),
    //                 },
    //             ),
    //         )?;

    //         translation += match edge {
    //             Edge::Top | Edge::Bottom => Point::new(paint_len as i32, 0),
    //             Edge::Left | Edge::Right => Point::new(0, paint_len as i32),
    //         };

    //         edge_len -= paint_len;
    //     }

    //     Ok(())
    // }

    // fn draw_center<D>(&self, area: &Rectangle, _target: &mut D) -> Result<(), D::Error>
    // where
    //     I: ImageDrawable,
    //     D: DrawTarget<Color = I::Color>,
    // {
    //     if self.config.fill_center {
    //         todo!()
    //     }
    //     Ok(())
    // }
}

impl<'a, I> OriginDimensions for NineSlice<'a, I> {
    fn size(&self) -> Size {
        self.config.size
    }
}

impl<'a, I> ImageDrawable for NineSlice<'a, I>
where
    I: ImageDrawable,
{
    type Color = I::Color;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color> + DrawTargetExt,
    {
        // Create a rect that encompasses the entire image
        let area = Rectangle::new(Point::zero(), self.config.size);

        // Draw sub-image with our entire image
        self.draw_sub_image(target, &area)
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        // Start with corners
        self.draw_region(
            &self
                .config
                .bounding_box_top_left(self.template_image)
                .intersection(area),
            self.config.offset_top_left(),
            target,
        )?;
        self.draw_region(
            &self
                .config
                .bounding_box_top_right(self.template_image)
                .intersection(area),
            self.config.offset_top_right(),
            target,
        )?;
        self.draw_region(
            &self
                .config
                .bounding_box_bottom_right(self.template_image)
                .intersection(area),
            self.config.offset_bottom_right(),
            target,
        )?;
        self.draw_region(
            &self
                .config
                .bounding_box_bottom_left(self.template_image)
                .intersection(area),
            self.config.offset_bottom_left(),
            target,
        )?;

        // // Then do edges
        // self.draw_edge(Edge::Top, area, target)?;
        // self.draw_edge(Edge::Right, area, target)?;
        // self.draw_edge(Edge::Bottom, area, target)?;
        // self.draw_edge(Edge::Left, area, target)?;

        // // Do the center
        // self.draw_center(area, target)

        Ok(())
    }
}
