// Import the necessary dependencies
use chrono::prelude::*;

/// A struct representing an important event with a name and a deadline.
struct ImportantEvent {
    /// The name of the event.
    what: String,
    /// The deadline for the event.
    when: DateTime<Local>,
}

/// A trait to check if a deadline has passed.
trait Deadline {
    /// Returns true if the deadline has passed, or false otherwise.
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when < Local::now()
    }
}

#[cfg(test)]
mod deadline_passed_tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn in_past() {
        // Create an instance of ImportantEvent for a past event
        let event = ImportantEvent {
            what: String::from("friend's birthday"),
            when: Local::now() - Duration::hours(25),
        };

        // Check if the past event deadline has passed
        assert!(event.is_passed())
    }

    #[test]
    fn in_future() {
        // Create an instance of ImportantEvent for a future event
        let event = ImportantEvent {
            what: String::from("friend's birthday"),
            when: Local::now() + Duration::hours(25),
        };

        // Check if the future event deadline has passed
        assert!(!event.is_passed())
    }
}
