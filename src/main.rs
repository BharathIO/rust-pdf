// use genpdf::{
//     elements::FrameCellDecorator, elements::Paragraph, elements::TableLayout, fonts, style::Style,
//     Alignment, SimplePageDecorator,
// };

// use genpdf::style::Style;
use genpdf::style;
use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::{elements, fonts};

const FONT_DIRS: &[&str] = &["/Users/bharath/Work/roboto/"];
const DEFAULT_FONT_NAME: &'static str = "Roboto";

const LOREM_IPSUM: &'static str = "Lorem ipsum dolor sit amet, ";

const IMAGE_PATH_JPG: &'static str = "/Users/bharath/Work/4doto.jpeg";

fn main() {
    // let args: Vec<_> = env::args().skip(1).collect();
    // if args.len() != 1 {
    //     panic!("Missing argument: output file");
    // }
    let output_file = "gen.pdf";
    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");
    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Helvetica))
            .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);

    // let mut decorator = SimplePageDecorator::new();
    // decorator.set_margins(10);
    // decorator.set_header(2);
    // doc.set_page_decorator(decorator);

    // Position::new(0, 0);
    // doc.push(pos);

    let mut decorator = genpdf::SimplePageDecorator::new();

    decorator.set_margins(10);

    let red = style::Color::Rgb(255, 0, 0);

    decorator.set_header(move |page| {
        let mut layout = elements::LinearLayout::vertical();

        let img = elements::Image::from_path(IMAGE_PATH_JPG)
            .expect("Unable to load alt image")
            .with_position(genpdf::Position::new(0, -5))
            .with_scale(genpdf::Scale::new(0.4, 0.4));
        layout.push(img);

        // if page > 1 {
        layout.push(elements::Paragraph::new(format!("Page {}", page)).aligned(Alignment::Center));
        layout.push(elements::Break::new(1));

        // }
        // layout.styled(style::Style::new().with_font_size(10))
        layout.styled(red)
    });
    doc.set_page_decorator(decorator);

    doc.set_title("genpdf Demo Document");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.5);

    // let mut text1 = elements::Text::new("Minimal PDF document");
    let text1 = elements::Paragraph::new("<b>Line1</b>").aligned(Alignment::Center);
    doc.push(text1);
    let text2 = elements::Paragraph::new("line2").aligned(Alignment::Center);
    doc.push(text2);

    doc.push(elements::Paragraph::new("Here is an example table:"));

    let mut table = elements::TableLayout::new(vec![1, 2]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, false, false));
    table
        .row()
        .element(
            elements::Paragraph::new("Header 1")
                // .styled(style::Style::new("red", style::Color::Rgb(255, 0, 0)))
                // .styled_string(
                //     s,
                //     style::StyledStr::new("red", style::Color::Rgb(255, 0, 0)),
                // )
                // .styled(Style::Effect::Bold)
                .padded(1),
        )
        .element(elements::Paragraph::new("Value 2").padded(1))
        .push()
        .expect("Invalid table row");
    table
        .row()
        .element(
            elements::Paragraph::new("Header 2")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new(
                "A long paragraph to demonstrate how wrapping works in tables.  Nice, right?",
            )
            .padded(1),
        )
        .push()
        .expect("Invalid table row");
    let list_layout = elements::LinearLayout::vertical()
        .element(elements::Paragraph::new(
            "Of course, you can use all other elements inside a table.",
        ))
        .element(
            elements::UnorderedList::new()
                .element(elements::Paragraph::new("Even lists!"))
                .element(
                    elements::Paragraph::new("And frames!").padded(genpdf::Margins::vh(0, 1)), // .framed(style::LineStyle::new()),
                ),
        );
    table
        .row()
        .element(
            elements::Paragraph::new("Header 3")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(list_layout.padded(1))
        .push()
        .expect("Invalid table row");
    doc.push(table);
    doc.push(elements::Break::new(1.5));

    doc.push(elements::Paragraph::new(
        "Now let’s print a long table to demonstrate how page wrapping works:",
    ));

    let mut table = elements::TableLayout::new(vec![1, 5]);

    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    table
        .row()
        .element(
            elements::Paragraph::new("Index")
                .styled(style::Effect::Bold)
                .padded(2),
        )
        .element(
            elements::Paragraph::new("Text")
                .styled(style::Effect::Bold)
                .padded(2),
        )
        .push()
        .expect("Invalid table row");
    for i in 0..30 {
        table
            .row()
            .element(elements::Paragraph::new(format!("#{}", i)).padded(1))
            .element(elements::Paragraph::new(LOREM_IPSUM).padded(1))
            .push()
            .expect("Invalid table row");
    }

    doc.push(table);

    doc.render_to_file(output_file)
        .expect("Failed to write output file");
    print!("gen.pdf created");
}
