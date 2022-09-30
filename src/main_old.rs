// use printpdf::*;
// use std::fs::File;
// use std::io::BufWriter;

extern crate printpdf;
// use image_crate::codecs::bmp::BmpDecoder;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
// use std::io::Cursor;

fn main() {
    // write_text();
    draw_line();
}

fn draw_line() {
    let initial_page_width = Mm(612.0);
    let initial_page_height = Mm(792.0);
    // let font_size = 50.0;
    let (doc, page1, layer1) = PdfDocument::new(
        "printpdf graphics test",
        initial_page_width,
        initial_page_height,
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let x = Mm(50.0);
    let y = Mm(700.0);
    let y2 = Mm(750.0);

    let x_n = Mm(550.0);
    let mut vert_line =
        Line::from_iter(vec![(Point::new(x, y), false), (Point::new(x, y2), false)]);
    vert_line.set_stroke(true);

    current_layer.add_shape(vert_line);

    let mut vert_line2 = Line::from_iter(vec![
        (Point::new(x_n, y), false),
        (Point::new(x_n, y2), false),
    ]);
    vert_line2.set_stroke(true);
    current_layer.add_shape(vert_line2);

    let mut hor_line1 =
        Line::from_iter(vec![(Point::new(x, y), false), (Point::new(x_n, y), false)]);
    hor_line1.set_stroke(true);
    current_layer.add_shape(hor_line1);

    let mut hor_line2 = Line::from_iter(vec![
        (Point::new(x, y2), false),
        (Point::new(x_n, y2), false),
    ]);
    hor_line2.set_stroke(true);
    current_layer.add_shape(hor_line2);

    // Quadratic shape. The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    // If you want holes, simply reorder the winding of the points to be
    // counterclockwise instead of clockwise.
    let points1 = vec![
        (Point::new(Mm(100.0), Mm(100.0)), false),
        (Point::new(Mm(100.0), Mm(200.0)), false),
        (Point::new(Mm(300.0), Mm(200.0)), false),
        (Point::new(Mm(300.0), Mm(100.0)), false),
    ];

    // Is the shape stroked? Is the shape closed? Is the shape filled?
    let line1 = Line {
        points: points1,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    let fill_color = Color::Cmyk(Cmyk::new(0.0, 0.23, 0.0, 0.0, None));
    let outline_color = Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None));
    let mut dash_pattern = LineDashPattern::default();
    dash_pattern.dash_1 = Some(20);

    current_layer.set_fill_color(fill_color);
    current_layer.set_outline_color(outline_color);
    current_layer.set_outline_thickness(10.0);

    // Draw first line
    current_layer.add_shape(line1);
    let fill_color_2 = Color::Cmyk(Cmyk::new(0.0, 0.0, 0.0, 0.0, None));
    let outline_color_2 = Color::Greyscale(Greyscale::new(0.45, None));

    // More advanced graphical options
    current_layer.set_overprint_stroke(true);
    current_layer.set_blend_mode(BlendMode::Seperable(SeperableBlendMode::Multiply));
    current_layer.set_line_dash_pattern(dash_pattern);
    current_layer.set_line_cap_style(LineCapStyle::Round);
    current_layer.set_line_join_style(LineJoinStyle::Round);
    current_layer.set_fill_color(fill_color_2);
    current_layer.set_outline_color(outline_color_2);
    current_layer.set_outline_thickness(15.0);

    // Text
    let font2 = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let text = "Customer Load Audit Report";
    let text2 = "Date <h1> Heading </h1>";
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let x = Mm(200.0);
    let y = Mm(730.0);
    let fill_color = Color::Cmyk(Cmyk::new(100.0, 88.0, 0.0, 0.0, None));
    current_layer.set_fill_color(fill_color);
    current_layer.set_outline_thickness(10.0);
    current_layer.begin_text_section();
    // setup the general fonts.
    // see the docs for these functions for details
    current_layer.set_font(&font2, 40.0);
    current_layer.set_text_cursor(x, y);
    current_layer.set_line_height(33.0);
    // current_layer.set_word_spacing(3000.0);
    // current_layer.set_character_spacing(10.0);
    // current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
    // write two lines (one line break)
    current_layer.write_text(text.clone(), &font2);
    // current_layer.add_line_break();
    // current_layer.write_text(text2.clone(), &font2);
    current_layer.add_line_break();
    // write one line, but write text2 in superscript
    // current_layer.set_text_cursor(text2_x, text2_y);
    current_layer.set_line_height(33.0);
    current_layer.set_font(&font2, 30.0);
    // current_layer.set_text_cursor(text2_x, text2_y);
    // current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
    current_layer.write_text(text2.clone(), &font2);
    // current_layer.set_line_offset(10.0);
    // current_layer.write_text(text2.clone(), &font2);
    current_layer.end_text_section();

    println!("Creating pdf..");
    doc.save(&mut BufWriter::new(File::create("test_line.pdf").unwrap()))
        .unwrap();
    println!("Done ! test_line.pdf");
}

fn _write_text() {
    let initial_page_width = Mm(612.0);
    let initial_page_height = Mm(792.0);
    // let font_size = 50.0;
    let (doc, page1, layer1) = PdfDocument::new(
        "printpdf graphics test",
        initial_page_width,
        initial_page_height,
        "Layer 1",
    );
    let font2 = doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
    let text = "Customer Load Audit Report";
    let text2 = "Date";
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let x = Mm(200.0);
    let y = Mm(742.0);
    let fill_color = Color::Cmyk(Cmyk::new(100.0, 88.0, 0.0, 0.0, None));
    current_layer.set_fill_color(fill_color);
    current_layer.set_outline_thickness(10.0);
    current_layer.begin_text_section();
    // setup the general fonts.
    // see the docs for these functions for details
    current_layer.set_font(&font2, 40.0);
    current_layer.set_text_cursor(x, y);
    current_layer.set_line_height(33.0);
    // current_layer.set_word_spacing(3000.0);
    // current_layer.set_character_spacing(10.0);
    // current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
    // write two lines (one line break)
    current_layer.write_text(text.clone(), &font2);
    // current_layer.add_line_break();
    // current_layer.write_text(text2.clone(), &font2);
    current_layer.add_line_break();
    // write one line, but write text2 in superscript
    // current_layer.set_text_cursor(text2_x, text2_y);
    current_layer.set_line_height(33.0);
    current_layer.set_font(&font2, 30.0);
    // current_layer.set_text_cursor(text2_x, text2_y);
    // current_layer.set_text_rendering_mode(TextRenderingMode::Stroke);
    current_layer.write_text(text2.clone(), &font2);
    // current_layer.set_line_offset(10.0);
    // current_layer.write_text(text2.clone(), &font2);
    current_layer.end_text_section();

    println!("Creating pdf..");
    doc.save(&mut BufWriter::new(
        File::create("test_working.pdf").unwrap(),
    ))
    .unwrap();
    println!("Done ! test_working.pdf");
}
