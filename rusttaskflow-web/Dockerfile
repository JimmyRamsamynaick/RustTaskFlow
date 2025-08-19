# Dockerfile pour le backend Rust
FROM rust:1.75 as builder

WORKDIR /app

# Copier les fichiers de configuration Cargo
COPY Cargo.toml Cargo.lock ./
COPY ../rusttaskflow-core/Cargo.toml ../rusttaskflow-core/

# Créer des répertoires src vides pour la compilation des dépendances
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN mkdir ../rusttaskflow-core/src && echo "" > ../rusttaskflow-core/src/lib.rs

# Compiler les dépendances (cette étape sera mise en cache)
RUN cargo build --release
RUN rm src/main.rs ../rusttaskflow-core/src/lib.rs

# Copier le code source
COPY src ./src
COPY ../rusttaskflow-core/src ../rusttaskflow-core/src
COPY migrations ./migrations

# Compiler l'application
RUN cargo build --release

# Image de production
FROM debian:bookworm-slim

# Installer les dépendances système
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Créer un utilisateur non-root
RUN useradd -r -s /bin/false appuser

# Créer le répertoire de l'application
WORKDIR /app

# Copier l'exécutable depuis l'étape de build
COPY --from=builder /app/target/release/rusttaskflow-web /app/
COPY --from=builder /app/migrations /app/migrations

# Changer le propriétaire des fichiers
RUN chown -R appuser:appuser /app

# Utiliser l'utilisateur non-root
USER appuser

# Exposer le port
EXPOSE 3000

# Variables d'environnement
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:///app/data/tasks.db

# Créer le répertoire pour la base de données
RUN mkdir -p /app/data

# Commande de démarrage
CMD ["./rusttaskflow-web"]