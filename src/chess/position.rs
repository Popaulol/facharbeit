#[derive(PartialEq, Debug)]
pub struct Position {
    file: i32,
    rank: i32,
}

impl Position {
    pub(crate) fn new(file:i32, rank: i32) -> Self {
        assert!((0..8).contains(&file));
        assert!((0..8).contains(&rank));
        Self {
            file, rank
        }
    }

    pub(crate) fn from_human_readable(position: &str) -> Option<Self> {
        if !position.is_ascii() { None }
            else if position.len() != 2 { None }
        else {
            let position = position.to_ascii_lowercase();
            let file = match position.chars().nth(0)? {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                _ => {
                    return None
                }
            };

            let rank  = match position.chars().nth(1)? {
                '1' => 0,
                '2' => 1,
                '3' => 2,
                '4' => 3,
                '5' => 4,
                '6' => 5,
                '7' => 6,
                '8' => 7,
                _ => {
                    return None
                }
            };

            Some(Self {
                file, rank
            })
        }
    }

    pub(crate) fn to_human_readable(&self) -> String {
        let file = match self.file {
            0 => "A",
            1 => "B",
            2 => "C",
            3 => "D",
            4 => "E",
            5 => "F",
            6 => "G",
            7 => "H",
            _ => {
                unreachable!()
            }
        };

        let rank = match self.rank {
            0 => "1",
            1 => "2",
            2 => "3",
            3 => "4",
            4 => "5",
            5 => "6",
            6 => "7",
            7 => "8",
            _ => {
                unreachable!()
            }
        };
        format!("{}{}", file, rank)
    }
}