use pretty_assertions::assert_eq;
use rstest::{fixture, rstest};
use testing_lib::attractions::{MovieTheater, Museum};
use testing_lib::management::VenueManagement;

#[fixture]
fn musuem_with_three_paintings() -> Museum {
    let mut museum = Museum::new();
    museum.buy_painting("Mona Lisa");
    museum.buy_painting("the Starry night");
    museum.buy_painting("Girl with a Pearl Earring");

    museum
}

#[fixture]
fn museum_managment(musuem_with_three_paintings: Museum) -> VenueManagement<Museum> {
    VenueManagement::new(musuem_with_three_paintings)
}

#[fixture]
fn movie_theater_with_one_movie() -> MovieTheater {
    let mut movie_theater = MovieTheater::new();
    movie_theater.add_movie("Titanic");
    movie_theater
}

#[fixture]
fn movie_threater_managment(
    movie_theater_with_one_movie: MovieTheater,
) -> VenueManagement<MovieTheater> {
    VenueManagement::new(movie_theater_with_one_movie)
}

#[fixture]
fn movie_theater_managment() -> VenueManagement<MovieTheater> {
    let mut movie_theater = MovieTheater::new();
    movie_theater.add_movie("Titanic");
    VenueManagement::new(movie_theater)
}

#[rstest]
fn venue_management_interacts_wity_venue(musuem_with_three_paintings: Museum) {
    let mut venue_management = VenueManagement::new(musuem_with_three_paintings);
    venue_management.make_money();
    assert_eq!(venue_management.venue.paintings.len(), 3);
    assert_eq!(venue_management.venue.revenue, 25);
}

#[rstest]
fn venue_management_interacts_with_movie_threater_venue(
    mut movie_theater_managment: VenueManagement<MovieTheater>,
) {
    movie_theater_managment.make_money();
    assert_eq!(movie_theater_managment.venue.sales, 15);
}
