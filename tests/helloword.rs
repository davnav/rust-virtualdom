use wasm_bindgen_test::*;
use virtualdom::*;
static html:&str = "<ul class='list'>
                        <li>item1</li>
                        <li>item2</li>
                     </ul>";


wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen_test]
pub fn test_run() {
    // parseNode(html);
   // let vdom = VirtualDom::new()    
}
