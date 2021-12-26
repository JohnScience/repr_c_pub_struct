//! This is a library that is meant to help exporting Rust code to other languages by providing
//! a [`parse_for_repr_c_pub_structs`] function. This function accepts a path to the crate root and
//! recursively traverses all directories in `src/**` returning [Vec]<[ParsedFile]>.
//!
//! # Hierarchy of structs
//!
//! <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="856px" viewBox="-0.5 -0.5 856 429" content="&lt;mxfile host=&quot;app.diagrams.net&quot; modified=&quot;2021-12-26T09:24:42.949Z&quot; agent=&quot;5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36&quot; etag=&quot;hi3LxXkbLtGmmD42Csz3&quot; version=&quot;16.1.0&quot;&gt;&lt;diagram id=&quot;6bblCNeE5JDsqI-cM0kn&quot; name=&quot;Page-1&quot;&gt;7Vtbb6M4FP41SLMPRYC55XGSdmY0u5Wq7Ug7faoIOAkaLlnjtMn8+rUD5mKTQBKgNFupUu1jxzbf+Xw459hIYBZuvyJnvbqPPRhImuJtJXAraZqqGhb5RyW7VGICLRUske9lnQrBo/8bZkIlk258DyaVjjiOA+yvq0I3jiLo4orMQSh+rXZbxEF11rWzhILg0XUCUfqP7+FVKrUNpZB/g/5yxWZWlawldFjnTJCsHC9+LYnAnQRmKI5xWgq3MxhQ8Bgu6e++HGjNF4ZghNv84H7+7cf9d3/tPMHft3EY//jz7tdNNsqLE2yyB84Wi3cMARRvIg/SQRQJTF9XPoaPa8elra9E50S2wmFAaiopiotiM0CE4bYkyhb5FcYhxGhHumStGkMwY4xmZvXXAn+VgboqYc/6OZnKl/nQBSqkkAFzAkiaAFKCXAEn8ni4CkaCUfwLzuIgRkQSxRHpOV34QcCJnMBfRqTqEtAgkU8pWD7h4OesIfQ9j05Ti35VP10oAHAKsAxBAWYN/qAv/PXxkVSvQgTqOApqMLL6wsgQMAodP5JRcr08ZbRkOrBFHQxKU3t8NOVt6dvzdCKAFPjzq6apBkbGU3WEL31gjY2oqvjWJ1yDkRPCq6arroyNrqCZrjDyPlNnn8IYOEniu1VdVFEi4KDdT1qRDVZ9Krfdbiu1HattffyzVH7KRqfl4ie0smvSRxJvkAuPPXXaDztoCXGzbwS9ShgjarekPaNGe0yGYOBg/6Ua/NSpNJvhIfbJk+XkMVRuG6scK9Lnzn5VDla4gXh7IDj3KTDCQHuG5Y99Aela+JwDky792UhoZ4+KdkC7FtqJbvwb004dEemY4/LBuo5ZZzWzbmiH0JhUMVFFL6RWt2DSmx9SF9+ZAfX6PP+FFJd4/+ipKFk7USEDkjGN4ufQiZYEUOOW9SILqXbMxTVDNsyy3szp3tgSJ5LICArmvxuaW5zOiiKRL2jjYhO5z9SVJeVPsiz/Qf7fkG5OSDWZTaGQBjqQNe1juVQZ9C+dpYfxrVNR7tyjD+ACv7k/b3Iu2aRmI9k1G6m3bKoqpgA+9tHHPnpv+yh/+7zZRmLux5he2zrvE7U8u8mF3aMk5nEYceeMs3/5EaXjJozuIi8p8XpesLq8f+eotAfSvR7Nk3XWTxQlxIHDzwGZRdJmlw3i7pd5wTAkfLh0JXSIY+uwbq/VCOjc0RgQT8ZUtY7efGjQHb27T1TkCQcTmFIR/SkyyOv1ESCtPEDkkyejicxq/HmjyIpiSeUQVFbzkPRQFEpr/JAH9dgYaDJsmiPNceU3+LSadm6kaSp2rTM4UKCpdZ/eOEhWY3I8XdFIVsKBClltG3ywtQ1bNS6dofIeUOu8COdMCFa0b7qazXR1N+hlz8Y9285MzUknJOYO8um90cTgtGufa9NYQHmaTSOKcnalbmvaITmyXP6EUmlYVW3+qmBoOn+3fG2Rx+uEr8Q46mXOqrKqWp0ax3dHZl7dfDzTls26wQ3UE51NbvexmyL9ElRMof4vztRN2+DAlviwYdAjdU1MwfEh8CV5BC5k6wBAflfUHQfkN2/LEE6MniBkujlma5OVs6bFRQC3mZ3txkWQm+KmC+zqZFx2lVf8ub7kuXa1K9MHRnjpir/om++g8q6y6nZVX4aJfRxwLFn3N1yj2cNm/ojRxsUHsnXX+ibhLx6rdQnW2nPR3lTW45WF04xekQuw9cq1BVmhkJ6YCziorMa4HbS1tdq4bK3Of3ihv1db2yJuH56SVomS+/zS6fmpITipfHCyF04OFptrF9/zYmP0cNVLt9vycFzxvcbF9/y187GF90LqtF226mA7n9IdIFmgd/09zSKOcPaBqd2NN8Z/2VFzGqgP6T7rh6/W5L7xg4MS6H3xCYTvx3MuqW6feexAd2ZVd6aou8mQbjSzC935LDxml/ow1TfEm78gmEczkheEyh/RnpuoyE/LDp31nu2okGrxtXnavfhmH9z9Bw==&lt;/diagram&gt;&lt;/mxfile&gt;" onclick="(function(svg){var src=window.event.target||window.event.srcElement;while (src!=null&amp;&amp;src.nodeName.toLowerCase()!='a'){src=src.parentNode;}if(src==null){if(svg.wnd!=null&amp;&amp;!svg.wnd.closed){svg.wnd.focus();}else{var r=function(evt){if(evt.data=='ready'&amp;&amp;evt.source==svg.wnd){svg.wnd.postMessage(decodeURIComponent(svg.getAttribute('content')),'*');window.removeEventListener('message',r);}};window.addEventListener('message',r);svg.wnd=window.open('https://viewer.diagrams.net/?client=1&amp;page=0&amp;edit=_blank');}}})(this);" style="cursor:pointer;max-width:100%;max-height:429px;"><defs/><g><rect x="165" y="258" width="120" height="60" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/><rect x="195" y="273" width="60" height="30" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 58px; height: 1px; padding-top: 288px; margin-left: 196px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;">src</div></div></div></foreignObject><text x="225" y="292" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px" text-anchor="middle">src</text></switch></g><rect x="5" y="358" width="130" height="70" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/><rect x="40" y="378" width="60" height="30" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 58px; height: 1px; padding-top: 393px; margin-left: 41px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;">main.rs</div></div></div></foreignObject><text x="70" y="397" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px" text-anchor="middle">main.rs</text></switch></g><rect x="165" y="358" width="130" height="70" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/><rect x="200" y="378" width="60" height="30" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 58px; height: 1px; padding-top: 393px; margin-left: 201px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;">lib.rs</div></div></div></foreignObject><text x="230" y="397" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px" text-anchor="middle">lib.rs</text></switch></g><rect x="335" y="358" width="130" height="70" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/><rect x="370" y="378" width="60" height="30" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 58px; height: 1px; padding-top: 393px; margin-left: 371px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;">filename.rs</div></div></div></foreignObject><text x="400" y="397" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px" text-anchor="middle">filename.rs</text></switch></g><path d="M 165 318 L 75.87 355.53" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 71.03 357.57 L 76.12 351.62 L 75.87 355.53 L 78.84 358.08 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><path d="M 225 318 L 229.21 351.68" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 229.86 356.89 L 225.52 350.38 L 229.21 351.68 L 232.47 349.51 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><path d="M 285 318 L 393.99 355.91" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 398.94 357.63 L 391.18 358.64 L 393.99 355.91 L 393.48 352.03 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><rect x="555" y="8" width="250" height="390" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/><rect x="575" y="88" width="280" height="60" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe flex-start; width: 278px; height: 1px; padding-top: 118px; margin-left: 577px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: left;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"><div><span>#[no_mangle]</span></div><div><span>pub extern "C" fn func_name (...) -&gt; ... {</span></div><div><span>// ...</span></div><div><span>}</span></div></div></div></div></foreignObject><text x="577" y="122" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px">#[no_mangle]...</text></switch></g><rect x="575" y="248" width="280" height="60" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe flex-start; width: 278px; height: 1px; padding-top: 278px; margin-left: 577px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: left;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"><div><span>#[no_mangle]</span></div><div><span>pub extern "C" fn func_name (...) -&gt; ... {</span></div><div><span>// ...</span></div><div><span>}</span></div></div></div></div></foreignObject><text x="577" y="282" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px">#[no_mangle]...</text></switch></g><rect x="385" y="28" width="120" height="120" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/><rect x="395" y="33" width="110" height="110" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe flex-start; width: 108px; height: 1px; padding-top: 88px; margin-left: 397px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: left;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"><b>LineColumnEnds</b> {<br />    start_line,<br />    start_column,<br />    end_line,<br />    end_column,<br />}</div></div></div></foreignObject><text x="397" y="92" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px">LineColumnEnds {...</text></switch></g><path d="M 464.96 69.96 L 566.86 95.45" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 571.96 96.73 L 564.32 98.43 L 566.86 95.45 L 566.01 91.63 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><path d="M 464.96 98.01 L 564.07 138.57" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 568.93 140.56 L 561.12 141.14 L 564.07 138.57 L 563.77 134.67 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><path d="M 485 78 Q 535 28 555 18 Q 575 8 575 81.63" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 575 86.88 L 571.5 79.88 L 575 81.63 L 578.5 79.88 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><path d="M 475 118 Q 585 198 579.81 161.33" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 579.08 156.13 L 583.52 162.57 L 579.81 161.33 L 576.59 163.55 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><rect x="650" y="18" width="60" height="30" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 58px; height: 1px; padding-top: 33px; margin-left: 651px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;">filename.rs</div></div></div></foreignObject><text x="680" y="37" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px" text-anchor="middle">filename.rs</text></switch></g><rect x="415" y="208" width="100" height="95" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe flex-start; width: 98px; height: 1px; padding-top: 256px; margin-left: 417px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: left;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;">LineColumnEnds</div></div></div></foreignObject><text x="417" y="259" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px">LineColumnEnds</text></switch></g><path d="M 513.89 262.9 L 517.06 253.41 L 558.08 267.09 L 561.4 257.13 L 574.53 277.84 L 551.6 286.54 L 554.92 276.58 Z" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><rect x="195" y="98" width="170" height="90" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/><rect x="205" y="128" width="150" height="30" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 148px; height: 1px; padding-top: 143px; margin-left: 206px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 12px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"><b>ReprCPubStructs</b></div></div></div></foreignObject><text x="280" y="147" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="12px" text-anchor="middle">ReprCPubStructs</text></switch></g><path d="M 337.97 189.98 L 410.15 251.37" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 414.15 254.78 L 406.55 252.91 L 410.15 251.37 L 411.08 247.57 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><path d="M 338.99 97.01 L 378.75 89.22" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 383.9 88.21 L 377.71 92.99 L 378.75 89.22 L 376.36 86.13 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><path d="M 140 55.5 Q 285 8 430 3 Q 575 -2 605 3 Q 635 8 658.96 15.99" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 663.94 17.65 L 656.19 18.75 L 658.96 15.99 L 658.41 12.11 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/><rect x="0" y="33" width="140" height="90" fill="rgb(255, 255, 255)" stroke="rgb(0, 0, 0)" pointer-events="all"/><rect x="30" y="63" width="90" height="30" fill="none" stroke="none" pointer-events="all"/><g transform="translate(-0.5 -0.5)"><switch><foreignObject pointer-events="none" width="100%" height="100%" requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility" style="overflow: visible; text-align: left;"><div xmlns="http://www.w3.org/1999/xhtml" style="display: flex; align-items: unsafe center; justify-content: unsafe center; width: 88px; height: 1px; padding-top: 78px; margin-left: 31px;"><div data-drawio-colors="color: rgb(0, 0, 0); " style="box-sizing: border-box; font-size: 0px; text-align: center;"><div style="display: inline-block; font-size: 14px; font-family: Helvetica; color: rgb(0, 0, 0); line-height: 1.2; pointer-events: all; white-space: normal; overflow-wrap: normal;"><b>ParsedFile</b></div></div></div></foreignObject><text x="75" y="82" fill="rgb(0, 0, 0)" font-family="Helvetica" font-size="14px" text-anchor="middle">ParsedFile</text></switch></g><path d="M 140 78 L 190.89 138.14" fill="none" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="stroke"/><path d="M 194.28 142.15 L 187.08 139.06 L 190.89 138.14 L 192.43 134.54 Z" fill="rgb(0, 0, 0)" stroke="rgb(0, 0, 0)" stroke-miterlimit="10" pointer-events="all"/></g><switch><g requiredFeatures="http://www.w3.org/TR/SVG11/feature#Extensibility"/><a transform="translate(0,-5)" xlink:href="https://www.diagrams.net/doc/faq/svg-export-text-problems" target="_blank"><text text-anchor="middle" font-size="10px" x="50%" y="100%">Viewer does not support full SVG 1.1</text></a></switch></svg>
//!
//! # Example
//!
//! `src/main.rs`
//!
//! ```rust
//! use repr_c_pub_struct::parse_for_repr_c_pub_structs;
//!
//! fn main() {
//!     let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
//!     let repr_c_structs = parse_for_repr_c_pub_structs(crate_root.as_str());
//!     println!("{:#?}", repr_c_structs);
//! }
//! ```
//!
//! `src/unused.rs`
//!
//! ```rust
//! #[repr(C)]
//! pub struct Unused {
//!    // Test
//! }
//! ```
//!
//! # Output on Windows
//!
//! ```text
//! [
//!     ParsedFile {
//!         path: "...\\repr_c_pub_struct\\src\\lib.rs",
//!         repr_c_pub_structs: ReprCPubStructs(
//!             [],
//!         ),
//!     },
//!     ParsedFile {
//!         path: "...\\repr_c_pub_struct\\src\\main.rs",
//!         repr_c_pub_structs: ReprCPubStructs(
//!             [],
//!         ),
//!     },
//!     ParsedFile {
//!         path: "...\\repr_c_pub_struct\\src\\unused.rs",
//!         repr_c_pub_structs: ReprCPubStructs(
//!             [
//!                 LineColumnEnds {
//!                     start_line: 1,
//!                     start_column: 0,
//!                     end_line: 4,
//!                     end_column: 1,
//!                 },
//!             ],
//!         ),
//!     },
//! ]
//! ```
//!
//! # Note
//! The paths will be absolute. The prefixes have been deleted intentionally.
//!
//! # Integration with serde
//!
//! [Serde][serde] is an amazing framework for serializing and deserializing Rust data structures efficiently
//! and generically. The following is a partial list of data formats that have been implemented for Serde by the community.
//!
//! - [JSON], the ubiquitous JavaScript Object Notation used by many HTTP APIs.
//! - [Bincode], a compact binary format
//!   used for IPC within the Servo rendering engine.
//! - [CBOR], a Concise Binary Object Representation designed for small message
//!   size without the need for version negotiation.
//! - [YAML], a self-proclaimed human-friendly configuration language that ain't
//!   markup language.
//! - [MessagePack], an efficient binary format that resembles a compact JSON.
//! - [TOML], a minimal configuration format used by [Cargo].
//! - [Pickle], a format common in the Python world.
//! - [RON], a Rusty Object Notation.
//! - [BSON], the data storage and network transfer format used by MongoDB.
//! - [Avro], a binary format used within Apache Hadoop, with support for schema
//!   definition.
//! - [JSON5], a superset of JSON including some productions from ES5.
//! - [Postcard], a no\_std and embedded-systems friendly compact binary format.
//! - [URL] query strings, in the x-www-form-urlencoded format.
//! - [Envy], a way to deserialize environment variables into Rust structs.
//!   *(deserialization only)*
//! - [Envy Store], a way to deserialize [AWS Parameter Store] parameters into
//!   Rust structs. *(deserialization only)*
//! - [S-expressions], the textual representation of code and data used by the
//!   Lisp language family.
//! - [D-Bus]'s binary wire format.
//! - [FlexBuffers], the schemaless cousin of Google's FlatBuffers zero-copy serialization format.
//! - [DynamoDB Items], the format used by [rusoto_dynamodb] to transfer data to
//!   and from DynamoDB.
//!
//! [JSON]: https://github.com/serde-rs/json
//! [Bincode]: https://github.com/servo/bincode
//! [CBOR]: https://github.com/enarx/ciborium
//! [YAML]: https://github.com/dtolnay/serde-yaml
//! [MessagePack]: https://github.com/3Hren/msgpack-rust
//! [TOML]: https://github.com/alexcrichton/toml-rs
//! [Pickle]: https://github.com/birkenfeld/serde-pickle
//! [RON]: https://github.com/ron-rs/ron
//! [BSON]: https://github.com/zonyitoo/bson-rs
//! [Avro]: https://github.com/flavray/avro-rs
//! [JSON5]: https://github.com/callum-oakley/json5-rs
//! [Postcard]: https://github.com/jamesmunns/postcard
//! [URL]: https://docs.rs/serde_qs
//! [Envy]: https://github.com/softprops/envy
//! [Envy Store]: https://github.com/softprops/envy-store
//! [Cargo]: https://doc.rust-lang.org/cargo/reference/manifest.html
//! [AWS Parameter Store]: https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-paramstore.html
//! [S-expressions]: https://github.com/rotty/lexpr-rs
//! [D-Bus]: https://docs.rs/zvariant
//! [FlexBuffers]: https://github.com/google/flatbuffers/tree/master/rust/flexbuffers
//! [DynamoDB Items]: https://docs.rs/serde_dynamo
//! [rusoto_dynamodb]: https://docs.rs/rusoto_dynamodb
//!
//! # F.A.Q.
//! * What are repr C pub structs? They are [pub](https://doc.rust-lang.org/rust-by-example/mod/visibility.html)lic [struct](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)ures that have `#[repr(C)]` [outer attribute](https://doc.rust-lang.org/reference/attributes.html).
//! * Will **this** crate support other [reprs](https://doc.rust-lang.org/nomicon/other-reprs.html) (such as `#[repr(Rust)]`)? No but some other crate might.

