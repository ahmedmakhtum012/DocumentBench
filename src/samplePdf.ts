import type { DocState } from './api';

// A small original demonstration PDF, used for the first-run preview and offline export.
// It deliberately contains no original application's text or artwork.
export function makeSamplePdf(doc:DocState):Uint8Array {
  const esc=(s:string)=>s.replace(/[\\()\r]/g,c=>c==='\r'?'':`\\${c}`).replace(/[^\x20-\x7e]/g,' ');
  const pages=Array.from({length:doc.pageCount},(_,i)=>{
    const lines=(i===0?(doc.text||'Project brief'):`Page ${i+1}\n\n${doc.text||'Project brief'}`).split('\n').slice(0,43);
    return `BT /F1 10 Tf 54 738 Td 14 TL ${lines.map((l,j)=>`${j===0?'':'T* '}(${esc(l)}) Tj`).join(' ')} ET`;
  });
  const objects:string[]=['<< /Type /Catalog /Pages 2 0 R >>',`<< /Type /Pages /Kids [${pages.map((_,i)=>`${5+i*2} 0 R`).join(' ')}] /Count ${pages.length} >>`,'<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>','<< /Title (NitroCraft document) /Creator (NitroCraft) >>'];
  pages.forEach((stream,i)=>{ objects.push(`<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 3 0 R >> >> /Contents ${6+i*2} 0 R >>`);objects.push(`<< /Length ${stream.length} >>\nstream\n${stream}\nendstream`); });
  let output='%PDF-1.7\n';const offsets=[0];objects.forEach((o,i)=>{offsets.push(output.length);output+=`${i+1} 0 obj\n${o}\nendobj\n`;});const xref=output.length;output+=`xref\n0 ${objects.length+1}\n0000000000 65535 f \n`;for(let i=1;i<offsets.length;i++)output+=`${offsets[i].toString().padStart(10,'0')} 00000 n \n`;output+=`trailer\n<< /Size ${objects.length+1} /Root 1 0 R /Info 4 0 R >>\nstartxref\n${xref}\n%%EOF`;
  return new TextEncoder().encode(output);
}
