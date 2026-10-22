struct Node {
    value:i32,
    next:Option<Box<Node>>,
}

struct LinkedList{
    head:Option<Box<Node>>,
}


impl LinkedList{
    fn new()-> Self{
        Self{
            head:None,
        }
    }

    fn push(&mut self,value:i32){
        let node = Box::new(Node{
            value,
            next:self.head.take(),
        });

        self.head = Some(node);
    }

    fn search(&self,target:i32)-> bool {
        let mut current = &self.head;

        while let Some(node) = current{
            if node.value == target{
                return true;
            }
            current = &node.next;
        }
        false
    }

    fn print(&self){
        let mut current = &self.head;

        while let Some(node) = current{
            print!("{} -> ",node.value);
            current = &node.next;
        }
    }
}

fn main(){
    let mut list = LinkedList::new();

    list.push(10);
    list.push(20);
    list.push(30);

    println!("Linked List: ");
    list.print();

    let target = 40;
    if list.search(target){
        println!("\n{} found in the linked list.",target);
    }
    else{
        println!("\n{} not found in the linked list.",target);
    }
}