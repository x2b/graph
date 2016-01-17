pub enum Direction {
    Outgoing,
    Incoming,
}

impl Direction {
    fn oppposite(&self) -> Direction {
        match *self {
            Direction::Outgoing => Direction::Incoming,
            Direction::Incoming => Direction::Outgoing,
        }
    }
}
