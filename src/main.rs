fn main() {
    use pdf_canvas::{BuiltinFont, Pdf};

    let pdf_name = "example.pdf";
    let mut document = Pdf::create(pdf_name).expect("Create pdf file");
    let font = BuiltinFont::Times_Roman;

    let width = point_from_mm(210.0);
    let height = point_from_mm(297.0);
    document
        .render_page(width, height, |canvas| {
            let hello = "Hello World!";
            let font_size = 24.0;

            canvas.left_text(0.0, height - font_size, font, font_size, hello)
        })
        .expect("Write page");
    document.finish().expect("Finish pdf document");

    open::that(pdf_name).unwrap();
}

fn point_from_mm(mm: f32) -> f32 {
    let factor = 1.0 / 0.352778;
    mm * factor
}
