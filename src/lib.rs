#[derive(Debug, PartialEq, Eq)]
struct Museum {
    paintings: Vec<String>,
    revenue: u32,
}

impl Museum {
    const MAXIMUM_CAPACITY: usize = 3;

    fn new() -> Self {
        Self {
            paintings: vec![],
            revenue: 0,
        }
    }

    fn buy_painting(&mut self, painting: &str) {
        if self.paintings.len() >= Self::MAXIMUM_CAPACITY {
            panic!("museum does not have storage space for another painting.");
        }

        self.paintings.push(painting.to_string());
    }

    fn sell_ticket(&mut self) {
        self.revenue += 25;
    }

    fn has_impressive_collection_ticket(&self) -> bool {
        self.paintings.len() > 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    #[test]
    fn print_success(){
        println!("Success inside the function");
        assert!(true);
    }

    #[test]
    fn print_failure(){
        println!("Failure inside the function");
        assert!(false);
    }

    #[test]
    fn result_example() -> Result<(), String> {
        Err(String::from("Failure"))
    }

    #[test]
    fn museum_sells_ticket_to_increase_revenue() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 25);
    }

    #[test]
    fn museum_can_sell_multiple_tickets() {
        let mut museum = Museum::new();
        museum.sell_ticket();
        museum.sell_ticket();
        assert_eq!(museum.revenue, 25, "The revenue expected does not match");
    }

    #[test]
    #[ignore]
    fn museum_can_have_impressive_art() {
        let mut musuem = Museum::new();
        musuem.buy_painting("Mona Lisa");
        musuem.buy_painting("The Starry Night");
        musuem.buy_painting("Sunnny Day");
        assert!(
            musuem.has_impressive_collection_ticket(),
            "The museum did not have an impressive collection despite having 2 painings"
        );
    }

    #[test]
    fn hashmaps() {
        let mut one = HashMap::new();
        one.insert("A", 2);
        one.insert("B", 3);

        let mut two = HashMap::new();
        two.insert("A", 2);
        two.insert("C", 4);

        assert_eq!(one, two);
    }

    #[test]
    fn new_museums_are_equal() {
        let museum1 = Museum::new();
        let museum2 = Museum::new();
        assert_eq!(
            museum1, museum2,
            "Two new museum instances were not found to be equal"
        );
    }

    #[test]
    #[should_panic(expected = "storage space")]
    fn museum_prohibits_adding_painting_when_capacity_has_been_reached() {
        let mut musuem = Museum::new();
        musuem.buy_painting("Mona Lisa");
        musuem.buy_painting("The Starry Night");
        musuem.buy_painting("Sunnny Day");
        musuem.buy_painting("Water lilies");
    }
}
