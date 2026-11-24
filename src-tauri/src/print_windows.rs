use crate::types::PrintOptions;
use std::{ffi::OsString,os::windows::ffi::{OsStringExt,OsStrExt}};
use windows::{core::{PCWSTR,PWSTR},Win32::Graphics::Gdi::*,Win32::Graphics::Printing::*,Win32::Storage::Xps::{DOCINFOW,EndDoc,EndPage,StartDocW,StartPage}};

fn wide(s:&str)->Vec<u16>{std::ffi::OsStr::new(s).encode_wide().chain(Some(0)).collect()}
fn from_wide(p:*const u16)->String{if p.is_null(){return String::new();}let mut n=0;unsafe{while *p.add(n)!=0{n+=1;}OsString::from_wide(std::slice::from_raw_parts(p,n)).to_string_lossy().into_owned()}}

pub fn list()->Vec<String>{
    unsafe{
        let flags=PRINTER_ENUM_LOCAL|PRINTER_ENUM_CONNECTIONS;let mut needed=0;let mut count=0;
        let _=EnumPrintersW(flags,PCWSTR::null(),4,None,&mut needed,&mut count);
        if needed==0{return vec!["Microsoft Print to PDF".into()];}
        let mut buffer=vec![0u8;needed as usize];
        if EnumPrintersW(flags,PCWSTR::null(),4,Some(&mut buffer),&mut needed,&mut count).is_ok(){
            let records=std::slice::from_raw_parts(buffer.as_ptr() as *const PRINTER_INFO_4W,count as usize);
            let out:Vec<String>=records.iter().map(|r|from_wide(r.pPrinterName.0)).collect();if !out.is_empty(){return out;}
        }
        vec!["Microsoft Print to PDF".into()]
    }
}
pub fn default_printer()->String{
    unsafe{let mut needed=0;let _=GetDefaultPrinterW(None,&mut needed);if needed>0{let mut buffer=vec![0u16;needed as usize];if GetDefaultPrinterW(Some(PWSTR(buffer.as_mut_ptr())),&mut needed).as_bool(){return from_wide(buffer.as_ptr());}}"Microsoft Print to PDF".into()}
}

fn source_image(bytes:&[u8],page:u32,width:u32,gray:bool)->Result<image::RgbaImage,String>{
    let encoded=crate::render::render_png(bytes,page,width,gray)?;
    let png=base64::Engine::decode(&base64::engine::general_purpose::STANDARD,encoded).map_err(|e|e.to_string())?;
    Ok(image::load_from_memory(&png).map_err(|e|e.to_string())?.to_rgba8())
}

