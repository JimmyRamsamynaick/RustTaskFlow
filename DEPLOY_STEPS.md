# Étapes de Déploiement - Guide Rapide

## 🚀 Déploiement du Backend sur Render

### Étape 1: Créer un compte Render
1. Allez sur https://render.com
2. Cliquez sur "Get Started for Free"
3. Connectez-vous avec votre compte GitHub

### Étape 2: Créer un Web Service
1. Dans le dashboard Render, cliquez sur "New +"
2. Sélectionnez "Web Service"
3. Connectez votre repository `RustTaskFlow`
4. Sélectionnez la branche `main`

### Étape 3: Configuration du Service
```
Name: rusttaskflow-backend
Root Directory: rusttaskflow-web
Environment: Rust
Build Command: cargo build --release
Start Command: ./target/release/rusttaskflow-web
```

### Étape 4: Variables d'Environnement
Dans l'onglet "Environment", ajoutez :
```
RUST_LOG=debug
PORT=10000
JWT_SECRET=votre-cle-secrete-jwt-changez-ceci-en-production
CORS_ORIGIN=https://jimmyramsamynaick.github.io
```

### Étape 5: Déployer
1. Cliquez sur "Create Web Service"
2. Attendez que le déploiement se termine (5-10 minutes)
3. Notez l'URL générée (ex: `https://rusttaskflow-backend.onrender.com`)

## 🔧 Configuration GitHub

### Étape 6: Configurer les Secrets GitHub
1. Allez dans votre repository GitHub
2. Settings → Secrets and variables → Actions
3. Cliquez sur "New repository secret"
4. Nom: `VITE_API_BASE_URL`
5. Valeur: L'URL de votre backend Render (ex: `https://rusttaskflow-backend.onrender.com`)

### Étape 7: Redéployer le Frontend
1. Faites un petit changement dans le README ou ajoutez un commentaire
2. Commitez et poussez vers GitHub
3. Le workflow GitHub Actions se déclenchera automatiquement
4. Attendez que le déploiement se termine

## ✅ Test Final

### Étape 8: Tester l'Application
1. Allez sur https://jimmyramsamynaick.github.io/RustTaskFlow/
2. Cliquez sur "Créer un compte"
3. Remplissez le formulaire d'inscription
4. Vérifiez que l'inscription fonctionne sans erreur

---

**Note**: Le premier déploiement sur Render peut prendre du temps car il doit compiler le code Rust. Les déploiements suivants seront plus rapides.

**Problème courant**: Si le backend ne démarre pas, vérifiez les logs dans Render pour identifier les erreurs de compilation ou de configuration.