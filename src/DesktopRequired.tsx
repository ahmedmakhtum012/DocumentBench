import React from 'react';
import {Brand} from './Icon';

export default function DesktopRequired(){
 return <main className="desktop-required"><section><Brand/><span className="desktop-required-kicker">DOCUMENTBENCH FOR WINDOWS</span><h1>This is a desktop application.</h1><p>The browser is showing only the interface build. PDF rendering, local files, recovery, and printing require the native Tauri application.</p><div className="desktop-required-steps"><b>Close this browser tab and open DocumentBench using:</b><code>launch-app.cmd</code><span>or</span><code>target\debug\documentbench.exe</code></div><small>No web version or browser PDF service is provided.</small></section></main>;
}
