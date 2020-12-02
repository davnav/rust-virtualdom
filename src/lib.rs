///How to write a Virtual Dom in Rust
///inspired from https://medium.com/@deathmood/how-to-write-your-own-virtual-dom-ee74acc13060
/// 
use html_parser::Dom; 

/// 
/// Sample DOM elements like
/// { tag:'..', props: '..' ,childern: [..] }
/// 
/// DOM text nodes would be plain Strings
/// 
///

///Virtual DOM
pub struct VirtualDom{
    parent:Node,
//    nodes:Vec<Box<Node>>,
}

impl VirtualDom{
    fn new(root:Node) ->Self{
        VirtualDom{
            parent:root,
        }
    }
}

static html:&str = "<ul class='list'>
                        <li>item1</li>
                        <li>item2</li>
                     </ul>";
pub struct Node{
    tag:String,
    props:Vec<String>,
    chirdren:Vec<Box<Node>>,
}


fn parseNode(htl:&str) {

    let dm = Dom::parse(&htl);
    println!("{:?}",dm);



}




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        parseNode(html);
    
    }
}
