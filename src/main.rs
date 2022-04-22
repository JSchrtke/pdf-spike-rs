fn main() {
    use printpdf::*;
    use std::fs::File;
    use std::io::BufWriter;

    let page_width = Mm(210.0);
    let page_height = Mm(297.0);
    let (doc, page, layer) =
        PdfDocument::new("PDF_Document_title", page_width, page_height, "Layer 1");
    let times_roman = printpdf::BuiltinFont::TimesRoman;
    let font = doc.add_builtin_font(times_roman).unwrap();

    let current_layer = doc.get_page(page).get_layer(layer);
    let text = "Hello, World!";

    current_layer.use_text(text, 14.0, Mm(0.0), Mm(290.0), &font);

    let file_name = "test_working.pdf";
    doc.save(&mut BufWriter::new(File::create(file_name).unwrap()))
        .unwrap();

    match open::that(file_name) {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    };
}
