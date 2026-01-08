use serde::{Deserialize, Serialize};

/// Spectral classification of stars
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpectralClass {
    /// Hottest, blue stars (30,000-60,000K)
    O,
    /// Blue-white stars (10,000-30,000K)
    B,
    /// White stars (7,500-10,000K)
    A,
    /// Yellow-white stars (6,000-7,500K)
    F,
    /// Yellow stars like our Sun (5,200-6,000K)
    G,
    /// Orange stars (3,700-5,200K)
    K,
    /// Red dwarf stars (2,400-3,700K)
    M,
    /// Brown dwarfs (1,300-2,400K)
    L,
    /// Cool brown dwarfs (700-1,300K)
    T,
    /// Coldest brown dwarfs (<700K)
    Y,
}

impl SpectralClass {
    /// Get the typical temperature range for this spectral class
    pub fn temperature_range(&self) -> (f64, f64) {
        match self {
            SpectralClass::O => (30000.0, 60000.0),
            SpectralClass::B => (10000.0, 30000.0),
            SpectralClass::A => (7500.0, 10000.0),
            SpectralClass::F => (6000.0, 7500.0),
            SpectralClass::G => (5200.0, 6000.0),
            SpectralClass::K => (3700.0, 5200.0),
            SpectralClass::M => (2400.0, 3700.0),
            SpectralClass::L => (1300.0, 2400.0),
            SpectralClass::T => (700.0, 1300.0),
            SpectralClass::Y => (250.0, 700.0),
        }
    }

    /// Get the typical color for this spectral class
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            SpectralClass::O => (155, 176, 255),
            SpectralClass::B => (170, 191, 255),
            SpectralClass::A => (202, 215, 255),
            SpectralClass::F => (248, 247, 255),
            SpectralClass::G => (255, 244, 234),
            SpectralClass::K => (255, 210, 161),
            SpectralClass::M => (255, 204, 111),
            SpectralClass::L => (255, 150, 80),
            SpectralClass::T => (200, 100, 60),
            SpectralClass::Y => (150, 80, 50),
        }
    }

    /// Relative probability of this star type (based on stellar population)
    pub fn probability_weight(&self) -> f64 {
        match self {
            SpectralClass::O => 0.00003,
            SpectralClass::B => 0.0013,
            SpectralClass::A => 0.006,
            SpectralClass::F => 0.03,
            SpectralClass::G => 0.076,
            SpectralClass::K => 0.121,
            SpectralClass::M => 0.765,
            SpectralClass::L => 0.0,   // Brown dwarfs handled separately
            SpectralClass::T => 0.0,
            SpectralClass::Y => 0.0,
        }
    }
}

/// Luminosity class of a star
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LuminosityClass {
    /// Hypergiants
    Ia0,
    /// Luminous supergiants
    Ia,
    /// Less luminous supergiants
    Ib,
    /// Bright giants
    II,
    /// Normal giants
    III,
    /// Subgiants
    IV,
    /// Main sequence (dwarfs)
    V,
    /// Subdwarfs
    VI,
    /// White dwarfs
    VII,
}

/// A star in the galaxy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    /// Unique identifier
    pub id: u64,
    /// Name of the star
    pub name: String,
    /// Mass relative to solar mass
    pub mass: f64,
    /// Radius relative to solar radius
    pub radius: f64,
    /// Surface temperature in Kelvin
    pub temperature: f64,
    /// Luminosity relative to solar luminosity
    pub luminosity: f64,
    /// Spectral classification
    pub spectral_class: SpectralClass,
    /// Luminosity classification
    pub luminosity_class: LuminosityClass,
    /// Age in billions of years
    pub age: f64,
    /// Metallicity (iron abundance relative to hydrogen, compared to Sun)
    pub metallicity: f64,
}

impl Star {
    /// Calculate the habitable zone inner boundary in AU
    pub fn habitable_zone_inner(&self) -> f64 {
        (self.luminosity / 1.1).sqrt()
    }

    /// Calculate the habitable zone outer boundary in AU
    pub fn habitable_zone_outer(&self) -> f64 {
        (self.luminosity / 0.53).sqrt()
    }

    /// Calculate the frost line distance in AU
    pub fn frost_line(&self) -> f64 {
        4.85 * self.luminosity.sqrt()
    }

    /// Get the color as RGB tuple
    pub fn color(&self) -> (u8, u8, u8) {
        self.spectral_class.color()
    }

    /// Check if the star is on the main sequence
    pub fn is_main_sequence(&self) -> bool {
        matches!(self.luminosity_class, LuminosityClass::V)
    }
}
