///How to write a Virtual Dom in Rust
///inspired from https://medium.com/@deathmood/how-to-write-your-own-virtual-dom-ee74acc13060
/// 
use html_parser::{ Dom, Node  }; 
use html_parser::Element;
use wasm_bindgen::prelude::*;




/// 
/// Sample DOM elements like
/// { tag:'..', props: '..' ,childern: [..] }
/// 
/// DOM text nodes would be plain Strings
/// 
///
//type NodeResult = Result<Node1,MyError>;
///Virtual DOM
pub struct VirtualDom{
    parent:Node1,
//    nodes:Vec<Box<Node>>,
}

impl VirtualDom{
    fn new(root:Node1) ->Self{
        VirtualDom{
            parent:root,
        }
    }
}

static html:&str = "<ul class='list'>
                        <li>item1</li>
                        <li>item2</li>
                     </ul>";

#[derive(Debug)]
pub struct Node1{
    tag:String,
    props:Vec<String>,
    children:Vec<Node>,
}

pub fn parseNode(htl:&str) -> Result<Node1,&'static str> {

    let dm = Dom::parse(&htl).unwrap();

    println!("{:?}",dm);

    // let nod = if let Node::Element(x) = dm.children[0].clone()   {
    //                     Ok (Node1{ tag: x.name,
    //                                  props: x.classes,
    //                                  children:x.children,
    //                     })
    //            }else{
    //                 Err("not a good html element") ;
    //            };

        match dm.children[0].clone(){
            Node::Element(x) => {
                    let nod = Node1{
                        tag:x.name,
                        props:x.classes,
                        children: x.children                                       
                    };
                    Ok(nod)

                },
             _ => { Err("not an element") },
        }
    //    Node::Text(t) => {println!("{}",t);},
    //    Node::Comment(s) => { println!( "comment");},
    //    _ => { println!("unreachable")},


}

#[wasm_bindgen(start)]
pub fn run() -> Result<(),JsValue>{
    let node = parseNode(html).unwrap();
    createElement(node)?;
    Ok(())
}
pub fn createElement(node:Node1) -> Result<(),JsValue> {


    let window =  web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element(node.tag.as_str())?;
    val.set_inner_html("hello rust");
    body.append_child(&val)?;

    Ok(())

} 





