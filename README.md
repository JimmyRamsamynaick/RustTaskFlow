# 🦀 RustTaskFlow

> Une application moderne de gestion de tâches construite avec Rust et React

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/react-18+-blue.svg)](https://reactjs.org/)
[![TypeScript](https://img.shields.io/badge/typescript-5+-blue.svg)](https://www.typescriptlang.org/)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📋 Description

RustTaskFlow est une application complète de gestion de tâches qui combine la performance de Rust pour le backend avec la modernité de React pour le frontend. L'application offre une interface utilisateur intuitive pour créer, organiser et suivre vos tâches avec des fonctionnalités collaboratives en temps réel.

## ✨ Fonctionnalités

### 🔐 Authentification
- Inscription et connexion sécurisées
- Authentification JWT
- Gestion des sessions

### 📝 Gestion des Tâches
- Création, modification et suppression de tâches
- Statuts personnalisables (En attente, En cours, Terminé)
- Priorités (Haute, Moyenne, Basse)
- Dates d'échéance
- Filtrage et recherche avancés

### 👥 Collaboration
- Partage de tâches entre utilisateurs
- Notifications en temps réel
- WebSocket pour les mises à jour instantanées

### 🎨 Interface Moderne
- Design responsive et moderne
- Interface utilisateur intuitive
- Thème sombre/clair (à venir)

## 🏗️ Architecture

### Backend (Rust)
- **Framework**: Axum pour l'API REST
- **Base de données**: SQLite avec SQLx
- **Authentification**: JWT avec bcrypt
- **WebSocket**: Support temps réel
- **Migrations**: Gestion automatique du schéma

### Frontend (React)
- **Framework**: React 18 avec TypeScript
- **Styling**: Tailwind CSS v4
- **Build**: Vite
- **État**: Context API
- **HTTP**: Fetch API

## 🚀 Installation et Démarrage

### Prérequis
- Rust 1.75+
- Node.js 18+
- Docker (optionnel)

### Démarrage Rapide

#### Mode Développement
```bash
# Cloner le repository
git clone https://github.com/votre-username/RustTaskFlow.git
cd RustTaskFlow

# Utiliser le script de démarrage
./start.sh dev
```

#### Mode Production (Docker)
```bash
# Construire et démarrer avec Docker
./start.sh prod
```

### Installation Manuelle

#### Backend
```bash
cd rusttaskflow-web
cp .env.example .env
# Éditer .env avec vos configurations
cargo run
```

#### Frontend
```bash
cd rusttaskflow-frontend
npm install
npm run dev
```

## 🐳 Docker

L'application est entièrement containerisée avec Docker Compose :

```bash
# Construction des images
docker-compose build

# Démarrage des services
docker-compose up -d

# Arrêt des services
docker-compose down
```

## 📚 API Documentation

### Endpoints Principaux

#### Authentification
- `POST /auth/register` - Inscription
- `POST /auth/login` - Connexion
- `POST /auth/logout` - Déconnexion

#### Tâches
- `GET /tasks` - Liste des tâches
- `POST /tasks` - Créer une tâche
- `PUT /tasks/{id}` - Modifier une tâche
- `DELETE /tasks/{id}` - Supprimer une tâche

#### Utilisateurs
- `GET /users/me` - Profil utilisateur
- `PUT /users/me` - Modifier le profil

## 🛠️ Scripts Utiles

```bash
# Démarrage en développement
./start.sh dev

# Démarrage en production
./start.sh prod

# Construction des images Docker
./start.sh build

# Nettoyage complet
./start.sh clean

# Arrêt des services
./start.sh stop
```

## 🔧 Configuration

### Variables d'Environnement

Créez un fichier `.env` dans `rusttaskflow-web/` :

```env
DATABASE_URL=sqlite:tasks.db
JWT_SECRET=your-super-secret-jwt-key
CORS_ORIGIN=http://localhost:5173
RUST_LOG=info
```

## 🧪 Tests

```bash
# Tests backend
cd rusttaskflow-web
cargo test

# Tests frontend
cd rusttaskflow-frontend
npm test
```

## 📁 Structure du Projet

```
RustTaskFlow/
├── rusttaskflow-core/          # Bibliothèque partagée
├── rusttaskflow-cli/           # Interface en ligne de commande
├── rusttaskflow-web/           # API Backend (Rust/Axum)
│   ├── src/
│   │   ├── handlers/           # Gestionnaires de routes
│   │   ├── middleware/         # Middleware personnalisés
│   │   ├── auth.rs            # Logique d'authentification
│   │   ├── database.rs        # Configuration base de données
│   │   └── main.rs            # Point d'entrée
│   ├── migrations/            # Migrations SQL
│   └── Dockerfile
├── rusttaskflow-frontend/      # Frontend (React/TypeScript)
│   ├── src/
│   │   ├── components/        # Composants réutilisables
│   │   ├── pages/            # Pages de l'application
│   │   ├── contexts/         # Contextes React
│   │   ├── services/         # Services API
│   │   └── types/           # Types TypeScript
│   ├── nginx.conf           # Configuration Nginx
│   └── Dockerfile
├── docker-compose.yml         # Orchestration Docker
├── start.sh                  # Script de démarrage
└── README.md
```

## 🤝 Contribution

Les contributions sont les bienvenues ! Voici comment contribuer :

1. Fork le projet
2. Créez votre branche feature (`git checkout -b feature/AmazingFeature`)
3. Committez vos changements (`git commit -m 'Add some AmazingFeature'`)
4. Push vers la branche (`git push origin feature/AmazingFeature`)
5. Ouvrez une Pull Request

## 📝 Roadmap

- [ ] Notifications push
- [ ] Mode hors ligne
- [ ] Thème sombre
- [ ] Application mobile
- [ ] Intégrations tierces
- [ ] Rapports et analytics

## 📄 License

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

## 👨‍💻 Auteur

**Votre Nom**
- GitHub: [@JimmyRamsamynaick](https://github.com/JimmyRamsamynaick)
- Email: jimmyramsamynaick@gmail.com

## 🙏 Remerciements

- [Axum](https://github.com/tokio-rs/axum) pour le framework web Rust
- [React](https://reactjs.org/) pour l'interface utilisateur
- [Tailwind CSS](https://tailwindcss.com/) pour le styling
- [SQLx](https://github.com/launchbadge/sqlx) pour l'accès à la base de données

---

⭐ N'hésitez pas à donner une étoile si ce projet vous a aidé !

## Features

- ✅ **Task Management**: Create, update, complete, and delete tasks
- 🏷️ **Tags & Organization**: Organize tasks with custom tags
- 📊 **Priority Levels**: Set task priorities (Low, Medium, High, Critical)
- 🔍 **Search & Filter**: Find tasks quickly with powerful filtering
- 📈 **Statistics**: Track your productivity with detailed stats
- 💾 **Multiple Storage**: Choose between JSON or SQLite storage
- 🎨 **Modern CLI**: Beautiful colored interface with intuitive commands
- ⏰ **Due Dates**: Set and track task deadlines
- 📤 **Export/Import**: Backup and restore your tasks

## Installation

### From Source

```bash
git clone https://github.com/yourusername/RustTaskFlow.git
cd RustTaskFlow
cargo build --release
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Add a new task
rusttaskflow add "Complete project documentation" --priority high --tags work,docs

# List all tasks
rusttaskflow list

# List tasks with filters
rusttaskflow list --status todo --priority high
rusttaskflow list --tags work

# Complete a task
rusttaskflow complete <task-id>

# Start working on a task
rusttaskflow start <task-id>

# Search tasks
rusttaskflow search "documentation"

# View task details
rusttaskflow show <task-id>

# Edit a task
rusttaskflow edit <task-id> --title "New title" --priority medium

# Add tags to a task
rusttaskflow tag add <task-id> urgent

# Remove tags from a task
rusttaskflow tag remove <task-id> urgent

# View statistics
rusttaskflow stats

# List all available tags
rusttaskflow tags

# Delete a task
rusttaskflow delete <task-id>

# Export tasks
rusttaskflow export tasks.json
rusttaskflow export tasks.csv --format csv

# Import tasks
rusttaskflow import tasks.json

# Clean completed tasks
rusttaskflow clean
```

### Storage Options

By default, RustTaskFlow uses JSON storage. You can switch to SQLite:

```bash
# Use SQLite storage
rusttaskflow --storage sqlite list

# Or set it as default in your shell profile
export RUSTTASKFLOW_STORAGE=sqlite
```

### Configuration

RustTaskFlow stores data in your system's data directory:
- **macOS**: `~/Library/Application Support/RustTaskFlow/`
- **Linux**: `~/.local/share/RustTaskFlow/`
- **Windows**: `%APPDATA%\RustTaskFlow\`

## Task Status

- **Todo**: Task is created but not started
- **InProgress**: Task is currently being worked on
- **Completed**: Task is finished
- **Cancelled**: Task was cancelled

## Priority Levels

- **Low**: Nice to have tasks
- **Medium**: Regular tasks
- **High**: Important tasks
- **Critical**: Urgent tasks that need immediate attention

## Examples

### Project Management Workflow

```bash
# Create project tasks
rusttaskflow add "Setup development environment" --priority high --tags setup,dev
rusttaskflow add "Write unit tests" --priority medium --tags testing,dev
rusttaskflow add "Deploy to production" --priority critical --tags deployment

# Start working on setup
rusttaskflow start 1

# Complete setup and move to testing
rusttaskflow complete 1
rusttaskflow start 2

# Check progress
rusttaskflow stats
rusttaskflow list --status inprogress
```

### Personal Task Management

```bash
# Add personal tasks
rusttaskflow add "Buy groceries" --priority low --tags personal,shopping
rusttaskflow add "Call dentist" --priority medium --tags personal,health --due "2024-01-15"
rusttaskflow add "Prepare presentation" --priority high --tags work,presentation

# Filter by context
rusttaskflow list --tags personal
rusttaskflow list --tags work

# Check overdue tasks
rusttaskflow list --overdue
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Debug Output

```bash
RUST_LOG=debug cargo run -- list
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) 🦀
- CLI powered by [clap](https://github.com/clap-rs/clap)
- Colors by [colored](https://github.com/mackwic/colored)
- Date handling by [chrono](https://github.com/chronotope/chrono)
- Storage with [rusqlite](https://github.com/rusqlite/rusqlite) and [serde](https://github.com/serde-rs/serde)