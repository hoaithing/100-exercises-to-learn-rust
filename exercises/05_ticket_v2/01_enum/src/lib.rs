// TODO: use `Status` as type for `Ticket::status`
//   Adjust the signature and implementation of all other methods as necessary.

trait TicketBase {
    fn is_valid(title: &String, description: &String) -> bool;
    fn title(&self) -> &String;
    fn description(&self) -> &String;
    fn status(&self) -> &Status;
}

#[derive(Debug, PartialEq)]
// `derive`s are recursive: it can only derive `PartialEq` if all fields also implement `PartialEq`.
// Same holds for `Debug`. Do what you must with `Status` to make this work.
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
        Ticket::is_valid(&title, &description);
        Ticket {
            title,
            description,
            status,
        }
    }
}

impl TicketBase for Ticket {
    fn is_valid(title: &String, description: &String) -> bool {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        true
    }

    fn title(&self) -> &String {
        &self.title
    }

    fn description(&self) -> &String {
        &self.description
    }

    fn status(&self) -> &Status {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};

    #[test]
    fn test_partial_eq() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::ToDo,
        };
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let title = valid_title();
        let ticket1 = Ticket {
            title: title.clone(),
            description: "description".to_string(),
            status: Status::ToDo
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: "description2".to_string(),
            status: Status::ToDo
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let description = valid_description();
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: description.clone(),
            status: Status::InProgress,
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: description.clone(),
            status: Status::InProgress,
        };
        assert_ne!(ticket1, ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let title = valid_title();
        let description = valid_description();
        let ticket1 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::InProgress,
        };
        let ticket2 = Ticket {
            title: title.clone(),
            description: description.clone(),
            status: Status::Done,
        };
        assert_ne!(ticket1, ticket2);
    }
}
