// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut hs = HashMap::new();
        hs.insert(String::from("done"), Status::Done);
        hs.insert(String::from("inprogress"), Status::InProgress);
        hs.insert(String::from("todo"), Status::ToDo);

        println!("{:?}", hs);
        match hs.get(value.to_lowercase().as_str()) {
            Some(v) => Ok(v.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = value.to_lowercase();
        match v.as_str() {
            "done" => Ok(Status::Done),
            "inprogress" => Ok(Status::InProgress),
            "todo" => Ok(Status::ToDo),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