use std::io::Read;

use parenthesized_c::ParenthesizedC;
use proc_macro2::Span;
use serde::{Deserialize, Serialize};
use syn::{parse2, spanned::Spanned, visit::Visit, Visibility};
use walkdir::WalkDir;

/// The location information of an individual `#[repr(C)] pub struct`ure.
///
/// Read more about `#[repr(C)] pub struct`ures in Rust [here](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)
#[derive(Serialize, Deserialize, Debug)]
pub struct LineColumnEnds {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

impl LineColumnEnds {
    fn new(span: &Span) -> Self {
        Self {
            start_line: span.start().line,
            start_column: span.start().column,
            end_line: span.end().line,
            end_column: span.end().column,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ReprCPubStructs(Vec<LineColumnEnds>);

/// Checks if the given [syn::ItemStruct] is a `#[repr(C)] pub` structure.
///
/// Read more about `#[repr(C)]` in Rust
///     [here](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc).
pub fn is_repr_c_and_public(node: &syn::ItemStruct) -> bool {
    let is_public = matches!(&node.vis, Visibility::Public(_));
    let is_repr_c = node.attrs.iter().any(|attr| {
        // According to https://docs.rs/syn/latest/syn/struct.Attribute.html ,
        //  syn::Attribute represents attributes like `#[derive(Debug, Default, ...)]`.
        // Attribute tokens for `repr` attributes are expected to be generally small
        attr.path.is_ident("repr") && matches!(parse2::<ParenthesizedC>(attr.tokens.clone()), Ok(_))
    });
    is_public && is_repr_c
}

// https://docs.rs/syn/latest/syn/visit/index.html
impl<'ast> Visit<'ast> for ReprCPubStructs {
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        if is_repr_c_and_public(node) {
            self.0.push(LineColumnEnds::new(&node.span()))
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedFile {
    pub path: String,
    pub repr_c_pub_structs: ReprCPubStructs,
}

/// Traverses all directories in `crate_root/src/**` filtering Rust source files, parsing them and returning a
/// collection of location information of `#[repc(C)] pub struct`ures in each file.
///
/// # Example
///
/// `src/main.rs`
///
/// ```rust
/// use repr_c_pub_struct::parse_for_repr_c_pub_structs;
///
/// fn main() {
///     let crate_root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
///     let repr_c_structs = parse_for_repr_c_pub_structs(crate_root.as_str());
///     println!("{:#?}", repr_c_structs);
/// }
/// ```
///
/// `src/unused.rs`
///
/// ```rust
/// #[repr(C)]
/// pub struct Unused {
///    // Test
/// }
/// ```
///
/// # Output on Windows
///
/// ```text
/// [
///     ParsedFile {
///         path: "...\\repr_c_pub_struct\\src\\lib.rs",
///         repr_c_pub_structs: ReprCPubStructs(
///             [],
///         ),
///     },
///     ParsedFile {
///         path: "...\\repr_c_pub_struct\\src\\main.rs",
///         repr_c_pub_structs: ReprCPubStructs(
///             [],
///         ),
///     },
///     ParsedFile {
///         path: "...\\repr_c_pub_struct\\src\\unused.rs",
///         repr_c_pub_structs: ReprCPubStructs(
///             [
///                 LineColumnEnds {
///                     start_line: 1,
///                     start_column: 0,
///                     end_line: 4,
///                     end_column: 1,
///                 },
///             ],
///         ),
///     },
/// ]
/// ```
///
/// # Note
/// The paths will be absolute. The prefixes have been deleted intentionally.
pub fn parse_for_repr_c_pub_structs(crate_root: &str) -> Vec<ParsedFile> {
    // With prior information, the buffer could be preallocated
    let mut buffer = String::new();

    // All errors are skipped. In the hindsight, the solution with ? would be better
    WalkDir::new(format!("{}{}src", crate_root, std::path::MAIN_SEPARATOR))
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".rs"))
        // BufReader is unnecessary bc the files are read only once
        // https://doc.rust-lang.org/std/io/struct.BufReader.html
        .filter_map(|rust_file| {
            let path = rust_file.path();
            std::fs::File::open(path)
                .map(|file: std::fs::File| (path.to_string_lossy().into_owned(), file))
                .ok()
        })
        .filter_map(|(path, mut file): (String, std::fs::File)| {
            file.read_to_string(&mut buffer)
                .map(|_byted_read| {
                    let file = syn::parse_file(&buffer);
                    buffer.truncate(0);
                    (path, file)
                })
                .ok()
        })
        .filter_map(|(path, res_file): (String, syn::Result<syn::File>)| {
            res_file.ok().map(|file| (path, file))
        })
        .fold(
            Vec::<ParsedFile>::new(),
            |mut parsed_files: Vec<ParsedFile>, (path, file): (String, syn::File)| {
                parsed_files.push(ParsedFile {
                    path,
                    repr_c_pub_structs: {
                        let mut repr_c_pub_structs = ReprCPubStructs::default();
                        repr_c_pub_structs.visit_file(&file);
                        repr_c_pub_structs
                    },
                });
                parsed_files
            },
        )
}
