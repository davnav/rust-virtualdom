///How to write a Virtual Dom in Rust
///inspired from https://medium.com/@deathmood/how-to-write-your-own-virtual-dom-ee74acc13060


pub struct VirtualDom{
    nodes:Vec<Node>,
}

pub struct Node{
    tag:String,
    props:Vec<String>,
    chirdren:Vec<Node>,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
