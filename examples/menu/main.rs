use embedded_graphics::{
    image::Image,
    mono_font::{ascii::FONT_7X13, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_nineslice::{NineSlice, NineSliceConfig};
use tinybmp::Bmp;

const BORDER_IMAGE_RAW: &[u8] = include_bytes!("border.bmp");

fn draw_text<D: DrawTarget<Color = Rgb888>>(text: &str, position: Point, display: &mut D) {
    let character_style = MonoTextStyleBuilder::new()
        .font(&FONT_7X13)
        .text_color(Rgb888::CSS_BLACK)
        .build();

    let center_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Middle)
        .build();

    let _ = Text::with_text_style(text, position, character_style, center_aligned).draw(display);
}

fn draw_background<C: PixelColor, D: DrawTarget<Color = C> + OriginDimensions>(
    color: C,
    display: &mut D,
) {
    let background_fill = PrimitiveStyle::with_fill(color);

    let _ = Rectangle::new(Point::zero(), display.size())
        .into_styled(background_fill)
        .draw(display);
}

fn main() {
    let display_resolution = Size::new(160, 144);
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(display_resolution);
    draw_background(Rgb888::new(255, 255, 255), &mut display);

    let border_bmp = Bmp::from_slice(BORDER_IMAGE_RAW).unwrap();

    let cfg = NineSliceConfig {
        size: Size::new(80, 128),
        left_width: 8,
        top_height: 8,
        right_width: 8,
        bottom_height: 8,
        fill_center: false,
    };

    let border_nineslice = NineSlice::new(&border_bmp, cfg);

    let border_image = Image::new(&border_nineslice, Point::new(80, 0));

    border_image.draw(&mut display).unwrap();

    let spacing = 15;
    let x_inset = 97;
    let mut offset = 20;

    let menu_items = &["MONSTERS", "PARTY", "ITEM", "ME", "SAVE", "OPTION", "EXIT"];

    for item in menu_items {
        draw_text(item, Point::new(x_inset, offset), &mut display);
        offset += spacing
    }

    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    Window::new("BMP image", &output_settings).show_static(&display);
}
