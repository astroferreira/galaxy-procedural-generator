use rand::Rng;

/// Generates procedural names for celestial bodies
pub struct NameGenerator {
    prefixes: Vec<&'static str>,
    suffixes: Vec<&'static str>,
    greek_letters: Vec<&'static str>,
    roman_numerals: Vec<&'static str>,
}

impl Default for NameGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl NameGenerator {
    pub fn new() -> Self {
        Self {
            prefixes: vec![
                "Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta", "Eta", "Theta",
                "Iota", "Kappa", "Lambda", "Mu", "Nu", "Xi", "Omicron", "Pi",
                "Rho", "Sigma", "Tau", "Upsilon", "Phi", "Chi", "Psi", "Omega",
            ],
            suffixes: vec![
                "Centauri", "Eridani", "Cygni", "Draconis", "Orionis", "Pegasi",
                "Aquarii", "Leonis", "Scorpii", "Tauri", "Virginis", "Geminorum",
                "Andromedae", "Persei", "Lyrae", "Carinae", "Bootis", "Aurigae",
                "Cephei", "Cassiopeiae", "Sagittarii", "Crucis", "Phoenicis",
                "Hydrae", "Ursae", "Serpentis", "Aquilae", "Capricorni",
            ],
            greek_letters: vec![
                "α", "β", "γ", "δ", "ε", "ζ", "η", "θ", "ι", "κ", "λ", "μ",
                "ν", "ξ", "ο", "π", "ρ", "σ", "τ", "υ", "φ", "χ", "ψ", "ω",
            ],
            roman_numerals: vec![
                "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X",
                "XI", "XII", "XIII", "XIV", "XV", "XVI", "XVII", "XVIII", "XIX", "XX",
            ],
        }
    }

    /// Generate a star system name
    pub fn generate_system_name<R: Rng>(&self, rng: &mut R) -> String {
        let style = rng.gen_range(0..4);
        match style {
            0 => {
                // Greek letter + constellation style: "Alpha Centauri"
                let prefix = self.prefixes[rng.gen_range(0..self.prefixes.len())];
                let suffix = self.suffixes[rng.gen_range(0..self.suffixes.len())];
                format!("{} {}", prefix, suffix)
            }
            1 => {
                // Catalog style: "HD 142527"
                let catalog = ["HD", "HR", "HIP", "GJ", "Kepler", "TRAPPIST", "TOI", "LHS"][rng.gen_range(0..8)];
                let number = rng.gen_range(1000..999999);
                format!("{}-{}", catalog, number)
            }
            2 => {
                // Designation style: "Ross 128"
                let names = ["Ross", "Wolf", "Barnard", "Lacaille", "Luyten", "Gliese", "Proxima", "Lalande"];
                let name = names[rng.gen_range(0..names.len())];
                let number = rng.gen_range(1..999);
                format!("{} {}", name, number)
            }
            _ => {
                // Generated syllable style
                self.generate_syllabic_name(rng, 2, 4)
            }
        }
    }

    /// Generate a star name (usually based on system name)
    pub fn generate_star_name<R: Rng>(&self, _rng: &mut R, system_name: &str, index: usize) -> String {
        if index == 0 {
            system_name.to_string()
        } else {
            let suffix = ['A', 'B', 'C', 'D', 'E', 'F'][index.min(5)];
            format!("{} {}", system_name, suffix)
        }
    }

    /// Generate a planet name
    pub fn generate_planet_name<R: Rng>(
        &self,
        _rng: &mut R,
        system_name: &str,
        index: usize,
    ) -> String {
        let numeral = if index < self.roman_numerals.len() {
            self.roman_numerals[index]
        } else {
            return format!("{} {}", system_name, index + 1);
        };
        format!("{} {}", system_name, numeral)
    }

    /// Generate a moon name
    pub fn generate_moon_name<R: Rng>(
        &self,
        rng: &mut R,
        planet_name: &str,
        index: usize,
    ) -> String {
        let style = rng.gen_range(0..2);
        match style {
            0 => {
                // Letter designation: "Kepler-442b a"
                let letter = (b'a' + index as u8) as char;
                format!("{} {}", planet_name, letter)
            }
            _ => {
                // Mythological style name
                let names = [
                    "Luna", "Io", "Europa", "Ganymede", "Callisto", "Titan", "Enceladus",
                    "Triton", "Charon", "Miranda", "Ariel", "Umbriel", "Titania", "Oberon",
                    "Phobos", "Deimos", "Hyperion", "Iapetus", "Rhea", "Dione", "Tethys",
                    "Mimas", "Proteus", "Nereid", "Larissa", "Naiad", "Nix", "Hydra",
                ];
                if index < names.len() {
                    names[index].to_string()
                } else {
                    let letter = (b'a' + (index % 26) as u8) as char;
                    format!("{} {}", planet_name, letter)
                }
            }
        }
    }

    /// Generate a syllabic name
    fn generate_syllabic_name<R: Rng>(&self, rng: &mut R, min_syllables: usize, max_syllables: usize) -> String {
        let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'r', 's', 't', 'v', 'w', 'x', 'z'];
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let special = ["th", "ch", "sh", "ph", "kr", "tr", "gr", "br", "dr", "st", "sp", "sk"];

        let syllables = rng.gen_range(min_syllables..=max_syllables);
        let mut name = String::new();

        for i in 0..syllables {
            // Start with consonant or consonant cluster
            if rng.gen_bool(0.3) && i == 0 {
                name.push_str(special[rng.gen_range(0..special.len())]);
            } else {
                name.push(consonants[rng.gen_range(0..consonants.len())]);
            }

            // Add vowel
            name.push(vowels[rng.gen_range(0..vowels.len())]);

            // Maybe add ending consonant
            if rng.gen_bool(0.4) {
                let endings = ['n', 's', 'r', 'l', 'm', 'x'];
                name.push(endings[rng.gen_range(0..endings.len())]);
            }
        }

        // Capitalize first letter
        let mut chars: Vec<char> = name.chars().collect();
        if let Some(first) = chars.first_mut() {
            *first = first.to_uppercase().next().unwrap_or(*first);
        }
        chars.into_iter().collect()
    }

    /// Generate a galaxy name
    pub fn generate_galaxy_name<R: Rng>(&self, rng: &mut R) -> String {
        let style = rng.gen_range(0..3);
        match style {
            0 => {
                // Catalog style: "NGC 4414"
                let catalogs = ["NGC", "IC", "M", "UGC", "PGC"];
                let catalog = catalogs[rng.gen_range(0..catalogs.len())];
                let number = rng.gen_range(1..9999);
                format!("{} {}", catalog, number)
            }
            1 => {
                // Named galaxy style
                let names = [
                    "Andromeda", "Whirlpool", "Sombrero", "Pinwheel", "Cartwheel",
                    "Tadpole", "Sunflower", "Cigar", "Black Eye", "Sculptor",
                ];
                names[rng.gen_range(0..names.len())].to_string()
            }
            _ => {
                // Generated name
                self.generate_syllabic_name(rng, 2, 3)
            }
        }
    }
}
