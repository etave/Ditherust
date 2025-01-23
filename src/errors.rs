use std::fmt;

#[derive(Debug)]
pub enum DitherustError {
    InvalidColorFormat,
    InvalidColorCount(u8),
    InvalidBayerOrder(u8),
    InvalidMode,
}

impl std::error::Error for DitherustError {}

impl fmt::Display for DitherustError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DitherustError::InvalidColorFormat => write!(
                f,
                "Les couleurs doivent être des codes hexadécimaux de 6 chiffres (ex: FFFFFF)"
            ),
            DitherustError::InvalidColorCount(count) => write!(
                f,
                "Nombre de couleurs invalide: {}. Doit être entre 1 et 8",
                count
            ),
            DitherustError::InvalidBayerOrder(order) => write!(
                f,
                "Ordre de Bayer invalide: {}. Doit être inférieur ou égal à 10 (pour éviter les overflows) (conseillé: 2)",
                order
            ),
            DitherustError::InvalidMode => write!(f, "Mode invalide, disponible: normal, floyd-steinberg"),
        }
    }
}