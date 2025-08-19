# 🤝 Guide de Contribution

Merci de votre intérêt pour contribuer à RustTaskFlow ! Ce guide vous aidera à comprendre comment participer au développement du projet.

## 📋 Table des Matières

- [Code de Conduite](#code-de-conduite)
- [Comment Contribuer](#comment-contribuer)
- [Configuration de l'Environnement](#configuration-de-lenvironnement)
- [Standards de Code](#standards-de-code)
- [Process de Pull Request](#process-de-pull-request)
- [Signalement de Bugs](#signalement-de-bugs)
- [Demandes de Fonctionnalités](#demandes-de-fonctionnalités)

## 📜 Code de Conduite

En participant à ce projet, vous acceptez de respecter notre code de conduite. Soyez respectueux, inclusif et constructif dans toutes vos interactions.

## 🚀 Comment Contribuer

Il existe plusieurs façons de contribuer :

### 🐛 Signaler des Bugs
- Utilisez les GitHub Issues
- Fournissez des informations détaillées
- Incluez les étapes de reproduction

### ✨ Proposer des Fonctionnalités
- Ouvrez une issue pour discuter de l'idée
- Expliquez le cas d'usage
- Proposez une implémentation si possible

### 💻 Contribuer au Code
- Fork le repository
- Créez une branche pour votre fonctionnalité
- Implémentez vos changements
- Ajoutez des tests si nécessaire
- Soumettez une Pull Request

### 📚 Améliorer la Documentation
- Corrigez les erreurs
- Ajoutez des exemples
- Clarifiez les instructions

## ⚙️ Configuration de l'Environnement

### Prérequis
- Rust 1.75+
- Node.js 18+
- Git
- Docker (optionnel)

### Installation

```bash
# 1. Fork et cloner le repository
git clone https://github.com/votre-username/RustTaskFlow.git
cd RustTaskFlow

# 2. Installer les dépendances backend
cd rusttaskflow-web
cargo build

# 3. Installer les dépendances frontend
cd ../rusttaskflow-frontend
npm install

# 4. Démarrer en mode développement
cd ..
./start.sh dev
```

## 📏 Standards de Code

### Backend (Rust)

#### Formatage
```bash
# Formatter le code
cargo fmt

# Vérifier avec Clippy
cargo clippy -- -D warnings
```

#### Conventions
- Utilisez `snake_case` pour les variables et fonctions
- Utilisez `PascalCase` pour les types et structs
- Documentez les fonctions publiques avec `///`
- Gérez les erreurs explicitement

#### Exemple
```rust
/// Crée une nouvelle tâche dans la base de données
/// 
/// # Arguments
/// * `task_data` - Les données de la tâche à créer
/// 
/// # Returns
/// * `Result<Task, DatabaseError>` - La tâche créée ou une erreur
pub async fn create_task(task_data: CreateTaskRequest) -> Result<Task, DatabaseError> {
    // Implémentation...
}
```

### Frontend (React/TypeScript)

#### Formatage
```bash
# Formatter le code
npm run format

# Vérifier le linting
npm run lint
```

#### Conventions
- Utilisez `camelCase` pour les variables et fonctions
- Utilisez `PascalCase` pour les composants
- Préférez les fonctions fléchées
- Utilisez TypeScript strict

#### Exemple
```typescript
interface TaskProps {
  task: Task;
  onUpdate: (task: Task) => void;
}

const TaskItem: React.FC<TaskProps> = ({ task, onUpdate }) => {
  const handleStatusChange = (newStatus: TaskStatus) => {
    onUpdate({ ...task, status: newStatus });
  };

  return (
    <div className="task-item">
      {/* Contenu du composant */}
    </div>
  );
};
```

## 🔄 Process de Pull Request

### 1. Préparation
- Assurez-vous que votre branche est à jour avec `main`
- Testez vos changements localement
- Vérifiez le formatage et le linting

### 2. Création de la PR
- Utilisez un titre descriptif
- Remplissez le template de PR
- Liez les issues concernées
- Ajoutez des captures d'écran si pertinent

### 3. Template de PR
```markdown
## 📝 Description
Brève description des changements apportés.

## 🔗 Issues Liées
- Fixes #123
- Related to #456

## 🧪 Tests
- [ ] Tests unitaires ajoutés/mis à jour
- [ ] Tests d'intégration passent
- [ ] Tests manuels effectués

## 📸 Captures d'écran
(Si applicable)

## ✅ Checklist
- [ ] Code formaté et linté
- [ ] Documentation mise à jour
- [ ] Changements testés
- [ ] Pas de breaking changes non documentés
```

### 4. Review
- Répondez aux commentaires de review
- Effectuez les changements demandés
- Demandez une nouvelle review si nécessaire

## 🐛 Signalement de Bugs

### Template d'Issue Bug
```markdown
## 🐛 Description du Bug
Description claire et concise du problème.

## 🔄 Étapes de Reproduction
1. Aller à '...'
2. Cliquer sur '...'
3. Faire défiler jusqu'à '...'
4. Voir l'erreur

## ✅ Comportement Attendu
Description de ce qui devrait se passer.

## 📸 Captures d'écran
(Si applicable)

## 🖥️ Environnement
- OS: [ex: macOS 14.0]
- Navigateur: [ex: Chrome 120]
- Version: [ex: 1.0.0]

## 📋 Informations Supplémentaires
Tout autre contexte utile.
```

## ✨ Demandes de Fonctionnalités

### Template d'Issue Feature
```markdown
## 🚀 Fonctionnalité Demandée
Description claire de la fonctionnalité souhaitée.

## 💡 Motivation
Pourquoi cette fonctionnalité serait-elle utile ?

## 📋 Solution Proposée
Comment imaginez-vous cette fonctionnalité ?

## 🔄 Alternatives Considérées
Autres solutions envisagées.

## 📸 Mockups/Exemples
(Si applicable)
```

## 🧪 Tests

### Backend
```bash
# Exécuter tous les tests
cargo test

# Tests avec couverture
cargo tarpaulin --out Html
```

### Frontend
```bash
# Tests unitaires
npm test

# Tests avec couverture
npm run test:coverage

# Tests e2e
npm run test:e2e
```

## 📚 Documentation

- Documentez les nouvelles fonctionnalités
- Mettez à jour le README si nécessaire
- Ajoutez des commentaires pour le code complexe
- Utilisez des exemples concrets

## 🎯 Bonnes Pratiques

### Commits
- Utilisez des messages de commit descriptifs
- Préférez les commits atomiques
- Suivez la convention Conventional Commits

```
feat: ajouter la fonctionnalité de filtrage des tâches
fix: corriger le bug de connexion WebSocket
docs: mettre à jour le guide d'installation
test: ajouter des tests pour l'authentification
```

### Branches
- `main` : branche principale stable
- `feature/nom-fonctionnalité` : nouvelles fonctionnalités
- `fix/nom-bug` : corrections de bugs
- `docs/sujet` : améliorations documentation

## 🆘 Besoin d'Aide ?

- Consultez la documentation
- Recherchez dans les issues existantes
- Posez vos questions dans les discussions
- Contactez les mainteneurs

## 🙏 Remerciements

Merci de contribuer à RustTaskFlow ! Chaque contribution, petite ou grande, est appréciée et aide à améliorer le projet pour tous.

---

**Happy Coding! 🦀✨**