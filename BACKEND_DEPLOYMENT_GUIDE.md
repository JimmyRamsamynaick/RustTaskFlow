# Guide de déploiement du backend pour GitHub Pages

## Problème identifié

L'application déployée sur GitHub Pages ne peut pas se connecter au backend car :
- Le frontend utilise `http://localhost:3000` comme URL de base de l'API
- Cette URL n'est accessible qu'en développement local
- Sur GitHub Pages, l'application ne peut pas accéder au serveur local

**Résultat :** Les fonctionnalités d'inscription et de connexion ne fonctionnent pas sur GitHub Pages.

## Solutions recommandées

### Option 1: Déploiement sur Railway (Recommandé)

Railway est gratuit pour les petits projets et supporte Rust nativement.

#### Étapes :
1. Créer un compte sur [Railway.app](https://railway.app)
2. Connecter votre repository GitHub
3. Déployer le service `rusttaskflow-web`
4. Configurer les variables d'environnement
5. Obtenir l'URL de production

### Option 2: Déploiement sur Render

Render offre également un plan gratuit pour les applications Rust.

#### Étapes :
1. Créer un compte sur [Render.com](https://render.com)
2. Créer un nouveau Web Service
3. Connecter votre repository
4. Configurer le build avec Rust
5. Déployer et obtenir l'URL

### Option 3: Déploiement sur Fly.io

Fly.io est optimisé pour les applications modernes.

#### Étapes :
1. Installer Fly CLI
2. Créer un compte Fly.io
3. Initialiser l'application avec `flyctl launch`
4. Déployer avec `flyctl deploy`

## Configuration après déploiement

### 1. Créer le fichier .env.production

```bash
# Dans rusttaskflow-frontend/
VITE_API_BASE_URL=https://votre-backend-url.railway.app
```

### 2. Modifier le workflow GitHub Actions

Ajouter la configuration d'environnement dans `.github/workflows/deploy.yml` :

```yaml
- name: Build
  env:
    VITE_API_BASE_URL: ${{ secrets.VITE_API_BASE_URL }}
  run: |
    cd rusttaskflow-frontend
    npm run build
```

### 3. Configurer les secrets GitHub

1. Aller dans Settings > Secrets and variables > Actions
2. Ajouter `VITE_API_BASE_URL` avec l'URL de votre backend déployé

## Configuration du backend pour CORS

Assurez-vous que le backend accepte les requêtes depuis GitHub Pages :

```rust
// Dans rusttaskflow-web/src/main.rs
let cors = CorsLayer::new()
    .allow_origin("https://jimmyramsamynaick.github.io".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([AUTHORIZATION, CONTENT_TYPE]);
```

## Base de données

Pour la production, vous aurez besoin d'une base de données :
- **Railway** : PostgreSQL gratuit inclus
- **Render** : PostgreSQL gratuit disponible
- **Supabase** : Base de données PostgreSQL gratuite

## Étapes suivantes

1. ✅ Choisir un service de déploiement
2. ⏳ Déployer le backend Rust
3. ⏳ Configurer les variables d'environnement
4. ⏳ Mettre à jour le workflow GitHub Actions
5. ⏳ Tester l'inscription/connexion sur GitHub Pages

## Coûts estimés

- **Railway** : Gratuit jusqu'à 500h/mois
- **Render** : Gratuit avec limitations
- **Fly.io** : Gratuit jusqu'à 3 applications

Tous ces services offrent des plans gratuits suffisants pour un projet de démonstration.