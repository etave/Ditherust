ETAVE Nathan

LANDON--NOMINÉ Cyril

# Rapport TP RUST Dithering

Ce rapport explicite les points dont il a été demandé de donner des détails dans le sujet de TP Dithering.

### Contenu :
- Qu'est-ce qu'une DynamicImage ?
- Explication de l'obtention d'une image rgb8 depuis une image DynamicImage.
- Réponse à la question 3 par rapport aux images ayant au départ un canal alpha.
- Détails du calcul entre 2 couleurs.
- Explication du comportement de notre application en cas de palette "vide".
- Détails sur la représentation de l'erreur à chaque pixel.

### Qu'est-ce qu'une DynamicImage ?

Une DynamicImage est un ``` enum ``` qui permet de représenter une image de manière dynamique.
Elle est utilisée pour représenter une image en mémoire, et peut être convertie en différents formats (RGB, RGBA, Luma, ...).

### Conversion d'une DynamicImage en rgb8

Une image DynamicImage peut être simplement convertit en rgb8 grâce à la fonction associée. 
Prenons une ``` DynamicImage image1 ```:

```rust
 let image1: DynamicImage = ImageReader::open("image.png")?.decode()?;
```

On la convertit simplement avec la fonction ```.to_rgb8()``` :

```rust
let img_rgb8: RgbImage = image1.to_rgb8();
```

**A NOTER** : Dans notre projet, nous utilisons l'image sous sa forme d'origine (rgba8), sans la convertir en rgb8. Par exemple, dans le fichier ```seuil.rs``` (dossier processors), la fonction ``` seuillage ``` remplace les pixels de l'image 1 à 1, sans jamais convertir l'image en rgb8.

### Réponse à la question 3

Premièrement, pour sauvegarder l'image, ici sous format png, on utilise ``` image1.save(nomdufichierdesortie) ```:

io > file.rs (l.8)

main.rs (l.60)


Deuxièmement, si l'image **d'origine** (avant conversion ``` .to_rgb8()``` ) incluait un canal alpha, après sa conversion, il aura été supprimé et toutes les zones transparentes seront désormais opaques.


### Détails du calcul entre 2 couleurs

La fonction ``` distance_between_colors ``` peut être retrouvé ici :

```rust
fn distance_between_colors(color1: &Rgba<u8>, color2: &Rgba<u8>) -> f64 {

    let red_diff = color1[0] as f64 - color2[0] as f64;
    let green_diff = color1[1] as f64 - color2[1] as f64;
    let blue_diff = color1[2] as f64 - color2[2] as f64;

    (red_diff.powi(2) + green_diff.powi(2) + blue_diff.powi(2)).sqrt()
}

```

Simplement, elle reproduit le calcul d'une distance euclidienne, ce qui revient à calculer la distance entre 2 points dans un espace à plusieurs dimensions.

Plus en détails, chacunes des 3 étapes du calcul (englobé par la racine carré ``` .sqrt() ```) va calculer la différence entre le rouge de la couleur 1, le rouge de la couleur 2, puis la différence entre le vert de la couleur 1, etc.


### Explication en cas de palette "vide"

GROS BLANC

### Détails sur la représentation de l'erreur

GROS BLANC * 2