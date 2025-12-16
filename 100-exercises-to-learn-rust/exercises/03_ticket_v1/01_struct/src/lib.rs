// Define a struct named `Order` with the following fields:
// - `price`, an unsigned integer
// - `quantity`, an unsigned integer
//
// It should also have a method named `is_available` that returns a `true` if the quantity is
// greater than 0, otherwise `false`.

struct Order {
    price: u32,
    quantity: u32
}

// this was the first syntax
// impl Order {
//     fn is_available(self) -> bool {
//         if self.quantity > 0 {
//             return true
//         }
//         false
//     }
// }

impl Order {
    // this borrows self
    // hence it will use the value and not destroy the order object
    // not using the '&self' makes sure we use self and destroy it completely
    // hence there is no way to use Order object again
    fn is_available(&self) -> bool { 
        self.quantity > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
