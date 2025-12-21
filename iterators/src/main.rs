#![allow(unused, dead_code)]

use std::collections::HashMap;
use std::env;
#[derive(Debug, PartialEq, Eq, Hash)]
enum Product {
    Blender,
    Microwave,
    Toaster,
    Fridge,
}

#[derive(Debug)]
struct CustomerOrder {
    product: Product,
    quantity: u32,
    shipped: bool,
}

impl CustomerOrder {
    fn new(product: Product, quantity: u32, shipped: bool) -> Self {
        Self {
            product,
            quantity,
            shipped,
        }
    }
}

#[derive(Debug)]
struct Customer {
    id: u32,
    orders: Vec<CustomerOrder>,
}

impl Ord for Customer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl Eq for Customer {}
impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl PartialOrd for Customer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut orders = vec![
        CustomerOrder::new(Product::Blender, 3, false),
        CustomerOrder::new(Product::Microwave, 1, true),
        CustomerOrder::new(Product::Toaster, 2, false),
        CustomerOrder::new(Product::Microwave, 5, true),
        CustomerOrder::new(Product::Blender, 1, false),
        CustomerOrder::new(Product::Fridge, 10, false),
    ];

    let customer_ids_by_order = [2, 1, 2, 3, 4, 1];

    let customers_that_ordered_blenders: Vec<&CustomerOrder> = orders
        .iter()
        .filter(|blender| blender.product == Product::Blender)
        .collect();

    //println!("{customers_that_ordered_blenders:#?}");

    let quantity_of_microwaves_ordered_filter: u32 = orders
        .iter()
        .filter(|microwave| microwave.product == Product::Microwave)
        .map(|q| q.quantity)
        .sum();

    //println!("{quantity_of_microwaves_ordered_filter}");

    let quantity_of_microwaves_ordered_filter_map: u32 = orders
        .iter()
        .filter_map(|microwave| match microwave.product {
            Product::Microwave => Some(microwave.quantity),
            _ => None,
        })
        .sum();

    //println!("{quantity_of_microwaves_ordered_filter_map}");

    let greater_than_number = env::args()
        .skip(1)
        .take(1)
        .map(|q| q.parse::<u32>().unwrap_or(2))
        .next()
        .unwrap_or(2);

    let orders_greater_than = orders
        .iter()
        .filter(|q| q.quantity >= greater_than_number)
        .collect::<Vec<&CustomerOrder>>();

    //println!("{orders_greater_than:#?}");

    let mut unshipped_inventory: HashMap<&Product, u32> = HashMap::new();
    let unshipped_orders: Vec<&CustomerOrder> = orders.iter().filter(|uo| !uo.shipped).collect();

    //consider using the fold method on the unshipped_orders vec here instead.
    for order in unshipped_orders {
        unshipped_inventory
            .entry(&order.product)
            .and_modify(|o| *o += order.quantity)
            .or_insert(order.quantity);
    }

    //println!("{unshipped_inventory:?}");

    //consider using the if let construct instead
    match orders.iter_mut().find(|o| !o.shipped) {
        Some(switch) => switch.shipped = true,
        _ => println!("Element not found!"),
    };

    //println!("{orders:#?}");

    //every for loop can be changed into an iterator method
    //let mut temp_details: Vec<(u32, Vec<CustomerOrder>)> = vec![];

    //consider using .zip method to merge two iterators together
    //can use an itorator method on fold method
    //into_iter.zip.fold.into_iter.map.collect
    //    for (index, details) in orders.into_iter().enumerate() {
    //        if temp_details
    //            .iter()
    //            .any(|d| d.0 == customer_ids_by_order[index])
    //        {
    //            //consider using an if let construct here
    //            match temp_details.iter_mut().find(|d| d.0 == customer_ids_by_order[index]){
    //                Some(c) => c.1.push(details),
    //                _ => println!("There was a problem pushing CustomerOrder into index 2 of the touple inside of temp_details vector"),
    //            }
    //        } else {
    //            temp_details.push((customer_ids_by_order[index], vec![details]));
    //        }
    //    }

    //println!("{temp_details:#?}");

    //    let mut customer_details: Vec<Customer> = vec![];
    //
    //    for detail in temp_details {
    //        customer_details.push(Customer {
    //            id: detail.0,
    //            orders: detail.1,
    //        });
    //    }

    //consider using sort_by_key method instead of impl Ord, Eq, PartialEq and PartialOrd
    //    customer_details.sort();
    //    println!("{customer_details:#?}");

    let mut unshipped_orders = customer_ids_by_order
        .into_iter()
        .zip(orders)
        .fold(
            HashMap::new(),
            |mut acc: HashMap<i32, Vec<CustomerOrder>>, (id, order)| {
                let orders = acc.entry(id).or_default();
                orders.push(order);
                acc
            },
        )
        .into_iter()
        .map(|(key, value)| Customer {
            id: key as u32,
            orders: value,
        })
        .collect::<Vec<Customer>>();

    unshipped_orders.sort_by_key(|customer| customer.id);
    println!("{unshipped_orders:#?}");
}
