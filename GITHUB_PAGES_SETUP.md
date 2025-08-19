# Configuration GitHub Pages pour RustTaskFlow

## Problème actuel
Le site GitHub Pages affiche actuellement le README au lieu de l'application React. Cela est dû à une configuration incorrecte des paramètres GitHub Pages.

## Solution

Pour résoudre ce problème, vous devez configurer GitHub Pages pour utiliser GitHub Actions au lieu du dossier `docs` :

### Étapes à suivre :

1. **Accédez aux paramètres du repository :**
   - Allez sur https://github.com/JimmyRamsamynaick/RustTaskFlow
   - Cliquez sur l'onglet "Settings"

2. **Configurez GitHub Pages :**
   - Dans la barre latérale, cliquez sur "Pages"
   - Sous "Build and deployment", dans la section "Source"
   - **Changez de "Deploy from a branch" vers "GitHub Actions"**

3. **Sauvegardez les modifications :**
   - Cliquez sur "Save"

### Pourquoi ce changement est nécessaire :

- **Actuellement :** GitHub Pages est configuré pour déployer depuis le dossier `docs` de la branche `main`
- **Problème :** Le dossier `docs` contient les anciens fichiers statiques qui ne se mettent pas à jour automatiquement
- **Solution :** Utiliser GitHub Actions permet un déploiement automatique à chaque push sur `main`

### Workflow GitHub Actions

Un workflow GitHub Actions a été ajouté (`.github/workflows/deploy.yml`) qui :
- Se déclenche automatiquement à chaque push sur `main`
- Installe les dépendances Node.js
- Construit l'application React
- Déploie automatiquement sur GitHub Pages

### Après la configuration

Une fois la configuration changée :
1. Le workflow se déclenchera automatiquement
2. L'application React sera construite et déployée
3. Le site sera accessible à l'adresse : https://jimmyramsamynaick.github.io/RustTaskFlow/

### Vérification

Pour vérifier que le déploiement fonctionne :
1. Allez dans l'onglet "Actions" du repository
2. Vérifiez que le workflow "Deploy React App to GitHub Pages" s'exécute avec succès
3. Une fois terminé, visitez l'URL du site

---

**Note :** Cette configuration permettra un déploiement automatique et continu de l'application React sans intervention manuelle.