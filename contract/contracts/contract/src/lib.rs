#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Symbol, Map, Address
};

#[contract]
pub struct EventTicketing;

// ✅ Required for storing struct in contract storage
#[contracttype]
#[derive(Clone)]
pub struct Event {
    pub name: Symbol,
    pub total_tickets: u32,
    pub tickets_sold: u32,
}

// ✅ Storage key constant
fn events_key(e: &Env) -> Symbol {
    Symbol::short("EVENTS")
}

// Get all events
fn events(e: &Env) -> Map<Symbol, Event> {
    e.storage()
        .instance()
        .get(&events_key(e))
        .unwrap_or(Map::new(e))
}

#[contractimpl]
impl EventTicketing {

    // Create a new event
    pub fn create_event(e: Env, event_id: Symbol, name: Symbol, total: u32) {
        let mut event_map = events(&e);

        if event_map.contains_key(event_id.clone()) {
            panic!("Event already exists");
        }

        let event = Event {
            name,
            total_tickets: total,
            tickets_sold: 0,
        };

        event_map.set(event_id, event);
        e.storage().instance().set(&events_key(&e), &event_map);
    }

    // Buy a ticket
    pub fn buy_ticket(e: Env, event_id: Symbol, buyer: Address) {
        buyer.require_auth();

        let mut event_map = events(&e);

        let mut event = event_map
            .get(event_id.clone())
            .expect("Event not found");

        if event.tickets_sold >= event.total_tickets {
            panic!("Tickets sold out");
        }

        event.tickets_sold += 1;

        event_map.set(event_id, event);
        e.storage().instance().set(&events_key(&e), &event_map);
    }

    // Get event details
    pub fn get_event(e: Env, event_id: Symbol) -> Event {
        let event_map = events(&e);
        event_map.get(event_id).expect("Event not found")
    }
}