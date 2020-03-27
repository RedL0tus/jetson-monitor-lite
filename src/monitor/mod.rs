mod display;

use embedded_graphics::{
    drawable::Pixel,
    fonts::{Font6x8, Font8x16, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, Rectangle},
    style::{PrimitiveStyle, PrimitiveStyleBuilder, TextStyle, TextStyleBuilder},
};

use super::info::InfoBundle;

use display::DisplayWrapper;

macro_rules! draw {
    ($text:expr, $style:expr, $display:expr, $x:expr, $y:expr) => {
        Text::new($text, Point::new($x, $y))
            .into_styled($style)
            .draw($display)
            .unwrap();
    };
    ($primitive:ident, $style:expr, $display:expr, $($x: expr, $y:expr), +) => {
        $primitive::new(
            $(
                Point::new($x, $y),
            )*
        )
            .into_styled($style)
            .draw($display)
            .unwrap();
    };
}

pub struct Monitor {
    display: DisplayWrapper,
    primitive_style: PrimitiveStyle<BinaryColor>,
    large_text_style: TextStyle<BinaryColor, Font8x16>,
    normal_text_style: TextStyle<BinaryColor, Font6x8>,
    info_bundle: InfoBundle,
}

impl Monitor {
    pub fn new(addr: u8, device: &'static str) -> Self {
        let display: DisplayWrapper = DisplayWrapper::new(addr, &device);
        let primitive_style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build();
        let large_text_style = TextStyleBuilder::new(Font8x16)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();
        let normal_text_style = TextStyleBuilder::new(Font6x8)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();
        let info_bundle = InfoBundle::new();
        Self {
            display,
            primitive_style,
            large_text_style,
            normal_text_style,
            info_bundle,
        }
    }

    fn draw_texts(&mut self) {
        draw!(
            self.info_bundle.hostname.as_str(),
            self.large_text_style,
            &mut self.display.inner,
            0,
            0
        );
        draw!(
            "temp:",
            self.normal_text_style,
            &mut self.display.inner,
            0,
            18
        );
        draw!(
            self.info_bundle.temperature.as_str(),
            self.normal_text_style,
            &mut self.display.inner,
            6,
            28
        );
        draw!(
            "f-lvl:",
            self.normal_text_style,
            &mut self.display.inner,
            0,
            39
        );
        draw!(
            self.info_bundle.fan_level.as_str(),
            self.normal_text_style,
            &mut self.display.inner,
            6,
            50
        );
        draw!(
            self.info_bundle.loadavg.as_str(),
            self.normal_text_style,
            &mut self.display.inner,
            100,
            8
        );
    }

    fn draw_graph(
        &mut self,
        step: i32,
        upper_left_x: i32,
        upper_left_y: i32,
        lower_right_x: i32,
        lower_right_y: i32,
    ) {
        // Draw frame
        draw!(
            Rectangle,
            self.primitive_style,
            &mut self.display.inner,
            upper_left_x,
            upper_left_y,
            lower_right_x,
            lower_right_y
        );

        // Draw lines
        let mut points: (i32, i32) = (lower_right_y, lower_right_y);
        for index in 0..((lower_right_x - upper_left_x) / step) {
            points.0 = points.1;
            points.1 = lower_right_y
                - ((((lower_right_y - upper_left_y) as f32)
                    * self.info_bundle.cpu_load.inner[index as usize]) as i32);
            let horizontal_next: i32 = upper_left_x + (step * index as i32);
            let horizontal_previous: i32 = if index == 0 {
                horizontal_next
            } else {
                horizontal_next - step
            };
            for point in (upper_left_y..lower_right_y).step_by(step as usize) {
                if point > points.1 {
                    Pixel(Point::new(horizontal_next, point), BinaryColor::On)
                        .draw(&mut self.display.inner)
                        .unwrap();
                }
            }
            draw!(
                Line,
                self.primitive_style,
                &mut self.display.inner,
                horizontal_previous,
                points.0,
                horizontal_next,
                points.1
            );
        }
    }

    pub fn update(&mut self) {
        self.display.inner.clear();
        self.info_bundle.update();
        self.draw_texts();
        self.draw_graph(
            5, 42, 17, // X and Y for the upper left corner
            127, 62, // X and Y for the lower right corner
        );
        self.display.inner.flush().unwrap();
    }
}
