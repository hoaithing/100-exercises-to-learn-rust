// TODO: Update `add_ticket`'s signature: it should take a `TicketDraft` as input and return a `TicketId` as output.
//  Each ticket should have a unique id, generated by `TicketStore`.
//  Feel free to modify `TicketStore` fields, if needed.
//  You also need to add a `get` method that takes as input a `TicketId` and returns an `Option<&Ticket>`.

use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};
use ticket_fields::{TicketDescription, TicketTitle};

pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TicketId(u64);

impl Display for TicketId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

impl From<TicketDraft> for Ticket {
    fn from(ticket: TicketDraft) -> Self {
        let description = ticket.description;
        let title = ticket.title;
        Self{
            id: TicketId(generate_timestamp_id()),
            title,
            description,
            status: Status::ToDo,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

fn generate_timestamp_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}


impl Default for TicketStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }
    
    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let t: Ticket = ticket.into();
        let t2 = t.clone();
        self.tickets.push(t);
        println!("Added ticket {}", &t2.id);
        t2.id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.iter().find(|&s| s.id == id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft1 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id1 = store.add_ticket(draft1.clone());
        let ticket1 = store.get(id1).unwrap();
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(draft1.description, ticket1.description);
        assert_eq!(ticket1.status, Status::ToDo);

        let draft2 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id2 = store.add_ticket(draft2);
        let _ = store.get(id2).unwrap();

        assert_ne!(id1, id2);
    }
}
