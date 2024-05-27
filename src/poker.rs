use rand_core::OsRng;
use vrf_r255::{PublicKey, SecretKey};

pub struct Card {
    number: u32,
}

pub struct Player {
    pub sk: Option<SecretKey>,
    pub pk: PublicKey,
    pub cards: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
        let sk = SecretKey::generate(OsRng);
        let pk = PublicKey::from(sk);

        Self {
            sk: Some(sk),
            pk,
            cards: vec![],
        }
    }
    pub fn from_pk(pk: PublicKey) -> Self {
        Self {
            sk: None,
            pk,
            cards: vec![],
        }
    }

    pub fn draw_card(self: Self, board: Board) -> Option<Card> {
        match self.sk {
            Some(sk) => {
                let proof = sk.prove(format!("{}-{}", board.initial_word, board.drawn_cards));
                //SHOULD GENERATE CARD based on mod proof
            }
            None => {}
        }
    }
}

// Should store another players with public keys and cards
pub struct Board {
    pub cards: Vec<Card>,
    pub players: Vec<Player>,
    pub initial_word: String,
    pub drawn_cards: u64,
}
impl Board {
    pub fn new(players: Vec<Player>) -> Self {
        Board {
            cards: vec![],
            players,
            initial_word: "vrf".to_string(),
            drawn_cards: 0,
        }
    }
}

pub fn generate() {
    let p1 = Player::new();
    let msg = "Real World Cryptography".as_bytes();

    println!("{:?}", proof);
    let proof = p1.sk.unwrap().prove(msg);
    println!("{:?}", proof);

    let hash_output = p1.pk.verify(msg, &proof);
    //let vrf_proof = keypair.vrf_proof(b"input data")
}

// Player - has hand with cards drawn by croupier

// Need to create poker game based on vrf card drawing
//
//
//
//
//

#[cfg(test)]
mod tests {
    use crate::poker;
    #[test]
    fn test0() {
        poker::generate();
        assert_eq!(0, 1)
    }
}
