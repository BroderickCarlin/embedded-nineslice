use embedded_graphics::{
    draw_target::DrawTargetExt,
    geometry::Point,
    prelude::{DrawTarget, ImageDrawable, OriginDimensions, Size},
    primitives::Rectangle,
};

pub struct NineSliceConfig {
    pub size: Size,
    pub left_width: u32,
    pub right_width: u32,
    pub top_height: u32,
    pub bottom_height: u32,
    pub fill_center: bool,
}

pub struct NineSlice<'a, I> {
    template_image: &'a I,
    config: NineSliceConfig,
}

impl<'a, I> NineSlice<'a, I> {
    pub fn new(template_image: &'a I, config: NineSliceConfig) -> Self {
        Self {
            template_image,
            config,
        }
    }
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
        // Start by drawing the top left
        self.template_image.draw_sub_image(
            target,
            &Rectangle::new(
                Point::zero(),
                Size::new(self.config.left_width, self.config.top_height),
            ),
        )?;

        // Draw the top right
        self.template_image.draw_sub_image(
            &mut target.translated(Point::new(
                self.config.size.width as i32 - self.config.right_width as i32,
                0,
            )),
            &Rectangle::new(
                Point::new(
                    self.template_image.size().width as i32 - self.config.right_width as i32,
                    0,
                ),
                Size::new(self.config.right_width, self.config.top_height),
            ),
        )?;

        // Draw the bottom right
        self.template_image.draw_sub_image(
            &mut target.translated(Point::new(
                self.config.size.width as i32 - self.config.right_width as i32,
                self.config.size.height as i32 - self.config.bottom_height as i32,
            )),
            &Rectangle::new(
                Point::new(
                    self.template_image.size().width as i32 - self.config.right_width as i32,
                    self.template_image.size().height as i32 - self.config.bottom_height as i32,
                ),
                Size::new(self.config.right_width, self.config.bottom_height),
            ),
        )?;

        // Draw the bottom left
        self.template_image.draw_sub_image(
            &mut target.translated(Point::new(
                0,
                self.config.size.height as i32 - self.config.bottom_height as i32,
            )),
            &Rectangle::new(
                Point::new(
                    0,
                    self.template_image.size().height as i32 - self.config.bottom_height as i32,
                ),
                Size::new(self.config.left_width, self.config.bottom_height),
            ),
        )?;

        // Move on to drawing the edges

        // Draw the top edge
        let mut edge_len =
            self.config.size.width - self.config.left_width - self.config.right_width;
        let block_len =
            self.template_image.size().width - self.config.left_width - self.config.right_width;
        let mut translation = Point::new(self.config.left_width as i32, 0);
        while edge_len > 0 {
            let paint_len = block_len.min(edge_len);
            self.template_image.draw_sub_image(
                &mut target.translated(translation),
                &Rectangle::new(
                    Point::new(self.config.left_width as i32, 0),
                    Size::new(paint_len, self.config.top_height),
                ),
            )?;

            translation += Point::new(paint_len as i32, 0);

            edge_len -= paint_len;
        }

        // Draw the right edge
        let mut edge_len =
            self.config.size.height - self.config.top_height - self.config.bottom_height;
        let block_len =
            self.template_image.size().height - self.config.top_height - self.config.bottom_height;
        let mut translation = Point::new(
            self.config.size.width as i32 - self.config.right_width as i32,
            self.config.top_height as i32,
        );
        while edge_len > 0 {
            let paint_len = block_len.min(edge_len);
            self.template_image.draw_sub_image(
                &mut target.translated(translation),
                &Rectangle::new(
                    Point::new(
                        self.template_image.size().width as i32 - self.config.right_width as i32,
                        self.config.top_height as i32,
                    ),
                    Size::new(self.config.right_width, paint_len),
                ),
            )?;

            translation += Point::new(0, paint_len as i32);

            edge_len -= paint_len;
        }

        // Draw the bottom edge
        let mut edge_len =
            self.config.size.width - self.config.left_width - self.config.right_width;
        let block_len =
            self.template_image.size().width - self.config.left_width - self.config.right_width;
        let mut translation = Point::new(
            self.config.left_width as i32,
            self.config.size.height as i32 - self.config.bottom_height as i32,
        );
        while edge_len > 0 {
            let paint_len = block_len.min(edge_len);
            self.template_image.draw_sub_image(
                &mut target.translated(translation),
                &Rectangle::new(
                    Point::new(
                        self.config.left_width as i32,
                        self.template_image.size().height as i32 - self.config.bottom_height as i32,
                    ),
                    Size::new(paint_len, self.config.top_height),
                ),
            )?;

            translation += Point::new(paint_len as i32, 0);

            edge_len -= paint_len;
        }

        // Draw the left edge
        let mut edge_len =
            self.config.size.height - self.config.top_height - self.config.bottom_height;
        let block_len =
            self.template_image.size().height - self.config.top_height - self.config.bottom_height;
        let mut translation = Point::new(0, self.config.top_height as i32);
        while edge_len > 0 {
            let paint_len = block_len.min(edge_len);
            self.template_image.draw_sub_image(
                &mut target.translated(translation),
                &Rectangle::new(
                    Point::new(0, self.config.top_height as i32),
                    Size::new(self.config.left_width, paint_len),
                ),
            )?;

            translation += Point::new(0, paint_len as i32);

            edge_len -= paint_len;
        }

        // Do the center
        if self.config.fill_center {
            todo!()
        }

        Ok(())
    }

    fn draw_sub_image<D>(&self, _target: &mut D, _area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        unimplemented!()
    }
}
