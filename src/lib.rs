///How to write a Virtual Dom in Rust
///inspired from https://medium.com/@deathmood/how-to-write-your-own-virtual-dom-ee74acc13060
/// 
use html_parser::{ Dom, Result,Node  }; 
use html_parser::Element;
/// 
/// Sample DOM elements like
/// { tag:'..', props: '..' ,childern: [..] }
/// 
/// DOM text nodes would be plain Strings
/// 
///

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

fn parseNode(htl:&str) -> Result<()>{

    let dm = Dom::parse(&htl)?;

    match dm.children[0].clone()  {
        Node::Element(x) => {
                               let nod = Node1{ tag: x.name,
                                     props: x.classes,
                                     children:x.children,
                                 
                                    };
                                println!("{:?}",nod);

                            }
                                    
        Node::Text(t) => {println!("{}",t);},
        Node::Comment(s) => { println!( "comment");},
        _ => { println!("unreachable")},
    }

    Ok(())

}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
       
        parseNode(html);
    
    }
}
