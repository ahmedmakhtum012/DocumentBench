use regex::Regex;
use crate::types::{Annotation,FormField};
use uuid::Uuid;

pub fn detect_fields(page:u32,text:&str)->Vec<FormField>{
    let mut fields=Vec::new();let labels=["Name","Date","Email","Phone","Approved by","Signature","Address"];
    for(i,line)in text.lines().enumerate(){for label in labels{
        if line.to_lowercase().starts_with(&label.to_lowercase())&&line.len()<40{
            fields.push(FormField{id:Uuid::new_v4().to_string(),page,name:label.into(),kind:if label=="Signature"{"signature".into()}else{"text".into()},x:0.13,y:(0.25+i as f64*0.009).min(0.83),width:0.44,height:0.032,value:String::new(),required:false});break;
        }
    }}fields
}
pub fn detect_pii(page:u32,text:&str)->Vec<Annotation>{
    let patterns=[("Email address",r"(?i)[\w.+-]+@[\w-]+\.[\w.-]+"),("Telephone",r"(?:\+\d{1,3}[ .-]?)?\(?\d{3}\)?[ .-]\d{3}[ .-]\d{4}"),("Payment card",r"\b(?:\d[ -]*?){13,16}\b")];
    let mut out=Vec::new();
    for(label,p)in patterns{let re=Regex::new(p).unwrap();for m in re.find_iter(text){
        let line=text[..m.start()].lines().count();
        out.push(Annotation{id:Uuid::new_v4().to_string(),page,kind:"redaction".into(),x:0.12,y:(0.17+line as f64*0.018).min(0.88),width:(m.as_str().len()as f64*0.006).clamp(0.08,0.72),height:0.022,text:format!("{label}: {}",m.as_str()),color:"#e14b65".into(),applied:false,stamp:None});
    }}out
}
pub fn table(text:&str)->Vec<Vec<String>>{text.lines().filter_map(|line|{let s=line.trim();if s.is_empty(){None}else{let cols:Vec<String>=s.split(|c:char|c=='|'||c=='\t').map(|v|v.trim().to_string()).filter(|v|!v.is_empty()).collect();if cols.len()>1{Some(cols)}else{let spaced:Vec<String>=s.split("  ").map(|v|v.trim().to_string()).filter(|v|!v.is_empty()).collect();if spaced.len()>1{Some(spaced)}else{None}}}}).take(250).collect()}
pub fn summarize(text:&str)->String{
    let sentences:Vec<&str>=text.split(|c|c=='.'||c=='!').map(str::trim).filter(|s|s.len()>45).take(4).collect();if sentences.is_empty(){return "This document has too little extractable text for a useful summary. If it is scanned, run OCR first.".into();}format!("Document summary\n\n{}\n\nThis is a local extractive summary. No document content was sent to an external service.",sentences.iter().map(|s|format!("• {}.",s)).collect::<Vec<_>>().join("\n"))
}
pub fn translate_extract(text:&str,target:&str)->String{format!("Translation workspace — {target}\n\nA cloud language model is not configured. The extracted source text is ready below; no machine-generated translation is being presented as authentic.\n\n{}",text.chars().take(12000).collect::<String>())}
pub fn xlsx(rows:&[Vec<String>])->Result<Vec<u8>,String>{
    let sheet=rows.iter().enumerate().map(|(i,r)|format!("<row r=\"{}\">{}</row>",i+1,r.iter().enumerate().map(|(j,c)|{let mut n=j;let mut name=String::new();loop{name.insert(0,(b'A'+(n%26)as u8)as char);if n<26{break;}n=n/26-1;}format!("<c r=\"{}{}\" t=\"inlineStr\"><is><t xml:space=\"preserve\">{}</t></is></c>",name,i+1,quick_xml::escape::escape(c))}).collect::<String>())).collect::<String>();
    let entries:[(&str,String);6]=[
        ("[Content_Types].xml",r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/></Types>"#.into()),
        ("_rels/.rels",r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/></Relationships>"#.into()),
        ("xl/workbook.xml",r#"<?xml version="1.0" encoding="UTF-8"?><workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Extracted data" sheetId="1" r:id="rId1"/></sheets></workbook>"#.into()),
        ("xl/_rels/workbook.xml.rels",r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/></Relationships>"#.into()),
        ("xl/worksheets/sheet1.xml",format!(r#"<?xml version="1.0" encoding="UTF-8"?><worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>{}</sheetData></worksheet>"#,sheet)),
        ("docProps/app.xml",r#"<?xml version="1.0" encoding="UTF-8"?><Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"><Application>NitroCraft</Application></Properties>"#.into())];
    let cursor=std::io::Cursor::new(Vec::new());let mut zip=zip::ZipWriter::new(cursor);let options=zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for (name,content)in entries{zip.start_file(name,options).map_err(|e|e.to_string())?;use std::io::Write;zip.write_all(content.as_bytes()).map_err(|e|e.to_string())?;}Ok(zip.finish().map_err(|e|e.to_string())?.into_inner())
}
