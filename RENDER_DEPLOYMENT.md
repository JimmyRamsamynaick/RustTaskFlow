# Déploiement du Backend sur Render

## Problème Identifié

L'erreur "Erreur lors de la création du compte" sur GitHub Pages est causée par le fait que l'application frontend tente de se connecter à `http://localhost:3000`, qui n'est pas accessible en production.

## Solution : Déploiement sur Render

Render est un service cloud gratuit qui permet de déployer facilement des applications Rust.

### Étapes de Déploiement

1. **Créer un compte sur Render**
   - Allez sur https://render.com
   - Créez un compte gratuit
   - Connectez votre compte GitHub

2. **Créer un nouveau Web Service**
   - Cliquez sur "New +" → "Web Service"
   - Connectez votre repository GitHub `RustTaskFlow`
   - Sélectionnez la branche `main`

3. **Configuration du Service**
   - **Name**: `rusttaskflow-backend`
   - **Root Directory**: `rusttaskflow-web`
   - **Environment**: `Rust`
   - **Build Command**: `cargo build --release`
   - **Start Command**: `./target/release/rusttaskflow-web`

4. **Variables d'Environnement**
   Ajoutez ces variables dans l'onglet "Environment":
   ```
   RUST_LOG=debug
   PORT=10000
   JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
   CORS_ORIGIN=https://jimmyramsamynaick.github.io
   ```

5. **Base de Données**
   - Render fournira automatiquement une variable `DATABASE_URL`
   - Ou créez une base de données PostgreSQL séparée sur Render

### Après le Déploiement

1. **Récupérer l'URL du Backend**
   - Une fois déployé, vous obtiendrez une URL comme: `https://rusttaskflow-backend.onrender.com`

2. **Configurer les Secrets GitHub**
   - Allez dans Settings → Secrets and variables → Actions
   - Ajoutez: `VITE_API_BASE_URL` = `https://rusttaskflow-backend.onrender.com`

3. **Redéployer le Frontend**
   - Faites un commit pour déclencher le workflow GitHub Actions
   - Le frontend utilisera maintenant la nouvelle URL backend

### Test

Après le déploiement, testez l'inscription sur https://jimmyramsamynaick.github.io/RustTaskFlow/

## Alternative : Configuration Manuelle

Si vous préférez configurer manuellement :

1. Modifiez `.env.production` avec l'URL réelle :
   ```
   VITE_API_BASE_URL=https://rusttaskflow-backend.onrender.com
   ```

2. Commitez et poussez les changements

3. Le workflow GitHub Actions utilisera automatiquement cette configuration