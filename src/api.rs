use arrayvec::ArrayString;
use serde::{Deserialize, de};
use serde_with::rust::display_fromstr;
use shakmaty::Square;
use shakmaty::fen::Fen;
use shakmaty::uci::Uci;

#[derive(Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum Orientation {
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation::White
    }
}

impl Orientation {
    pub fn fold<T>(self, white: T, black: T) -> T {
        match self {
            Orientation::White => white,
            Orientation::Black => black,
        }
    }

    pub fn x(self, square: Square) -> usize {
        self.fold(usize::from(square.file()), 7 - usize::from(square.file()))
    }

    pub fn y(self, square: Square) -> usize {
        self.fold(7 - usize::from(square.rank()), usize::from(square.rank()))
    }
}

pub type PlayerName = ArrayString<[u8; 100]>;

#[derive(Deserialize)]
pub struct RequestParams {
    pub white: Option<PlayerName>,
    pub black: Option<PlayerName>,
    #[serde(with = "display_fromstr", default)]
    pub fen: Fen,
    #[serde(deserialize_with = "display_fromstr::deserialize", default = "uci_null", rename = "lastMove")]
    pub last_move: Uci,
    #[serde(deserialize_with = "maybe_square", default)]
    pub check: Option<Square>,
    #[serde(default)]
    pub orientation: Orientation,
}

#[derive(Deserialize)]
pub struct RequestBody {
    pub white: Option<PlayerName>,
    pub black: Option<PlayerName>,
    pub frames: Vec<RequestFrame>,
    #[serde(default)]
    pub orientation: Orientation,
    #[serde(default)]
    pub delay: u16,
}

#[derive(Deserialize)]
pub struct RequestFrame {
    #[serde(with = "display_fromstr")]
    pub fen: Fen,
    pub delay: Option<u16>,
    #[serde(deserialize_with = "display_fromstr::deserialize", default = "uci_null", alias = "lastMove")]
    pub last_move: Uci,
    #[serde(deserialize_with = "maybe_square")]
    pub check: Option<Square>,
}

fn uci_null() -> Uci {
    Uci::Null
}

fn maybe_square<'de, D>(deserializer: D) -> Result<Option<Square>, D::Error>
where
    D: de::Deserializer<'de>,
{
    Option::<&str>::deserialize(deserializer).and_then(|maybe_name| {
        Ok(match maybe_name {
            Some(name) => Some(name.parse().map_err(|_| de::Error::custom("invalid square name"))?),
            None => None,
        })
    })
}

impl RequestBody {
    pub fn example() -> RequestBody {
        RequestBody {
            white: Some(PlayerName::from("Molinari").unwrap()),
            black: Some(PlayerName::from("Bordais").unwrap()),
            orientation: Orientation::White,
            delay: 50,
            frames: vec![
                RequestFrame {
                    fen: Fen::default(),
                    delay: None,
                    last_move: Uci::Null,
                    check: None,
                },
                RequestFrame {
                    fen: "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1".parse().unwrap(),
                    delay: None,
                    last_move: "e2e4".parse().unwrap(),
                    check: None,
                },
            ],
        }
    }
}