// Compose exactly what will be spooled, rather than relying on driver-specific N-up settings.
fn compose_sheets(bytes:&[u8],options:&PrintOptions,current:u32)->Result<Vec<image::RgbaImage>,String>{
    let document=lopdf::Document::load_mem(bytes).map_err(|e|e.to_string())?;
    let pages=document.get_pages();let total=pages.len() as u32;
    let selected=crate::pdf::selected_pages(options,total,current.max(1));
    if selected.is_empty(){return Err("The selected print range contains no pages.".into());}
    let mut groups:Vec<Vec<Option<u32>>>=if options.booklet{
        crate::pdf::booklet_sheets(&selected,options.booklet_binding=="right").into_iter().map(|(a,b)|vec![a,b]).collect()
    }else{
        let per=(options.rows.max(1)*options.columns.max(1))as usize;
        selected.chunks(per).map(|c|c.iter().map(|p|Some(*p)).collect()).collect()
    };
    if groups.is_empty(){return Err("There are no sheets to print.".into());}
    let first_size=selected.first().and_then(|p|pages.get(p)).map(|id|crate::pdf::page_size(&document,*id)).unwrap_or((612.,792.));
    let landscape=options.orientation=="landscape"||(options.orientation=="auto"&&first_size.0>first_size.1&&!options.booklet);
    let(sheet_w,sheet_h)=if landscape{(1650u32,1275u32)}else{(1275u32,1650u32)};
    let rows=if options.booklet{1}else{options.rows.max(1)};let columns=if options.booklet{2}else{options.columns.max(1)};
    let margin=32i32;let gutter=16i32;let cell_w=(sheet_w as i32-margin*2-gutter*(columns as i32-1))/columns as i32;let cell_h=(sheet_h as i32-margin*2-gutter*(rows as i32-1))/rows as i32;
    let mut sheets=Vec::new();
    for group in groups.drain(..){
        let mut sheet=image::RgbaImage::from_pixel(sheet_w,sheet_h,image::Rgba([255,255,255,255]));
        for(index,page)in group.into_iter().enumerate(){
            let Some(page)=page else{continue};
            let id=pages.get(&page).copied().ok_or("A selected page no longer exists.")?;
            let(pw,_)=crate::pdf::page_size(&document,id);
            let source=source_image(bytes,page,(pw as f32*2.).clamp(300.,1800.)as u32,options.grayscale)?;
            let fit=(cell_w as f32/source.width()as f32).min(cell_h as f32/source.height()as f32);
            let ratio=match options.scaling.as_str(){"actual"=>1.,"shrink"=>fit.min(1.),"custom"=>(options.scale as f32/100.).clamp(0.1,4.),_=>fit};
            let w=((source.width()as f32*ratio).round()as u32).max(1);let h=((source.height()as f32*ratio).round()as u32).max(1);
            let resized=image::imageops::resize(&source,w,h,image::imageops::FilterType::Lanczos3);
            let(row,column)=if options.order=="vertical"&&!options.booklet{(index%rows as usize,index/rows as usize)}else{(index/columns as usize,index%columns as usize)};
            if row>=rows as usize||column>=columns as usize{continue;}
            let cell_x=margin+column as i32*(cell_w+gutter);let cell_y=margin+row as i32*(cell_h+gutter);
            let x=cell_x+(cell_w-w as i32)/2;let y=cell_y+(cell_h-h as i32)/2;
            image::imageops::overlay(&mut sheet,&resized,x as i64,y as i64);
        }
        sheets.push(sheet);
    }
    Ok(sheets)
}

