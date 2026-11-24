use tauri::Manager;
mod types;mod pdf;mod render;mod extract;mod recovery;mod print_windows;mod capture_windows;
use types::*;
use base64::{engine::general_purpose::STANDARD as B64,Engine as _};
use std::{fs,path::PathBuf,time::Duration};

#[tauri::command]
async fn open_document(path:String)->Result<DocumentInfo,String>{
    tauri::async_runtime::spawn_blocking(move||{
        let bytes=fs::read(&path).map_err(|e|format!("Cannot open the document: {e}"))?;
        let doc=lopdf::Document::load_mem(&bytes).map_err(|e|format!("This PDF could not be opened: {e}"))?;
        let mut pages:Vec<_>=doc.get_pages().into_iter().collect();pages.sort_by_key(|p|p.0);
        let page_sizes=pages.iter().map(|(_,id)|{let(w,h)=pdf::page_size(&doc,*id);PageSize{width:w as f64,height:h as f64}}).collect();
        let text=render::text(&bytes).unwrap_or_default();let bookmarks=Vec::new();let name=PathBuf::from(&path).file_name().map(|s|s.to_string_lossy().to_string()).unwrap_or_else(||"Document.pdf".into());
        Ok(DocumentInfo{id:uuid::Uuid::new_v4().to_string(),name,path:Some(path),page_count:doc.get_pages().len()as u32,text,is_xfa:bytes.windows(4).any(|w|w==b"XFA "),encrypted:doc.is_encrypted(),file_size:bytes.len()as u64,page_sizes,bookmarks})
    }).await.map_err(|e|e.to_string())?
}
#[tauri::command]
#[allow(non_snake_case)]
async fn render_page(sourcePath:Option<String>,data:Option<String>,page:u32,width:u32,grayscale:bool)->Result<String,String>{
    tauri::async_runtime::spawn_blocking(move||{let bytes=if let Some(p)=sourcePath{fs::read(p).map_err(|e|e.to_string())?}else{B64.decode(data.ok_or("No PDF data was supplied.")?).map_err(|e|e.to_string())?};render::render_png(&bytes,page,width,grayscale)}).await.map_err(|e|e.to_string())?
}
#[tauri::command]
async fn save_document(path:String,request:SaveRequest)->Result<SaveResult,String>{
    tauri::async_runtime::spawn_blocking(move||{let bytes=pdf::serialize(&request)?;pdf::atomic_write(std::path::Path::new(&path),&bytes)?;recovery::clear(&request.id);Ok(SaveResult{path:path.clone(),name:PathBuf::from(path).file_name().map(|s|s.to_string_lossy().into()).unwrap_or_default(),size:bytes.len()as u64,data:B64.encode(&bytes)})}).await.map_err(|e|e.to_string())?
}
#[tauri::command]
async fn export_content(path:String,content:String,format:String)->Result<String,String>{tauri::async_runtime::spawn_blocking(move||{let bytes=if format=="base64"{B64.decode(content).map_err(|e|e.to_string())?}else{content.into_bytes()};fs::write(&path,bytes).map_err(|e|format!("Cannot export the file: {e}"))?;Ok(path)}).await.map_err(|e|e.to_string())?}
#[tauri::command]
async fn list_printers()->Result<Vec<String>,String>{tauri::async_runtime::spawn_blocking(move||Ok(print_windows::list())).await.map_err(|e|e.to_string())?}
#[tauri::command]
async fn print_document(request:SaveRequest,options:PrintOptions)->Result<String,String>{
    tauri::async_runtime::spawn_blocking(move||{
        let mut printable=request;
        match options.content.as_str(){
            "document"=>{printable.annotations.clear();printable.fields.clear();},
            "markups"=>printable.fields.clear(),
            "forms"=>printable.annotations.clear(),
            _=>{}
        }
        let bytes=pdf::serialize(&printable)?;
        print_windows::print_bytes(&bytes,&options,"DocumentBench document",options.current_page)
    }).await.map_err(|e|e.to_string())?
}
#[tauri::command]
async fn autosave_document(request:SaveRequest)->Result<recovery::RecoveryInfo,String>{
    tauri::async_runtime::spawn_blocking(move||{let bytes=pdf::serialize(&request)?;recovery::save(&request.id,request.source_path.as_deref().unwrap_or("Untitled.pdf"),&bytes)}).await.map_err(|e|e.to_string())?
}
#[tauri::command]
fn recovery_documents()->Vec<recovery::RecoveryInfo>{recovery::list()}
#[tauri::command]
async fn page_operation(request:SaveRequest,operation:String,from:Option<u32>,to:Option<u32>,pages:Option<Vec<u32>>,prefix:Option<String>,start:Option<i64>)->Result<String,String>{
    tauri::async_runtime::spawn_blocking(move||{let bytes=match operation.as_str(){"rotate-left"=>pdf::rotate_pages(&request,false),"rotate-right"=>pdf::rotate_pages(&request,true),"reorder"=>pdf::reorder_pages(&request,from.unwrap_or(1),to.unwrap_or(1)),"extract"=>pdf::extract_pages(&request,&pages.unwrap_or_default()),"delete"=>pdf::delete_pages(&request,&pages.unwrap_or_default()),"metadata"=>pdf::remove_metadata(&request),"redact"=>pdf::redact(&request),"bates"=>pdf::add_page_numbering(&request,prefix.unwrap_or_default(),start.unwrap_or(1)),_=>Err(format!("Unknown PDF operation: {operation}"))}?;Ok(B64.encode(bytes))}).await.map_err(|e|e.to_string())?
}
#[tauri::command]
async fn combine_documents(paths:Vec<String>)->Result<String,String>{tauri::async_runtime::spawn_blocking(move||{let bytes=pdf::merge(&paths)?;Ok(B64.encode(bytes))}).await.map_err(|e|e.to_string())?}
#[tauri::command]
async fn smart_extract(text:String,kind:String,target:Option<String>)->Result<serde_json::Value,String>{
    tauri::async_runtime::spawn_blocking(move||{match kind.as_str(){
        "table"|"form"=>{let rows=if kind=="table"{extract::table(&text)}else{text.lines().filter_map(|l|l.split_once(':').map(|(a,b)|vec![a.trim().into(),b.trim().into()])).collect::<Vec<Vec<String>>>()};let excel=extract::xlsx(&rows)?;Ok(serde_json::json!({"rows":rows,"data":B64.encode(excel),"message":format!("{} rows are ready to review.",rows.len())}))},
        "fields"=>Ok(serde_json::json!({"fields":extract::detect_fields(1,&text),"message":"Suggested fields are ready to review."})),
        "pii"=>Ok(serde_json::json!({"annotations":extract::detect_pii(1,&text),"message":"Potential personal information was found. Review every mark before applying redaction."})),
        "summarize"=>Ok(serde_json::json!({"message":extract::summarize(&text)})),
        "translate"=>Ok(serde_json::json!({"message":extract::translate_extract(&text,target.as_deref().unwrap_or("English"))})),
        _=>Err("This smart tool is not available.".into())}
    }).await.map_err(|e|e.to_string())?
}
#[tauri::command]
async fn export_excel(path:String,rows:Vec<Vec<String>>)->Result<String,String>{tauri::async_runtime::spawn_blocking(move||{let bytes=extract::xlsx(&rows)?;fs::write(&path,bytes).map_err(|e|e.to_string())?;Ok(path)}).await.map_err(|e|e.to_string())?}
#[tauri::command]
async fn ocr_document(request:SaveRequest,language:String)->Result<String,String>{
    tauri::async_runtime::spawn_blocking(move||{let _=request;Ok(format!("OCR ({language}) needs a language data installation. No text was replaced; configure Tesseract with the desired trained-data files before running a scanned-document OCR job."))}).await.map_err(|e|e.to_string())?
}
#[tauri::command]
fn capture_window(app:tauri::AppHandle,path:String)->Result<(),String>{
    for _ in 0..120{std::thread::sleep(Duration::from_millis(16));}
    let window=app.get_webview_window("main").ok_or("The main window is unavailable.")?;
    let _=window.set_focusable(false);
    let (sender,receiver)=std::sync::mpsc::channel();
    let capture_path=path.clone();
    app.run_on_main_thread(move||{let result=capture_windows::capture(&window,&capture_path);let _=sender.send(result);}).map_err(|e|e.to_string())?;
    receiver.recv().map_err(|e|e.to_string())??;
    std::thread::sleep(Duration::from_millis(250));app.exit(0);Ok(())
}

#[cfg_attr(mobile,tauri::mobile_entry_point)]
pub fn run(){
    tauri::Builder::default().plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![open_document,render_page,save_document,export_content,list_printers,print_document,autosave_document,recovery_documents,page_operation,combine_documents,smart_extract,export_excel,ocr_document,capture_window])
        .setup(|app|{
            let capture=std::env::args().collect::<Vec<_>>().windows(2).find(|p|p[0]=="--screenshot").map(|p|p[1].clone());
            if let Some(path)=capture{
                if let Some(window)=app.get_webview_window("main"){let _=window.set_focusable(false);let _=window.set_always_on_bottom(true);let _=window.show();}
                let handle=app.handle().clone();tauri::async_runtime::spawn(async move{let _=tauri::async_runtime::spawn_blocking(move||capture_window(handle,path)).await;});
            }
            Ok(())
        }).run(tauri::generate_context!()).expect("error while running DocumentBench");
}
