/// Sanskrit verb roots (dhātus)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dhatu {
    /// Root text (e.g., "भू")
    pub root: String,
    /// Gaṇa (verb class): bhvādi, adādi, etc.
    pub gana: Gana,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gana {
    /// First gaṇa (भ्वादि)
    Bhvadi,
    /// Second gaṇa (अदादि)
    Adadi,
    /// Third gaṇa (जुहोत्यादि)
    Juhotyadi,
    /// Fourth gaṇa (दिवादि)
    Divadi,
    /// Fifth gaṇa (स्वादि)
    Svadi,
    /// Sixth gaṇa (तुदादि)
    Tudadi,
    /// Seventh gaṇa (रुधादि)
    Rudhadi,
    /// Eighth gaṇa (तनादि)
    Tanadi,
    /// Ninth gaṇa (क्र्यादि)
    Kryadi,
    /// Tenth gaṇa (चुरादि)
    Curadi,
}

impl Dhatu {
    pub fn new(root: String, gana: Gana) -> Self {
        Self { root, gana }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dhatu_creation() {
        let dhatu = Dhatu::new("भू".to_string(), Gana::Bhvadi);
        assert_eq!(dhatu.root, "भू");
        assert_eq!(dhatu.gana, Gana::Bhvadi);
    }
}
