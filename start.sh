#!/bin/bash

# Script de démarrage pour RustTaskFlow
# Usage: ./start.sh [dev|prod|build|clean]

set -e

command="${1:-dev}"

case $command in
  "dev")
    echo "🚀 Démarrage en mode développement..."
    echo "📦 Installation des dépendances frontend..."
    cd rusttaskflow-frontend && npm install && cd ..
    
    echo "🔧 Compilation du backend..."
    cd rusttaskflow-web && cargo build && cd ..
    
    echo "🌐 Démarrage des serveurs..."
    # Démarrer le backend en arrière-plan
    cd rusttaskflow-web && RUST_LOG=debug cargo run &
    BACKEND_PID=$!
    
    # Attendre que le backend démarre
    sleep 3
    
    # Démarrer le frontend
    cd ../rusttaskflow-frontend && npm run dev &
    FRONTEND_PID=$!
    
    echo "✅ Serveurs démarrés:"
    echo "   - Backend: http://localhost:3000"
    echo "   - Frontend: http://localhost:5173"
    echo "   - Appuyez sur Ctrl+C pour arrêter"
    
    # Attendre l'interruption
    trap "kill $BACKEND_PID $FRONTEND_PID 2>/dev/null; exit" INT
    wait
    ;;
    
  "prod")
    echo "🐳 Démarrage en mode production avec Docker..."
    docker-compose up -d
    echo "✅ Application démarrée sur http://localhost"
    ;;
    
  "build")
    echo "🔨 Construction des images Docker..."
    docker-compose build
    echo "✅ Images construites avec succès"
    ;;
    
  "clean")
    echo "🧹 Nettoyage..."
    docker-compose down -v
    docker system prune -f
    cd rusttaskflow-web && cargo clean && cd ..
    cd rusttaskflow-frontend && rm -rf node_modules dist && cd ..
    echo "✅ Nettoyage terminé"
    ;;
    
  "stop")
    echo "🛑 Arrêt des services..."
    docker-compose down
    echo "✅ Services arrêtés"
    ;;
    
  *)
    echo "Usage: $0 [dev|prod|build|clean|stop]"
    echo "  dev   - Démarrage en mode développement"
    echo "  prod  - Démarrage en mode production (Docker)"
    echo "  build - Construction des images Docker"
    echo "  clean - Nettoyage complet"
    echo "  stop  - Arrêt des services Docker"
    exit 1
    ;;
esac