use embedded_graphics::{
    draw_target::DrawTargetExt,
    geometry::Point,
    prelude::{DrawTarget, ImageDrawable, OriginDimensions, Size},
    primitives::Rectangle,
};

enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

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

    fn draw_corner<D>(&self, corner: Corner, target: &mut D) -> Result<(), D::Error>
    where
        I: ImageDrawable,
        D: DrawTarget<Color = I::Color>,
    {
        // Calculate the translation and area we want to draw depending on which corner it is
        let (translate, area) = match corner {
            Corner::TopLeft => (
                Point::zero(),
                Rectangle::new(
                    Point::zero(),
                    Size::new(self.config.left_width, self.config.top_height),
                ),
            ),
            Corner::TopRight => (
                Point::new(
                    self.config.size.width as i32 - self.config.right_width as i32,
                    0,
                ),
                Rectangle::new(
                    Point::new(
                        self.template_image.size().width as i32 - self.config.right_width as i32,
                        0,
                    ),
                    Size::new(self.config.right_width, self.config.top_height),
                ),
            ),
            Corner::BottomRight => (
                Point::new(
                    self.config.size.width as i32 - self.config.right_width as i32,
                    self.config.size.height as i32 - self.config.bottom_height as i32,
                ),
                Rectangle::new(
                    Point::new(
                        self.template_image.size().width as i32 - self.config.right_width as i32,
                        self.template_image.size().height as i32 - self.config.bottom_height as i32,
                    ),
                    Size::new(self.config.right_width, self.config.bottom_height),
                ),
            ),
            Corner::BottomLeft => (
                Point::new(
                    0,
                    self.config.size.height as i32 - self.config.bottom_height as i32,
                ),
                Rectangle::new(
                    Point::new(
                        0,
                        self.template_image.size().height as i32 - self.config.bottom_height as i32,
                    ),
                    Size::new(self.config.left_width, self.config.bottom_height),
                ),
            ),
        };

        // Actually draw it
        self.template_image
            .draw_sub_image(&mut target.translated(translate), &area)
    }

    fn draw_edge<D>(&self, edge: Edge, target: &mut D) -> Result<(), D::Error>
    where
        I: ImageDrawable,
        D: DrawTarget<Color = I::Color>,
    {
        let (mut edge_len, block_len) = match edge {
            Edge::Top | Edge::Bottom => (
                self.config.size.width - self.config.left_width - self.config.right_width,
                self.template_image.size().width - self.config.left_width - self.config.right_width,
            ),
            Edge::Right | Edge::Left => (
                self.config.size.height - self.config.top_height - self.config.bottom_height,
                self.template_image.size().height
                    - self.config.top_height
                    - self.config.bottom_height,
            ),
        };

        let (mut translation, top_left) = match edge {
            Edge::Top => (
                Point::new(self.config.left_width as i32, 0),
                Point::new(self.config.left_width as i32, 0),
            ),
            Edge::Right => (
                Point::new(
                    self.config.size.width as i32 - self.config.right_width as i32,
                    self.config.top_height as i32,
                ),
                Point::new(
                    self.template_image.size().width as i32 - self.config.right_width as i32,
                    self.config.top_height as i32,
                ),
            ),
            Edge::Bottom => (
                Point::new(
                    self.config.left_width as i32,
                    self.config.size.height as i32 - self.config.bottom_height as i32,
                ),
                Point::new(
                    self.config.left_width as i32,
                    self.template_image.size().height as i32 - self.config.bottom_height as i32,
                ),
            ),
            Edge::Left => (
                Point::new(0, self.config.top_height as i32),
                Point::new(0, self.config.top_height as i32),
            ),
        };

        while edge_len > 0 {
            let paint_len = block_len.min(edge_len);
            self.template_image.draw_sub_image(
                &mut target.translated(translation),
                &Rectangle::new(
                    top_left,
                    match edge {
                        Edge::Top => Size::new(paint_len, self.config.top_height),
                        Edge::Right => Size::new(self.config.right_width, paint_len),
                        Edge::Bottom => Size::new(paint_len, self.config.bottom_height),
                        Edge::Left => Size::new(self.config.left_width, paint_len),
                    },
                ),
            )?;

            translation += match edge {
                Edge::Top | Edge::Bottom => Point::new(paint_len as i32, 0),
                Edge::Left | Edge::Right => Point::new(0, paint_len as i32),
            };

            edge_len -= paint_len;
        }

        Ok(())
    }

    fn draw_center<D>(&self, _target: &mut D) -> Result<(), D::Error>
    where
        I: ImageDrawable,
        D: DrawTarget<Color = I::Color>,
    {
        if self.config.fill_center {
            todo!()
        }
        Ok(())
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
        // Start with corners
        self.draw_corner(Corner::TopLeft, target)?;
        self.draw_corner(Corner::TopRight, target)?;
        self.draw_corner(Corner::BottomRight, target)?;
        self.draw_corner(Corner::BottomLeft, target)?;

        // Then do edges
        self.draw_edge(Edge::Top, target)?;
        self.draw_edge(Edge::Right, target)?;
        self.draw_edge(Edge::Bottom, target)?;
        self.draw_edge(Edge::Left, target)?;

        // Do the center
        self.draw_center(target)
    }

    fn draw_sub_image<D>(&self, _target: &mut D, _area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        unimplemented!()
    }
}
