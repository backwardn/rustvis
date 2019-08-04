extern crate image;
extern crate imageproc;
use image::{DynamicImage, Rgba, RgbaImage, GenericImage};
use imageproc::drawing::*;
use crate::{Rgb};
use crate::text::draw_text;
use crate::barchart::{Chart, draw_labels};
use crate::drawing::*;
use imageproc::rect::Rect;
use imageproc::pixelops::interpolate;

/// Draw a linechart, with a specified title and data.
///
/// #### Arguments
/// * `img` - Image to draw the linechart onto.
/// * `chart` - Chart struct, which contains all data & meta-data about the barchart.
pub fn draw_linechart(mut img: &mut DynamicImage, chart: &Chart) {
    draw_lines(img, chart, false);
}

/// Draw a linechart with points, containing a specified title and data.
///
/// #### Arguments
/// * `img` - Image to draw the linechart onto.
/// * `chart` - Chart struct, which contains all data & metadata about the barchart.
pub fn draw_linechart_points(mut img: &mut DynamicImage, chart: &Chart) {
    draw_lines(img, chart, true);
}

fn draw_lines(mut img: &mut DynamicImage, chart: &Chart, points: bool) {
    draw_labels(&mut img, chart);
    let axis_len = chart.width as f32 * 0.8;
    let y_origin = 20.0 + axis_len;

    let x_inc = axis_len / chart.data.len() as f32;

    let mut start_x = 20.0;
    let line_pixel = image::Rgba([255, 167, 90, 255]);

    let white = image::Rgba([155, 155, 155, 255]);

    let max_item = chart.data.iter().max().unwrap();

    let mut start_y = y_origin;
    
    for item in &chart.data {
        let div: f32 = *max_item as f32 / *item as f32;

        let end_x: i32 = (start_x + x_inc) as i32;
        let end_y: i32 = (y_origin - (axis_len / div)) as i32;
        
        for i in 0..3 {
            draw_antialiased_line_segment_mut(img, (start_x as i32 + i, start_y as i32), (end_x + i, end_y), line_pixel, interpolate);
        }

        if (points) { 
            draw_filled_circle_mut(img, (end_x, end_y), 4, white);
        }

        start_x += x_inc;
        start_y = end_y as f32;
    }
}