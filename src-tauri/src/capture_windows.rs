use std::{ffi::c_void,path::Path};

type Hwnd=*mut c_void;type Hdc=*mut c_void;type Hbitmap=*mut c_void;type Hgdiobj=*mut c_void;
#[repr(C)]struct Rect{left:i32,top:i32,right:i32,bottom:i32}
#[repr(C)]#[derive(Default)]struct BitmapInfoHeader{size:u32,width:i32,height:i32,planes:u16,bit_count:u16,compression:u32,size_image:u32,x_pels_per_meter:i32,y_pels_per_meter:i32,clr_used:u32,clr_important:u32}
#[repr(C)]struct BitmapInfo{header:BitmapInfoHeader,colors:[u32;3]}
#[link(name="user32")]unsafe extern "system"{fn GetClientRect(hwnd:Hwnd,rect:*mut Rect)->i32;fn GetDC(hwnd:Hwnd)->Hdc;fn ReleaseDC(hwnd:Hwnd,dc:Hdc)->i32;fn PrintWindow(hwnd:Hwnd,dc:Hdc,flags:u32)->i32;}
#[link(name="gdi32")]unsafe extern "system"{fn CreateCompatibleDC(dc:Hdc)->Hdc;fn CreateCompatibleBitmap(dc:Hdc,width:i32,height:i32)->Hbitmap;fn SelectObject(dc:Hdc,object:Hgdiobj)->Hgdiobj;fn DeleteObject(object:Hgdiobj)->i32;fn DeleteDC(dc:Hdc)->i32;fn BitBlt(dest:Hdc,x:i32,y:i32,width:i32,height:i32,source:Hdc,x_src:i32,y_src:i32,rop:u32)->i32;fn GetDIBits(dc:Hdc,bitmap:Hbitmap,start:u32,lines:u32,bits:*mut c_void,info:*mut BitmapInfo,usage:u32)->i32;}

pub fn capture(window:&tauri::WebviewWindow,path:&str)->Result<(),String>{
    let hwnd=window.hwnd().map_err(|e|format!("Cannot access the native window: {e}"))?;
    unsafe{
        let mut rect=Rect{left:0,top:0,right:0,bottom:0};if GetClientRect(hwnd.0,&mut rect)==0{return Err("Cannot read the window dimensions.".into());}
        let width=rect.right-rect.left;let height=rect.bottom-rect.top;if width<=0||height<=0{return Err("The window is not visible.".into());}
        let window_dc=GetDC(hwnd.0);let memory_dc=CreateCompatibleDC(window_dc);let bitmap=CreateCompatibleBitmap(window_dc,width,height);let old=SelectObject(memory_dc,bitmap);
        // PrintWindow must run on the UI thread; a worker-thread call can deadlock WebView2.
        if PrintWindow(hwnd.0,memory_dc,2)==0{BitBlt(memory_dc,0,0,width,height,window_dc,0,0,0x00CC0020);}
        let mut info=BitmapInfo{header:BitmapInfoHeader{size:std::mem::size_of::<BitmapInfoHeader>()as u32,width,height,planes:1,bit_count:32,..Default::default()},colors:[0;3]};
        let mut bgr=vec![0u8;(width*height*4)as usize];
        let lines=GetDIBits(window_dc,bitmap,0,height as u32,bgr.as_mut_ptr()as*mut c_void,&mut info,0);
        SelectObject(memory_dc,old);DeleteObject(bitmap);DeleteDC(memory_dc);ReleaseDC(hwnd.0,window_dc);
        if lines==0{return Err("Windows returned no pixels for the window capture.".into());}
        let mut rgba=vec![0u8;bgr.len()];let stride=(width*4)as usize;
        for y in 0..height as usize{let source=(height as usize-1-y)*stride;let target=y*stride;for x in 0..width as usize{let s=source+x*4;let t=target+x*4;rgba[t]=bgr[s+2];rgba[t+1]=bgr[s+1];rgba[t+2]=bgr[s];rgba[t+3]=255;}}
        let image=image::RgbaImage::from_raw(width as u32,height as u32,rgba).ok_or("The captured window has invalid dimensions.")?;
        if let Some(parent)=Path::new(path).parent(){std::fs::create_dir_all(parent).map_err(|e|e.to_string())?;}
        image.save(path).map_err(|e|format!("Cannot save the screenshot: {e}"))?;Ok(())
    }
}
