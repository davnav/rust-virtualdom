///How to write a Virtual Dom in Rust
///inspired from https://medium.com/@deathmood/how-to-write-your-own-virtual-dom-ee74acc13060
///
use html_parser::{ Dom, Node  }; 
use html_parser::Element;
use wasm_bindgen::prelude::*;
use std::panic;
use web_sys::console;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}



static html:&str = "<ul class='list'>
                        <p>
                        <li>item1</li>
                        <li>item2</li>
                        </p>
                     </ul>";
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


#[derive(Debug)]
pub struct Node1{
    tag:String,
    props:Vec<String>,
    children:Vec<Node>,
}

pub fn parseNode(htl:&str)  {

    let dm = Dom::parse(&htl).unwrap();

    println!("{:?}",dm);

        let mut i = 0;

  
        let mut parentchild = dm.children.clone();
        PrintElement(parentchild);

    }
    
fn PrintElement(vecnode: Vec<Node>){
    
    for child in vecnode{
            match  child  {
                Node::Element(x) => {
                                     println!("{}",x.name);
                                     PrintElement(x.children);
                                     createElement(x.name.as_str());
                                    }
            
                Node::Text(x)    => { println!("{}",x);
                                     createText(x.as_str());
                                    }
                _                => { println!("end of element");}

    }
}
         //   println!("{:?}",child);
       
/*
            while let Node::Element(ref p)  = parentchild{
                let MychildNodeIter = p.children.clone();
   
                let mut i =0;
                for  child in MychildNodeIter{

                    if let Node::Element(x) = &child{
                        let nod = Node1{
                            tag:x.name.clone(),
                            props:x.classes.clone(),
                            children: x.children.clone(),                                      
                        };
                        println!("{}",nod.tag);
                       // createElement(&nod);
            
                        parentchild  = nod.children[i].clone();
                        i += 1;
                    }

                }     
           
            }
*/             
        #[cfg(feature = "debug")]
        panic::set_hook(Box::new(console_error_panic_hook::hook));


        //    Node::Text(t) => {println!("{}",t);},
    //    Node::Comment(s) => { println!( "comment");},
    //    _ => { println!("unreachable")},

}

#[wasm_bindgen(start)]
pub fn run() -> Result<(),JsValue>{
    parseNode(html);
    // let current = &node.children;
    // let mut i =0;
    // while  !current.is_empty()   {
    //     createElement(&node)?;
    //     current = &node.children[i];
    //     i=i+1;
    // }
    Ok(())
}
pub fn createElement(tag:&str) -> Result<(),JsValue> {
    
    let window =  web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element(tag)?;
    val.set_inner_html("hello rust");
    body.append_child(&val)?;

    Ok(())

} 


pub fn createText(text:&str) -> Result<(),JsValue> {
    
    let window =  web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document window");
    let body = document.body().expect("document should have a body");
    console::log_1(&text.into());
    let val = document.create_comment(text);
  //  val.set_inner_html("hello rust");
    body.append_child(&val)?;

    Ok(())

} 


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn it_works(){
        parseNode(html);
    }
}




