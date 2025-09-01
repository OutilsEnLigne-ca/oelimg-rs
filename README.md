
# oelimg-rs

[lien NPM](https://www.npmjs.com/package/oelimg-rs)

🦀 **Bibliothèque de traitement d’images WebAssembly haute performance** développée en Rust pour les applications web modernes.

Développé par l’équipe d’[OutilsEnLigne.ca](https://outilsenligne.ca) pour offrir un traitement d’images rapide côté client avec une gamme complète de filtres et de transformations.

## ✨ Fonctionnalités

### 🎨 Ajustements de couleur

- **HSL** – Ajustement de la teinte, saturation et luminosité
- **Luminosité & Contraste** – Réglage précis de la luminance et du contraste
- **Température de couleur** – Balance chaud/froid (1000K-40000K)
- **Vibrance** – Saturation intelligente qui préserve les tons de peau
- **Balance des blancs** – Correction de la température et de la teinte

### 🔄 Opérations de transformation

- **Rogner** – Découpage précis avec détection intelligente
- **Redimensionner** – Redimensionnement de haute qualité avec plusieurs algorithmes (Nearest, Bilinear, CatmullRom, Mitchell, Lanczos3)
- **Rotation** – Rotation à angle arbitraire avec remplissage d’arrière-plan
- **Miroir** – Inversion horizontale et verticale

### 🎭 Filtres artistiques

- **Flou gaussien** – Effet de flou doux avec rayon configurable
- **Flou directionnel** – Flou directionnel avec angle et distance
- **Flou radial** – Effet de flou centré
- **Netteté** – Accentuation de base et masque flou
- **Sépia** – Effet sépia classique
- **Vintage** – Effets style film avec chaleur, vignette, grain et fondu
- **Grain de film** – Simulation réaliste du grain
- **Cross Process** – Plusieurs styles de traitement croisé

### 🔧 Outils de correction

- **Niveaux automatiques** – Correction automatique basée sur l’histogramme
- **Niveaux manuels** – Ajustement des ombres, gamma et hautes lumières
- **Courbes** – Ajustement des courbes pour ombres, tons moyens et hautes lumières
- **Exposition** – Contrôle professionnel de l’exposition avec récupération des hautes lumières/ombres
- **Ombres/hautes lumières** – Récupération sélective des ombres et hautes lumières

### 🖼️ Support SVG

- **Rendu SVG vers raster** – Conversion SVG vers PNG/WebP avec qualité vectorielle
- **Dimensions configurables** – Contrôle précis de la taille de sortie
- **Arrière-plan personnalisable** – Transparence ou couleur de fond au choix
- **Filtres sur SVG** – Application de tous les filtres après rendu vectoriel
- **Détection automatique** – Reconnaissance automatique du format SVG

### 🏷️ Support ICO

- **Génération de favicons** – Création automatique de favicon.ico optimisés
- **Conversion universelle** – Conversion depuis tout format vers ICO
- **Optimisation intelligente** – Redimensionnement et netteté adaptés aux icônes
- **Multi-plateformes** – Génération d'icônes pour Windows, web et applications
- **Tailles standards** – Support des tailles d'icônes courantes (16x16 à 256x256)

### 🎯 Fonctionnalités intelligentes

- **Filtres style Instagram** – Combinaisons de filtres populaires préconfigurées
- **Génération de miniatures** – Création intelligente de vignettes avec options de ratio
- **Prise en charge des formats** – Entrée/sortie PNG, JPEG, WebP, ICO, SVG
- **Efficacité mémoire** – Optimisé pour les grandes images
- **Gestion des erreurs** – Gestion complète avec messages explicites


## 🚀 Démarrage rapide

### Installation

```bash
npm install oelimg-rs
```

### Utilisation de base

```javascript
import init, { WasmImageProcessor } from 'oelimg-rs';

async function processImage() {
  // Initialiser le module WebAssembly
  await init();
  
  // Charger l’image depuis un input fichier ou via fetch
  const fileInput = document.getElementById('imageInput');
  const file = fileInput.files[0];
  const arrayBuffer = await file.arrayBuffer();
  const imageBytes = new Uint8Array(arrayBuffer);
  
  // Créer le processeur
  const processor = WasmImageProcessor.from_bytes(imageBytes);
  console.log(`Image size: ${processor.width}x${processor.height}`);
  
  // Appliquer des filtres
  processor.gaussian_blur(2.0);
  processor.adjust_brightness_contrast(0.1, 1.2);
  processor.adjust_vibrance(15.0);
  processor.resize_fit(800, 600);
  
  // Exporter le résultat
  const processedBytes = processor.to_png_bytes();
  
  // Créer un blob téléchargeable
  const blob = new Blob([processedBytes], { type: 'image/png' });
  const url = URL.createObjectURL(blob);
  
  // Afficher ou télécharger
  const img = document.createElement('img');
  img.src = url;
  document.body.appendChild(img);
}
```

### Traitement SVG

```javascript
import init, { 
  WasmImageProcessor, 
  convert_svg_to_webp, 
  is_svg_format 
} from 'oelimg-rs';

async function processSVG() {
  await init();
  
  // Charger un fichier SVG
  const fileInput = document.getElementById('svgInput');
  const file = fileInput.files[0];
  const arrayBuffer = await file.arrayBuffer();
  const svgBytes = new Uint8Array(arrayBuffer);
  
  // Vérifier que c'est bien un SVG
  if (!is_svg_format(svgBytes)) {
    console.error('Le fichier n\'est pas un SVG valide');
    return;
  }
  
  // Conversion directe vers WebP
  const webpBytes = convert_svg_to_webp(
    svgBytes, 
    800,           // largeur
    600,           // hauteur
    [255, 255, 255, 255] // arrière-plan blanc
  );
  
  // Ou création d'un processeur pour appliquer des filtres
  const processor = WasmImageProcessor.from_svg_bytes(
    svgBytes, 
    1024,          // largeur de rendu
    768            // hauteur de rendu
  );
  
  // Appliquer des effets sur le SVG rendu
  processor.vintage(1.2, 0.3, 0.15, 0.1);
  processor.gaussian_blur(0.5);
  
  const resultBytes = processor.to_webp_bytes();
  
  // Afficher le résultat
  const blob = new Blob([resultBytes], { type: 'image/webp' });
  const url = URL.createObjectURL(blob);
  const img = document.createElement('img');
  img.src = url;
  document.body.appendChild(img);
}
```

### Génération de Favicon

```javascript
import init, { create_favicon, convert_to_ico } from 'oelimg-rs';

async function generateFavicon() {
  await init();
  
  // Charger une image source
  const fileInput = document.getElementById('imageInput');
  const file = fileInput.files[0];
  const arrayBuffer = await file.arrayBuffer();
  const imageBytes = new Uint8Array(arrayBuffer);
  
  // Génération automatique de favicon optimisé
  const faviconBytes = create_favicon(imageBytes);
  
  // Créer un blob téléchargeable
  const blob = new Blob([faviconBytes], { type: 'image/x-icon' });
  const url = URL.createObjectURL(blob);
  
  // Créer lien de téléchargement
  const link = document.createElement('a');
  link.href = url;
  link.download = 'favicon.ico';
  link.textContent = 'Télécharger favicon.ico';
  document.body.appendChild(link);
  
  // Ou conversion simple vers ICO
  const icoBytes = convert_to_ico(imageBytes);
  console.log('Fichier ICO généré:', icoBytes.length, 'bytes');
}
```

### Filtres style Instagram

```javascript
import init, { apply_instagram_filter } from 'oelimg-rs';

await init();

const imageBytes = new Uint8Array(/* vos données d’image */);

// Appliquer des styles de filtres populaires
const clarendon = apply_instagram_filter(imageBytes, 'clarendon'); // Contraste élevé
const gingham = apply_instagram_filter(imageBytes, 'gingham');     // Doux, onirique
const moon = apply_instagram_filter(imageBytes, 'moon');           // Désaturé, froid
const lark = apply_instagram_filter(imageBytes, 'lark');           // Lumineux, vibrant  
const reyes = apply_instagram_filter(imageBytes, 'reyes');         // Style film vintage
```


### Génération de miniatures

```javascript
import init, { create_thumbnail } from 'oelimg-rs';

await init();

const imageBytes = new Uint8Array(/* vos données d’image */);

// Créer une miniature carrée
const thumbnailBytes = create_thumbnail(imageBytes, 150, true);

// Créer une miniature ajustée (conserve le ratio)
const fittedThumbnail = create_thumbnail(imageBytes, 150, false);
```


## 📖 Intégration Next.js

### Hook côté client

```typescript
// hooks/useImageProcessor.ts
import { useImageProcessor } from './hooks/useImageProcessor'; // Voir examples/nextjs-hook.ts

function ImageEditor() {
  const {
    processor,
    isReady,
    error,
    loadImage,
    applyFilter,
    exportImage,
    reset,
    getPreview
  } = useImageProcessor({ maxDimension: 2048, autoResize: true });

  const handleFileLoad = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      await loadImage(file);
    }
  };

  const handleApplyBlur = () => {
    applyFilter('blur', { radius: 3.0 });
  };

  const handleExport = () => {
    const bytes = exportImage('png');
    if (bytes) {
      const blob = new Blob([bytes], { type: 'image/png' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'processed-image.png';
      a.click();
    }
  };

  if (!isReady) return <div>Chargement de WebAssembly...</div>;

  return (
    <div>
      <input type="file" accept="image/*" onChange={handleFileLoad} />
      {error && <div className="error">{error}</div>}
      {processor && (
        <div>
          <img src={getPreview()!} alt="Aperçu" />
          <button onClick={handleApplyBlur}>Appliquer flou</button>
          <button onClick={() => applyFilter('sepia', { intensity: 0.8 })}>
            Appliquer sépia
          </button>
          <button onClick={handleExport}>Exporter PNG</button>
          <button onClick={reset}>Réinitialiser</button>
        </div>
      )}
    </div>
  );
}
```


### Route API côté serveur

```typescript
// pages/api/image/process.ts ou app/api/image/process/route.ts
// Voir examples/nextjs-api-route.ts pour l’implémentation complète

export default async function handler(req, res) {
  const { imageData, operations } = req.body;
  
  // Initialiser WASM
  const { default: init, WasmImageProcessor } = await import('oelimg-rs');
  await init();
  
  // Traiter l’image
  const imageBytes = Buffer.from(imageData.split(',')[1], 'base64');
  const processor = WasmImageProcessor.from_bytes(new Uint8Array(imageBytes));
  
  // Appliquer les opérations
  for (const operation of operations) {
    // Appliquer les filtres selon operation.type et operation.options
  }
  
  // Retourner l’image traitée
  const resultBytes = processor.to_png_bytes();
  const resultBase64 = `data:image/png;base64,${Buffer.from(resultBytes).toString('base64')}`;
  
  res.json({ success: true, imageData: resultBase64 });
}
```


## 🎛️ Référence des filtres

### Ajustements de couleur

#### Ajustement HSL

```javascript
processor.adjust_hsl(
  10.0,  // teinte : -180 à 180 degrés
  1.2,   // saturation : 0 à 2 (1 = aucun changement)
  1.0    // luminosité : 0 à 2 (1 = aucun changement)
);
```

#### Luminosité & Contraste

```javascript
processor.adjust_brightness_contrast(
  0.1,   // luminosité : -1 à 1 (0 = aucun changement)
  1.3    // contraste : 0 à 2 (1 = aucun changement)
);
```

#### Température de couleur

```javascript
processor.adjust_color_temperature(
  3200,  // kelvin : 1000 à 40000 (6500 = lumière du jour)
  0.8    // intensité : 0 à 1 (optionnel, par défaut 1.0)
);
```

#### Vibrance

```javascript
processor.adjust_vibrance(
  25.0   // vibrance : -100 à 100 (0 = aucun changement)
);
```

### Opérations de transformation

#### Rogner

```javascript
processor.crop(
  100,   // x : décalage à gauche
  50,    // y : décalage en haut  
  400,   // largeur
  300    // hauteur
);
```

#### Redimensionner

```javascript
// Redimensionnement exact
processor.resize(800, 600, ResizeAlgorithm.Lanczos3);

// Ajuster dans les limites (conserve le ratio)
processor.resize_fit(800, 600, ResizeAlgorithm.Lanczos3);
```

#### Rotation

```javascript
processor.rotate(
  45.0,                    // angle en degrés
  [255, 255, 255, 255]     // couleur d’arrière-plan [R, G, B, A] (optionnel)
);
```

### Filtres artistiques

#### Flou gaussien

```javascript
processor.gaussian_blur(
  3.0    // rayon : 0 à 100
);
```

#### Netteté

```javascript
processor.sharpen(
  1.5    // intensité : 0 à 10
);
```

#### Sépia

```javascript
processor.sepia(
  0.8    // intensité : 0 à 1
);
```

#### Vintage

```javascript
processor.vintage(
  1.3,   // chaleur : 0 à 2 (1 = neutre)
  0.4,   // vignette : 0 à 1
  0.2,   // grain : 0 à 1
  0.1    // fondu : 0 à 1
);
```

### Outils de correction

#### Niveaux automatiques

```javascript
processor.auto_levels(); // Correction automatique basée sur l’histogramme
```

#### Niveaux manuels

```javascript
processor.adjust_levels(
  0.1,   // ombres : 0 à 1
  0.9,   // gamma : 0.1 à 3.0
  0.95   // hautes lumières : 0 à 1
);
```

#### Exposition

```javascript
processor.adjust_exposure(
  0.5,   // exposition : -5 à 5 (stops)
  -20.0, // hautes lumières : -100 à 0 (récupération)
  30.0   // ombres : 0 à 100 (rehaussement)
);
```


## 🏗️ Compilation à partir des sources

### Prérequis

- [Rust](https://rustup.rs/) (stable le plus récent)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Node.js 16+

### Compilation

```bash
# Cloner le dépôt
git clone https://github.com/outilsenligne/oelimg-rs
cd oelimg-rs

# Installer les dépendances
npm install

# Compiler pour toutes les cibles
./build.sh

# Ou compiler une cible spécifique
npm run build        # Cible web
npm run build:nodejs # Cible Node.js  
npm run build:bundler # Cible bundler

# Compilation développement (plus rapide, taille plus grande)
npm run dev
```

### Structure du projet

```tree
oelimg-rs/
├── src/
│   ├── core/           # Utilitaires de traitement d’image
│   │   ├── image_buffer.rs
│   │   ├── color_space.rs
│   │   └── histogram.rs
│   ├── filters/        # Implémentations de filtres
│   │   ├── color/      # Filtres d’ajustement de couleur
│   │   ├── artistic/   # Filtres d’effets artistiques
│   │   ├── correction/ # Filtres de correction d’image
│   │   └── transform/  # Transformations géométriques
│   ├── utils/          # Fonctions utilitaires
│   ├── wasm.rs         # Interface WebAssembly
│   └── lib.rs          # Point d’entrée principal
├── examples/           # Exemples d’intégration Next.js
├── types/              # Définitions TypeScript
├── pkg/                # Sortie WebAssembly générée
├── Cargo.toml          # Dépendances Rust
├── package.json        # Configuration npm
└── build.sh            # Script de compilation
```


## 🔧 Détails techniques

### Performance

- **Redimensionnement rapide** : Utilise la crate `fast_image_resize` pour un redimensionnement optimisé et de haute qualité
- **Optimisation SIMD** : Exploite les instructions SIMD lorsque disponible
- **Efficacité mémoire** : Opérations en place lorsque possible
- **Optimisé WebAssembly** : Compilation optimisée pour la taille et la vitesse

### Compatibilité navigateur

- **Navigateurs modernes** : Chrome 57+, Firefox 52+, Safari 11+, Edge 16+
- **WebAssembly requis** : Tous les navigateurs modernes prennent en charge WebAssembly
- **Mémoire requise** : Minimum 4 Mo pour le module WebAssembly + données image

### Prise en charge des formats d’image

- **Entrée** : PNG, JPEG, WebP, GIF (statique), BMP, TIFF
- **Sortie** : PNG, JPEG, WebP
- **Espaces couleur** : RGB, RGBA avec gestion correcte de l’alpha
- **Profondeur de bits** : 8 bits par canal (24/32 bits au total)

### Gestion des erreurs

Toutes les opérations retournent `Result<T, JsValue>` avec des messages d’erreur explicites :

- Validation du format d’image
- Vérification des limites de dimensions (max 64MP, 8192px par côté)
- Erreurs d’allocation mémoire
- Plages de paramètres invalides


## 🤝 Contribuer

Pour l'instant , nous n'acceptons pas les contributions. Nous allons considérer cela vers les releases plus officielles.


### Configuration du développement

1. Forkez le dépôt
2. Créez une branche de fonctionnalité : `git checkout -b feature/amazing-feature`
3. Faites vos modifications et ajoutez des tests
4. Lancez les tests : `npm test`
5. Compilez et testez le paquet : `./build.sh`
6. Commitez vos changements : `git commit -m 'Ajout de la fonctionnalité incroyable'`
7. Poussez sur votre branche : `git push origin feature/amazing-feature`
8. Ouvrez une Pull Request


## 📄 Licence

Ce projet est sous licence MIT.

## 🏢 À propos d’OutilsEnLigne.ca

Ce projet est maintenu par [OutilsEnLigne.ca](https://www.outilsenligne.ca)!

### Autres projets

Nous considérons 


## 🙏 Remerciements

- **Communauté Rust** – Pour l’excellent écosystème
- **WebAssembly Working Group** – Pour avoir rendu cela possible
- **Bibliothèques de traitement d’image** :
  - [`image`](https://github.com/image-rs/image) – Traitement d’image en Rust
  - [`fast_image_resize`](https://github.com/Cykooz/fast_image_resize) – Redimensionnement haute performance
  - [`photon-rs`](https://github.com/silvia-odwyer/photon) – Filtres de traitement d’image


##  Soutien

- 🐛 **Quelquechose qui ne marches pas?** : [GitHub Issues](https://github.com/outilsenligne/oelimg-rs/issues)
- 📧 **Courriel** : support@outilsenligne.ca
- 🌐 **Site web** : [outilsenligne.ca](https://outilsenligne.ca)

---

**Fait avec ❤️ par  OutilsEnLigne.ca**