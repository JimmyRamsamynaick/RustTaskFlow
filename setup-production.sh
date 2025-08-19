#!/bin/bash

# Script de configuration pour la production
# Ce script aide à configurer l'environnement de production

echo "🚀 Configuration de RustTaskFlow pour la production"
echo "================================================="

# Vérifier si nous sommes dans le bon répertoire
if [ ! -f "Cargo.toml" ] || [ ! -d "rusttaskflow-web" ]; then
    echo "❌ Erreur: Ce script doit être exécuté depuis la racine du projet RustTaskFlow"
    exit 1
fi

echo "✅ Répertoire de projet détecté"

# Vérifier les fichiers de configuration
echo "\n📋 Vérification des fichiers de configuration..."

if [ -f "render.yaml" ]; then
    echo "✅ render.yaml trouvé"
else
    echo "❌ render.yaml manquant"
fi

if [ -f "rusttaskflow-frontend/.env.production" ]; then
    echo "✅ .env.production trouvé"
else
    echo "❌ .env.production manquant"
fi

if [ -f "RENDER_DEPLOYMENT.md" ]; then
    echo "✅ Guide de déploiement trouvé"
else
    echo "❌ Guide de déploiement manquant"
fi

# Afficher l'URL actuelle configurée
echo "\n🔧 Configuration actuelle:"
if [ -f "rusttaskflow-frontend/.env.production" ]; then
    echo "URL Backend configurée:"
    grep "VITE_API_BASE_URL" rusttaskflow-frontend/.env.production
fi

echo "\n📝 Prochaines étapes:"
echo "1. Déployez le backend sur Render en suivant DEPLOY_STEPS.md"
echo "2. Notez l'URL générée par Render"
echo "3. Configurez le secret GitHub VITE_API_BASE_URL avec cette URL"
echo "4. Redéployez le frontend via GitHub Actions"
echo "5. Testez l'inscription sur GitHub Pages"

echo "\n🔗 Liens utiles:"
echo "- Render: https://render.com"
echo "- GitHub Secrets: https://github.com/JimmyRamsamynaick/RustTaskFlow/settings/secrets/actions"
echo "- GitHub Pages: https://jimmyramsamynaick.github.io/RustTaskFlow/"

echo "\n✨ Configuration terminée! Suivez les étapes ci-dessus pour déployer."