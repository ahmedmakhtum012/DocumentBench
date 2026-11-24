use pdfium_render::prelude::*;
use std::path::PathBuf;
use base64::{engine::general_purpose::STANDARD as B64,Engine as _};

fn library_path()->PathBuf{
    let exe=std::env::current_exe().unwrap_or_default();
    [exe.parent().unwrap_or_else(||std::path::Path::new(".")).join("pdfium.dll"),PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"),"/resources/pdfium.dll")),PathBuf::from("src-tauri/resources/pdfium.dll"),PathBuf::from("resources/pdfium.dll")].into_iter().find(|p|p.exists()).unwrap_or_else(||PathBuf::from("pdfium.dll"))
}
pub fn engine()->Result<Pdfium,String>{let bind=Pdfium::bind_to_library(library_path()).map_err(|e|format!("PDFium could not be loaded: {e}"))?;Ok(Pdfium::new(bind))}
pub fn render_png(bytes:&[u8],page_number:u32,width:u32,grayscale:bool)->Result<String,String>{
    let engine=engine()?;let doc=engine.load_pdf_from_byte_slice(bytes,None).map_err(|e|format!("Unable to open PDF: {e}"))?;let pages=doc.pages();let page=pages.get((page_number.saturating_sub(1))as u16).map_err(|_|"That page is outside this document.".to_string())?;
    let height=(width as f32*page.height().value/page.width().value)as u32;
    let bitmap=page.render_with_config(&PdfRenderConfig::new().set_target_width(width as i32).set_maximum_height(height as i32).rotate_if_landscape(PdfPageRenderRotation::None,false)).map_err(|e|e.to_string())?;
    let mut image=bitmap.as_image().into_rgb8();
    if grayscale{for pixel in image.pixels_mut(){let v=(pixel[0]as u32*299+pixel[1]as u32*587+pixel[2]as u32*114)/1000;*pixel=image::Rgb([v as u8,v as u8,v as u8]);}}
    let mut out=std::io::Cursor::new(Vec::new());image::DynamicImage::ImageRgb8(image).write_to(&mut out,image::ImageFormat::Png).map_err(|e|e.to_string())?;Ok(B64.encode(out.into_inner()))
}
pub fn text(bytes:&[u8])->Result<String,String>{let engine=engine()?;let doc=engine.load_pdf_from_byte_slice(bytes,None).map_err(|e|e.to_string())?;let mut result=String::new();for page in doc.pages().iter(){if let Ok(t)=page.text(){result.push_str(&t.all());result.push_str("\n\n");}}Ok(result)}

