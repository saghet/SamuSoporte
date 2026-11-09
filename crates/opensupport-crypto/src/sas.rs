use sha2::{Digest, Sha256};

pub struct ShortAuthString;

impl ShortAuthString {
    /// Genera un código visual de 6 dígitos basado en el hash del handshake
    pub fn generate_numeric(handshake_hash: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"OPENSUPPORT_SAS_NUMERIC_V1");
        hasher.update(handshake_hash);
        let digest = hasher.finalize();

        // Extraer los primeros 4 bytes como entero
        let val = u32::from_be_bytes([digest[0], digest[1], digest[2], digest[3]]);
        let code = val % 1_000_000;
        format!("{:06}", code)
    }

    /// Genera una lista de 3 palabras reconocibles para validación fonética/visual
    pub fn generate_words(handshake_hash: &[u8]) -> String {
        const WORDLIST: [&str; 64] = [
            "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel",
            "india", "juliet", "kilo", "lima", "mike", "november", "oscar", "papa",
            "quebec", "romeo", "sierra", "tango", "uniform", "victor", "whiskey", "xray",
            "yankee", "zulu", "amber", "bronze", "copper", "diamond", "emerald", "garnet",
            "hazard", "iron", "jade", "knight", "laser", "magnet", "nexus", "orbit",
            "plasma", "quantum", "radar", "shield", "titan", "uranus", "vortex", "zenith",
            "falcon", "hawk", "eagle", "tiger", "panther", "wolf", "badger", "lynx",
            "cobalt", "silver", "golden", "shadow", "summit", "crystal", "phoenix", "beacon"
        ];

        let mut hasher = Sha256::new();
        hasher.update(b"OPENSUPPORT_SAS_WORDS_V1");
        hasher.update(handshake_hash);
        let digest = hasher.finalize();

        let w1 = WORDLIST[(digest[0] as usize) % WORDLIST.len()];
        let w2 = WORDLIST[(digest[1] as usize) % WORDLIST.len()];
        let w3 = WORDLIST[(digest[2] as usize) % WORDLIST.len()];

        format!("{}-{}-{}", w1, w2, w3)
    }
}
