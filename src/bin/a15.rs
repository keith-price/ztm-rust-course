// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(f64, String),
    Standard(f64),
    VIP(f64, String),
}

fn main() {
    let tickets_vec = vec![
        Tickets::Backstage(49.99, "Derton Flartfqop".to_owned()),
        Tickets::Standard(12.99),
        Tickets::VIP(239.99, "Reemus Lungrattle".to_owned()),
    ];

    for ticket in tickets_vec {
        match ticket {
            Tickets::Backstage(price, name) => {
                println!("This ticket costs: {:?} and belongs to {:?}", price, name)
            }
            Tickets::Standard(price) => println!("This ticket costs: {:?}", price),
            Tickets::VIP(price, name) => {
                println!("This ticket costs: {:?} and belongs to {:?}", price, name)
            }
        }
    }
}
