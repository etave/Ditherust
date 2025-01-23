use argh::FromArgs;

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
pub struct DitherustArgs {
    /// le fichier d’entrée
    #[argh(positional)]
    pub input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    pub output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    pub mode: DitherustMode,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
pub enum DitherustMode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
    TramageAleatoire(OptsTramageAleatoire),
    TramageBayer(OptsTramageBayer),
    DiffusionErreur(OptsDiffusionErreur),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "seuil")]
/// Rendu de l’image par seuillage monochrome.
pub struct OptsSeuil {
    #[argh(option)]
    /// la première couleur (en hexadécimal) à utiliser (ex: 777777)
    pub couleur_1: Option<String>,
    #[argh(option)]
    /// la deuxième couleur (en hexadécimal) à utiliser (ex: 333333)
    pub couleur_2: Option<String>,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
pub struct OptsPalette {
    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA] (dans cet ordre de 1 à 8)
    #[argh(option)]
    pub nb_couleurs: usize,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "tramage_aleatoire")]
/// Rendu de l’image par tramage aléatoire.
pub struct OptsTramageAleatoire {}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "tramage_bayer")]
/// Rendu de l’image par tramage Bayer.
pub struct OptsTramageBayer {
    /// l'ordre de la matrice de Bayer (conseillé: 2, max: 10 (> 10: overflow))
    #[argh(positional)]
    pub ordre: u32,
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name = "diffusion_erreur")]
/// Rendu de l’image par diffusion d'erreur.
pub struct OptsDiffusionErreur {}
