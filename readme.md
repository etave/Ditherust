ETAVE Nathan

LANDON--NOMINÉ Cyril

# Rapport TP RUST Dithering

Ce rapport explicite les points dont il a été demandé de donner des détails dans le sujet de TP Dithering.

### Contenu :
- Explication de l'obtention d'une image rgb8 depuis une image DynamicImage
- Réponse à la question 3 par rapport aux images ayant au départ un canal alpha
- Détails du calcul entre 2 couleurs
- Explication du comportement de notre application en cas de palette "vide"
- Détails sur la représentation de l'erreur à chaque pixel


### DynamicImage -> rgb8 

Une image DynamicImage peut être simplement convertit en rgb8 grâce à la fonction associée. 
Prenons une ``` DynamicImage image1 ```:

``` let image1: DynamicImage = ImageReader::open("myimage.png")?.decode()?;```

On la convertit simplement avec la fonction ``` .to_rgb8() ``` :

``` img_rgb8 = image1.to_rgb8(); ```

**A NOTER** : Dans ce projet et son code, nous n'utilisons pas, n'y n'avons besoin de faire cette conversion de l'image en rgb8. 
Par exemple dans le fichier processors > seuil.rs, fonction ``` seuillage ``` les pixels de l'image sont remplacés 1 à 1 dans l'image, l'image en elle-même n'est jamais convertie !

### Réponse à la question 3

Premièrement, pour sauvegarder l'image, ici sous format png, on utilise ``` image1.save(nomdufichierdesortie) ```:

-> io > file.rs (l.8)

-> main.rs (l.60)


Deuxièmement, si l'image **d'origine** (avant conversion ``` .to_rgb8()``` ) incluait un canal alpha, après sa conversion, il aura été supprimé et toutes les zones transparentes seront désormais opaques.


### Détails du calcul entre 2 couleurs

La fonction ``` distance_between_colors ``` peut être retrouvé ici :

-> processors > palette.rs

Simplement, elle reproduit le calcul d'une distance euclidienne, ce qui revient à calculer la distance entre 2 points dans un espace à plusieurs dimensions.

Plus en détails, chacunes des 3 étapes du calcul (englobé par la racine carré ``` .sqrt() ```) va calculer la différence entre le rouge de la couleur 1, le rouge de la couleur 2, puis la différence entre le vert de la couleur 1, etc.


### Explication en cas de palette "vide"

GROS BLANC

### Détails sur la représentation de l'erreur

GROS BLANC * 2