use crate::attractions::{MovieTheater, TicketSeller};

#[derive(Debug)]
pub struct VenueManagement<T: TicketSeller> {
    pub venue: T,
    manager: Option<String>,
}

impl<T: TicketSeller> VenueManagement<T> {
    pub fn new(venue: T) -> Self {
        Self {
            venue,
            manager: None,
        }
    }

    fn hire_manager(&mut self, manager: &str) {
        self.manager = Some(manager.to_string());
    }

    pub fn make_money(&mut self) {
        self.venue.sell_ticket();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attractions::MovieTheater;
    use pretty_assertions::assert_eq;

    #[test]
    fn venue_managment_can_hire_manager() {
        let movie_threater = MovieTheater::new();
        let mut venue_mgmt = VenueManagement::new(movie_threater);
        venue_mgmt.hire_manager("Mario");
        assert_eq!(venue_mgmt.manager.unwrap(), String::from("Mario"));
    }
}
