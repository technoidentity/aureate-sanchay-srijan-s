#[derive(Debug)]
enum OrderStatus{
    Order_Placed,
    Processing,
    Out_for_delivery,
    Delivered,
}

struct Order {
    OrderID:u32,
    Name:String,
    Amount:u32,
    Status:OrderStatus,
}

fn main() {
    let order = Create_Order(12345,String::from("Sanchay srijan"),1000,OrderStatus::Delivered);
    println!("Hi {} , your order details:",order.Name);
    println!("Your order ID {}:",order.OrderID);
    println!("your order Amount:{}",order.Amount);
    println!("Order Status: {:?}",order.Status);
}

fn Create_Order(OrderID:u32,Name:String,Amount:u32,Status:OrderStatus,)->Order{
    Order {
        OrderID:OrderID,
        Name:Name,
        Amount:Amount,
        Status:Status,
    }
}
