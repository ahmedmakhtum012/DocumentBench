export const bytesToBase64=(bytes:Uint8Array)=>{let s='';for(let i=0;i<bytes.length;i+=0x8000)s+=String.fromCharCode(...bytes.subarray(i,i+0x8000));return btoa(s)};
export const base64ToBytes=(s:string)=>Uint8Array.from(atob(s),c=>c.charCodeAt(0));
export const download=(data:string,name:string,type='application/octet-stream')=>{const a=document.createElement('a');a.href=URL.createObjectURL(new Blob([base64ToBytes(data)],{type}));a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(a.href),5000)};
