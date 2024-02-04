use crate::chess::position::Position;

#[test]
fn position_from_human_readable() {
    assert_eq!(Position::from_human_readable(""), None);
    assert_eq!(Position::from_human_readable("rewrwe"), None);
    assert_eq!(Position::from_human_readable("5a"), None);
    assert_eq!(Position::from_human_readable("a9"), None);
    assert_eq!(Position::from_human_readable("i3"), None);
    assert_eq!(Position::from_human_readable("z9"), None);
    assert_eq!(
        Position::from_human_readable("a5").unwrap(),
        Position::new(0, 4)
    );

    round_trip_position("a1");
    round_trip_position("a2");
    round_trip_position("b5");
    round_trip_position("f7");
    round_trip_position("d1");
    round_trip_position("g8");
    round_trip_position("a4");
    round_trip_position("c4");
    round_trip_position("e6");
    round_trip_position("h3");
    round_trip_position("h2");
    round_trip_position("f5");
    round_trip_position("h5");
    round_trip_position("d6");
    round_trip_position("e3");
    round_trip_position("f8");
    round_trip_position("a1");
    round_trip_position("c5");
    round_trip_position("d5");
    round_trip_position("g7");
    round_trip_position("b7");
    round_trip_position("d7");
}

fn round_trip_position(case: &str) {
    let case = case.to_ascii_uppercase();
    assert_eq!(
        case,
        Position::from_human_readable(&case)
            .unwrap()
            .to_human_readable()
    )
}
