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

enum Ticket {
    Backstage(TicketInfo),
    Vip(TicketInfo),
    Standard(f64),
    // Course answer used two data in an enum, removing the need for another struct.
}

struct TicketInfo {
    holder_name: String,
    price: f64,
}

fn main() {
    let tickets = vec![
        Ticket::Backstage(TicketInfo {
            holder_name: "Anthony".to_owned(),
            price: 45.99,
        }),
        Ticket::Backstage(TicketInfo {
            holder_name: "Ryan".to_owned(),
            price: 45.99,
        }),
        Ticket::Backstage(TicketInfo {
            holder_name: "Ian".to_owned(),
            price: 45.99,
        }),
        Ticket::Vip(TicketInfo {
            holder_name: "Shayne".to_owned(),
            price: 95.99,
        }),
        Ticket::Vip(TicketInfo {
            holder_name: "Courtney".to_owned(),
            price: 95.99,
        }),
        Ticket::Standard(25.99),
    ];

    for ticket in tickets {
        match ticket {
            // Match arms can point to an bracket for multiline actions.
            Ticket::Standard(price) => println!("Standard Ticket (Price: {:?})", price),
            Ticket::Vip(ticket_info) => println!(
                "VIP Ticket (Owned by {:?} | Price: {:?})",
                ticket_info.holder_name, ticket_info.price
            ),
            Ticket::Backstage(ticket_info) => println!(
                "Backstage Ticket (Owned by {:?} | Price: {:?})",
                ticket_info.holder_name, ticket_info.price
            ),
        }
    }
}