pub fn print_bytes(bytes:&[u8],options:&PrintOptions,doc_name:&str,current:u32)->Result<String,String>{
    let sheets=compose_sheets(bytes,options,current)?;
    let printer=if options.printer.is_empty(){default_printer()}else{options.printer.clone()};
    unsafe{
        let name=wide(&printer);let mut handle=PRINTER_HANDLE::default();
        OpenPrinterW(PCWSTR(name.as_ptr()),&mut handle,None).map_err(|e|format!("Cannot open printer {printer}: {e}"))?;
        let result=(||{
            let mut needed=0;let _=GetPrinterW(handle,2,None,&mut needed);if needed==0{return Err("The printer did not return a device configuration.".into());}
            let mut buffer=vec![0u8;needed as usize];GetPrinterW(handle,2,Some(&mut buffer),&mut needed).map_err(|e|e.to_string())?;
            let info=&mut*(buffer.as_mut_ptr()as*mut PRINTER_INFO_2W);if info.pDevMode.is_null(){return Err("The printer has no device configuration.".into());}
            let dm=&mut*info.pDevMode;dm.dmFields|=DM_ORIENTATION|DM_COPIES|DM_COLOR;
            dm.Anonymous1.Anonymous1.dmOrientation=if options.orientation=="landscape"{DMORIENT_LANDSCAPE as i16}else{DMORIENT_PORTRAIT as i16};dm.Anonymous1.Anonymous1.dmCopies=1;dm.dmColor=if options.grayscale{DMCOLOR_MONOCHROME}else{DMCOLOR_COLOR};
            let dc=CreateDCW(PCWSTR::null(),PCWSTR(name.as_ptr()),PCWSTR::null(),Some(dm));
            if dc.is_invalid(){return Err(format!("Failed to create printer device context for {printer}. No print preview generated."));}
            let job=wide(doc_name);let doc_info=DOCINFOW{cbSize:std::mem::size_of::<DOCINFOW>()as i32,lpszDocName:PCWSTR(job.as_ptr()),..Default::default()};
            if StartDocW(dc,&doc_info)<=0{let _=DeleteDC(dc);return Err("The printer could not start this document.".into());}
            let paper_w=GetDeviceCaps(Some(dc),HORZRES);let paper_h=GetDeviceCaps(Some(dc),VERTRES);let _=SetStretchBltMode(dc,COLORONCOLOR);
            let copies=options.copies.max(1);
            let mut order=Vec::new();if options.collate{for _ in 0..copies{order.extend(0..sheets.len());}}else{for index in 0..sheets.len(){for _ in 0..copies{order.push(index);}}}
            for index in order{
                if StartPage(dc)<=0{let _=EndDoc(dc);let _=DeleteDC(dc);return Err("The printer could not start a page.".into());}
                let sheet=&sheets[index];let mut bgr=Vec::with_capacity((sheet.width()*sheet.height()*4)as usize);
                for p in sheet.pixels(){bgr.extend_from_slice(&[p[2],p[1],p[0],255]);}
                let bitmap=BITMAPINFO{bmiHeader:BITMAPINFOHEADER{biSize:std::mem::size_of::<BITMAPINFOHEADER>()as u32,biWidth:sheet.width()as i32,biHeight:-(sheet.height()as i32),biPlanes:1,biBitCount:32,biCompression:BI_RGB.0,..Default::default()},..Default::default()};
                let written=StretchDIBits(dc,0,0,paper_w,paper_h,0,0,sheet.width()as i32,sheet.height()as i32,Some(bgr.as_ptr()as*const _),&bitmap,DIB_RGB_COLORS,SRCCOPY);
                if written==0{let _=EndDoc(dc);let _=DeleteDC(dc);return Err("Windows could not transfer a rendered page to the printer.".into());}
                if EndPage(dc)<=0{let _=EndDoc(dc);let _=DeleteDC(dc);return Err("The printer could not finish a page.".into());}
            }
            EndDoc(dc);let _=DeleteDC(dc);
            Ok(format!("Sent {} sheet(s) of {doc_name} to {printer}",sheets.len()*copies as usize))
        })();
        let _=ClosePrinter(handle);result
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    fn two_page_pdf()->Vec<u8>{
        let mut doc=lopdf::Document::with_version("1.7");let pages_id=doc.new_object_id();
        let kids:Vec<_>=(0..2).map(|_|{
            let content=doc.add_object(lopdf::Stream::new(lopdf::dictionary!{},b"0.2 0.4 0.8 rg 50 50 400 600 re f".to_vec()));
            let page=doc.add_object(lopdf::dictionary!{"Type"=>"Page","Parent"=>pages_id,"MediaBox"=>vec![0.into(),0.into(),612.into(),792.into()],"Contents"=>content});
            lopdf::Object::Reference(page)
        }).collect();
        doc.objects.insert(pages_id,lopdf::dictionary!{"Type"=>"Pages","Kids"=>kids,"Count"=>2}.into());
        let root=doc.add_object(lopdf::dictionary!{"Type"=>"Catalog","Pages"=>pages_id});doc.trailer.set("Root",root);
        let mut bytes=Vec::new();doc.save_to(&mut bytes).expect("test PDF should serialize");bytes
    }
    #[test]
    fn pdfium_composes_real_page_ranges_and_booklets(){
        let bytes=two_page_pdf();
        let options=PrintOptions{range:"custom".into(),custom_range:"2, 1".into(),..Default::default()};
        let sheets=compose_sheets(&bytes,&options,1).expect("PDFium should render composed sheets");
        assert_eq!(sheets.len(),2);assert!(sheets.iter().all(|s|s.width()==1275&&s.height()==1650));
        let booklet=compose_sheets(&bytes,&PrintOptions{booklet:true,..Default::default()},1).expect("booklet composition should render");
        assert_eq!(booklet.len(),2);assert!(booklet.iter().all(|s|s.width()==1275&&s.height()==1650));
    }
}
