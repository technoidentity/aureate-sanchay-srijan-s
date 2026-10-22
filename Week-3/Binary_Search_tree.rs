struct Node {
    value :i32,
    left : Option<Box<Node>>,
    right : Option<Box<Node>>
}

impl Node {
    fn new(value:i32)->Self{
        Self{
            value:value,
            left:None,
            right:None
        }
    }

    fn insert(&mut self , value:i32){

        if value <self.value{
            match self.left.as_mut(){
                Some(left_node)=> {
                    left_node.insert(value);
                }
                None=>{
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
        }
        else{
            match self.right.as_mut(){
                Some(right_node)=> {
                    right_node.insert(value);
                }
                None=>{
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
        }
    }

    fn search(&self,target:i32)-> bool{
        if target == self.value{
            return true;
        }

        if target < self.value{
            match self.left.as_ref(){
                Some(left_node)=> left_node.search(target),
                
                None=> return false,
                
            }
        }
        else{
            match self.right.as_ref(){
                Some(right_node)=> 
                    right_node.search(target),
                None=> 
                    return false,
                
            }
        }
    }
    fn inorder(&self){
        if let Some(left_node) =self.left.as_ref(){
            left_node.inorder()
        }
        println!("{}",self.value);

        if let Some(right_node) = self.right.as_ref(){
            right_node.inorder();
        }
    }
    
}

fn main(){
    let mut root  = Node::new(10);

    root.insert(5);
    root.insert(15);    
    root.insert(3);
    root.insert(7); 
    root.insert(12);

    println!("{}", root.search(11));
    root.inorder();
    
}