# Configuration GitHub Pages avec Backend Render

## ✅ Modifications effectuées

### 1. Configuration des environnements
- **`.env.local`** : Créé pour le développement local (http://localhost:3000)
- **`.env.production`** : Mis à jour avec l'URL Render (https://rusttaskflow-backend.onrender.com)
- **`vite.config.ts`** : Configuré pour utiliser la bonne base URL selon l'environnement

### 2. Workflow GitHub Actions
- Mis à jour pour utiliser le mode production
- Configuré pour utiliser l'URL Render par défaut
- Support des secrets GitHub pour l'URL de l'API

## 🔧 Étapes de finalisation

### Étape 1: Configurer les secrets GitHub (Optionnel)

1. Allez sur votre repository GitHub : https://github.com/JimmyRamsamynaick/RustTaskFlow
2. Cliquez sur **Settings** → **Secrets and variables** → **Actions**
3. Cliquez sur **New repository secret**
4. Nom : `VITE_API_BASE_URL`
5. Valeur : `https://rusttaskflow-backend.onrender.com`
6. Cliquez sur **Add secret**

> **Note :** Cette étape est optionnelle car l'URL Render est déjà configurée par défaut dans le workflow.

### Étape 2: Configurer GitHub Pages

1. Allez dans **Settings** → **Pages**
2. Sous "Build and deployment" → "Source"
3. Sélectionnez **"GitHub Actions"** (pas "Deploy from a branch")
4. Cliquez sur **Save**

### Étape 3: Déclencher le déploiement

1. Faites un commit et push de ces modifications :
   ```bash
   git add .
   git commit -m "Configure frontend for Render backend"
   git push origin main
   ```

2. Le workflow GitHub Actions se déclenchera automatiquement
3. Attendez que le déploiement se termine (5-10 minutes)

### Étape 4: Tester l'application

1. Allez sur : https://jimmyramsamynaick.github.io/RustTaskFlow/
2. Cliquez sur **"Créer un compte"**
3. Remplissez le formulaire d'inscription
4. Vérifiez que l'inscription fonctionne sans erreur

## 🔍 Vérification

### Vérifier le déploiement
1. Allez dans l'onglet **Actions** de votre repository
2. Vérifiez que le workflow "Deploy React App to GitHub Pages" s'exécute avec succès
3. Consultez les logs en cas d'erreur

### Vérifier la configuration
- **Développement local** : `npm run dev` utilisera `http://localhost:3000`
- **Production GitHub Pages** : utilisera `https://rusttaskflow-backend.onrender.com`

## 🚨 Dépannage

### Si l'inscription ne fonctionne toujours pas :

1. **Vérifiez les logs du backend Render :**
   - Allez sur votre dashboard Render
   - Cliquez sur votre service backend
   - Consultez les logs pour voir les erreurs

2. **Vérifiez la configuration CORS du backend :**
   - Le backend doit autoriser les requêtes depuis `https://jimmyramsamynaick.github.io`

3. **Vérifiez la console du navigateur :**
   - Ouvrez les outils de développement (F12)
   - Consultez l'onglet Console pour voir les erreurs JavaScript
   - Consultez l'onglet Network pour voir les requêtes API

### Erreurs courantes :

- **CORS Error** : Le backend n'autorise pas les requêtes depuis GitHub Pages
- **404 Not Found** : L'URL du backend est incorrecte
- **500 Internal Server Error** : Erreur dans le code backend

## 📝 Résumé

Après ces modifications :
- ✅ Le frontend est configuré pour utiliser votre backend Render
- ✅ Les environnements de développement et production sont séparés
- ✅ Le workflow GitHub Actions est optimisé
- ✅ L'application devrait fonctionner sur GitHub Pages

**URL de votre application :** https://jimmyramsamynaick.github.io/RustTaskFlow/