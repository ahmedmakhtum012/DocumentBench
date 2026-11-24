import type {IconName} from './Icon';
export type Tab='Home'|'Edit'|'Convert'|'Review'|'Forms'|'Protect'|'Share'|'Smart Tools'|'Format';
export type Action={label:string;icon:IconName;command:string;description?:string;large?:boolean;disabled?:boolean;menu?:{label:string;icon?:IconName;command:string;description?:string}[]};
export type Group={title:string;items:Action[]};
const zoomCommands=['25','50','75','100','125','150','200','400','800','1600'].map(v=>({label:`${v}%`,icon:'zoom' as IconName,command:`zoom-${v}`}));
export const tabNames:Tab[]=['Home','Edit','Convert','Review','Forms','Protect','Share','Smart Tools'];
export const ribbon:Record<Tab,Group[]>={
 Format:[
  {title:'Selection',items:[{label:'Properties',icon:'properties',command:'object-properties',large:true},{label:'Delete Object',icon:'delete',command:'delete-selection',large:true}]},
  {title:'Arrange',items:[{label:'Bring to Front',icon:'arrowUp',command:'bring-front'},{label:'Send to Back',icon:'arrowDown',command:'send-back'}]},
  {title:'Named Colors',items:[{label:'Color & Appearance',icon:'edit',command:'object-properties',large:true}]}
 ],
 Home:[
  {title:'Tools',items:[{label:'Hand',icon:'hand',command:'tool-hand',description:'Pan the document · Ctrl+H'},{label:'Select',icon:'select',command:'tool-select',description:'Select text and objects'},{label:'Type Text',icon:'text',command:'tool-text',description:'Add text anywhere · Ctrl+T'},{label:'Edit',icon:'edit',command:'tool-edit',description:'Edit page content · Ctrl+E'}]},
  {title:'Pages',items:[{label:'Insert Pages',icon:'pages',command:'insert-pages',large:true},{label:'Combine Files',icon:'combine',command:'combine',large:true},{label:'Split Document',icon:'split',command:'split',large:true},{label:'Rotate Left',icon:'rotateLeft',command:'rotate-left'},{label:'Rotate Right',icon:'rotateRight',command:'rotate-right'},{label:'Extract Pages',icon:'extract',command:'extract'},{label:'Delete Pages',icon:'delete',command:'delete-pages'}]},
  {title:'Create',items:[{label:'From File',icon:'document',command:'create-file',large:true},{label:'From Scanner',icon:'scan',command:'scan',large:true},{label:'Blank PDF',icon:'plus',command:'blank',large:true}]},
  {title:'View',items:[{label:'Fit Page',icon:'fit',command:'fit-page'},{label:'Fit Width',icon:'zoom',command:'fit-width'},{label:'Actual Size',icon:'document',command:'zoom-100'},{label:'Zoom',icon:'search',command:'zoom-menu',menu:zoomCommands}]},
  {title:'Tools',items:[{label:'Find Text',icon:'search',command:'find',description:'Search this document · Ctrl+F'},{label:'Snapshot',icon:'image',command:'snapshot'},{label:'Bookmarks',icon:'bookmark',command:'add-bookmark'}]}
 ],
 Edit:[
  {title:'Edit Content',items:[{label:'Edit Text & Images',icon:'edit',command:'tool-edit',large:true},{label:'Add Text',icon:'text',command:'tool-text',large:true},{label:'Add Image',icon:'image',command:'add-image',large:true},{label:'Erase',icon:'eraser',command:'erase'}]},
  {title:'Page Layout',items:[{label:'Insert Pages',icon:'pages',command:'insert-pages'},{label:'Replace Pages',icon:'refresh',command:'replace-pages'},{label:'Crop Pages',icon:'split',command:'crop'},{label:'Resize Pages',icon:'fit',command:'resize'}]},
  {title:'Page Design',items:[{label:'Watermark',icon:'watermark',command:'watermark',large:true},{label:'Header & Footer',icon:'header',command:'header-footer',large:true},{label:'Bates Numbering',icon:'number',command:'bates',large:true},{label:'Background',icon:'image',command:'background'},{label:'Optimize PDF',icon:'compress',command:'optimize'}]},
  {title:'Organize',items:[{label:'Page Thumbnails',icon:'pages',command:'sidebar-pages'},{label:'Rotate',icon:'rotateRight',command:'rotate-right'},{label:'Extract',icon:'extract',command:'extract'},{label:'Delete',icon:'delete',command:'delete-pages'}]}
 ],
 Convert:[
  {title:'Convert PDF',items:[{label:'To Word',icon:'word',command:'convert-word',large:true},{label:'To Excel',icon:'excel',command:'convert-excel',large:true},{label:'To PowerPoint',icon:'powerpoint',command:'convert-powerpoint',large:true},{label:'To Image',icon:'image',command:'convert-image'},{label:'To PDF/A',icon:'shield',command:'convert-pdfa'},{label:'To Plain Text',icon:'text',command:'convert-text'}]},
  {title:'Create PDF',items:[{label:'From File',icon:'document',command:'create-file',large:true},{label:'Combine Files',icon:'combine',command:'combine',large:true},{label:'Batch Process',icon:'checklist',command:'batch',large:true}]},
  {title:'OCR',items:[{label:'Make Searchable',icon:'scan',command:'ocr',large:true},{label:'OCR Language',icon:'translate',command:'ocr-language'},{label:'OCR Options',icon:'settings',command:'ocr-options'}]},
  {title:'Export',items:[{label:'Extract Text',icon:'download',command:'convert-text'},{label:'Extract Images',icon:'image',command:'extract-images'},{label:'Conversion Settings',icon:'settings',command:'conversion-settings'}]}
 ],
 Review:[
  {title:'Comments',items:[{label:'Add Note',icon:'note',command:'tool-note',large:true},{label:'Type Text',icon:'text',command:'tool-text',large:true},{label:'Call Out',icon:'comment',command:'callout',large:true},{label:'Text Box',icon:'form',command:'text-box'}]},
  {title:'Markup',items:[{label:'Highlight',icon:'highlight',command:'tool-highlight'},{label:'Underline',icon:'underline',command:'tool-underline'},{label:'Strikethrough',icon:'strike',command:'tool-strikeout'},{label:'Draw',icon:'draw',command:'tool-draw'},{label:'Shapes',icon:'form',command:'tool-rectangle'}]},
  {title:'Stamps',items:[{label:'Stamps',icon:'stamp',command:'stamps',large:true},{label:'Create Stamp',icon:'plus',command:'create-stamp'}]},
  {title:'Compare',items:[{label:'Compare Documents',icon:'compare',command:'compare',large:true},{label:'Synchronize Scrolling',icon:'link',command:'sync-scroll'}]},
  {title:'Review',items:[{label:'Comments Pane',icon:'comment',command:'sidebar-comments'},{label:'Spelling',icon:'checklist',command:'spelling'},{label:'Measure',icon:'fit',command:'measure'}]}
 ],
 Forms:[
  {title:'Form Fields',items:[{label:'Text Field',icon:'form',command:'tool-field',large:true},{label:'Checkbox',icon:'checkbox',command:'tool-checkbox',large:true},{label:'Signature Field',icon:'signature',command:'tool-signature-field',large:true},{label:'Button',icon:'plus',command:'form-button'},{label:'Dropdown',icon:'chevron',command:'form-dropdown'},{label:'Radio Button',icon:'check',command:'form-radio'}]},
  {title:'Create',items:[{label:'Auto Create Fields',icon:'magicForm',command:'form-create',large:true},{label:'Form Templates',icon:'document',command:'form-templates',large:true}]},
  {title:'Form Data',items:[{label:'Extract to Excel',icon:'excel',command:'form-extract',large:true},{label:'Import Data',icon:'upload',command:'import-form-data'},{label:'Export Data',icon:'download',command:'form-extract'},{label:'Reset Fields',icon:'refresh',command:'reset-fields'}]},
  {title:'Properties',items:[{label:'Calculate Fields',icon:'settings',command:'calculate'},{label:'Set Tab Order',icon:'list',command:'tab-order'},{label:'Highlight Fields',icon:'highlight',command:'highlight-fields'}]}
 ],
 Protect:[
  {title:'Redaction',items:[{label:'Mark for Redaction',icon:'redact',command:'tool-redaction',large:true},{label:'Search and Redact',icon:'search',command:'search-redact',large:true},{label:'Apply All Redactions',icon:'check',command:'apply-redactions',large:true},{label:'Repeat Mark',icon:'copy',command:'repeat-redaction'}]},
  {title:'Document Security',items:[{label:'Password Security',icon:'lock',command:'password',large:true},{label:'Permissions',icon:'shield',command:'permissions',large:true},{label:'Digital Signatures',icon:'signature',command:'signatures',large:true}]},
  {title:'Clean Document',items:[{label:'Remove Metadata',icon:'eraser',command:'remove-metadata',large:true},{label:'Sanitize Document',icon:'shield',command:'sanitize',large:true},{label:'Whiteout',icon:'eraser',command:'whiteout'}]},
  {title:'Certificates',items:[{label:'Trusted Identities',icon:'person',command:'identities'},{label:'Security Settings',icon:'settings',command:'security-settings'}]}
 ],
 Share:[
  {title:'Send',items:[{label:'Email Document',icon:'mail',command:'email',large:true},{label:'Share a Link',icon:'link',command:'share-link',large:true},{label:'Share for Review',icon:'share',command:'share-review',large:true}]},
  {title:'Cloud',items:[{label:'Open from Cloud',icon:'cloud',command:'cloud-open'},{label:'Save to Cloud',icon:'cloud',command:'cloud-save'},{label:'Cloud Accounts',icon:'settings',command:'cloud-accounts'}]},
  {title:'Collaborate',items:[{label:'Track Reviews',icon:'comment',command:'track-review',large:true},{label:'Review Settings',icon:'settings',command:'review-settings',large:true}]},
  {title:'Export',items:[{label:'Save a Copy',icon:'save',command:'save-as'},{label:'Print',icon:'print',command:'print'}]}
 ],
 'Smart Tools':[
  {title:'Extract',items:[{label:'Form Extract',icon:'form',command:'form-extract',large:true,description:'Extract form data to Excel'},{label:'Table Extract',icon:'table',command:'table-extract',large:true,description:'Detect and export tables to Excel'}]},
  {title:'Forms',items:[{label:'Form Create',icon:'magicForm',command:'form-create',large:true,description:'Detect likely form fields'}]},
  {title:'Protect',items:[{label:'Smart Redact',icon:'sparkle',command:'smart-redact',large:true,description:'Find personal information for review'}]},
  {title:'Document Assistant',items:[{label:'Document Assistant',icon:'sparkle',command:'assistant',large:true},{label:'Summarize',icon:'document',command:'summarize'},{label:'Translate',icon:'translate',command:'translate'},{label:'Search with Assistant',icon:'search',command:'assistant-search'}]},
  {title:'About Smart Tools',items:[{label:'How it works',icon:'help',command:'smart-help'},{label:'Privacy',icon:'shield',command:'privacy'}]}
 ]
};
export const stamps=[{label:'APPROVED',kind:'Dynamic',color:'#218756'},{label:'REVIEWED',kind:'Sign here',color:'#5d3ba8'},{label:'CONFIDENTIAL',kind:'Standard business',color:'#cf3f5b'},{label:'DRAFT',kind:'Standard business',color:'#b87414'},{label:'FINAL',kind:'Standard business',color:'#218756'},{label:'VOID',kind:'Standard business',color:'#cf3f5b'},{label:'COPY',kind:'Standard business',color:'#64748b'},{label:'FOR COMMENT',kind:'Reviews',color:'#4475b9'},{label:'PRELIMINARY',kind:'Reviews',color:'#b87414'},{label:'SIGN HERE',kind:'Sign here',color:'#5d3ba8'}];
