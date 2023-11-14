use embedded_graphics::{
    image::Image,
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_nineslice::{NineSlice, NineSliceConfig};
use tinybmp::Bmp;

const BORDER_IMAGE_RAW: &[u8] = include_bytes!("border.bmp");

fn main() {
    let display_resolution = Size::new(160, 144);
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(display_resolution);
    let border_bmp = Bmp::from_slice(BORDER_IMAGE_RAW).unwrap();

    let cfg = NineSliceConfig {
        size: Size::new(160, 30),
        left_width: 7,
        top_height: 7,
        right_width: 7,
        bottom_height: 7,
        fill_center: false,
    };

    let border_nineslice = NineSlice::new(&border_bmp, cfg);

    let border_image = Image::new(&border_nineslice, Point::zero());

    let background_fill = PrimitiveStyle::with_fill(Rgb888::new(255, 241, 232));

    Rectangle::new(Point::zero(), display_resolution)
        .into_styled(background_fill)
        .draw(&mut display)
        .unwrap();

    border_image.draw(&mut display).unwrap();

    let character_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(Rgb888::CSS_SLATE_GRAY)
        .build();

    let center_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();

    Text::with_text_style(
        "Example",
        Point::new(80, 15),
        character_style,
        center_aligned,
    )
    .draw(&mut display)
    .unwrap();

    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    Window::new("BMP image", &output_settings).show_static(&display);
}
