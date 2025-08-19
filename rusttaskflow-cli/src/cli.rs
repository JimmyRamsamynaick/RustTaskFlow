use crate::task::{Priority, Status};
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand, ValueEnum};
use uuid::Uuid;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "rtf")]
#[command(about = "RustTaskFlow - Un gestionnaire de tâches moderne en ligne de commande")]
#[command(version = "0.1.0")]
#[command(author = "Jimmy Ramsamy-Naick")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Ajouter une nouvelle tâche
    Add {
        /// Titre de la tâche
        title: String,
        /// Description de la tâche
        #[arg(short, long)]
        description: Option<String>,
        /// Priorité de la tâche
        #[arg(short, long, value_enum)]
        priority: Option<CliPriority>,
        /// Tags pour la tâche (séparés par des virgules)
        #[arg(short, long)]
        tags: Option<String>,
        /// Date d'échéance (format: YYYY-MM-DD ou YYYY-MM-DD HH:MM)
        #[arg(short = 'u', long)]
        due: Option<String>,
    },
    /// Lister les tâches
    List {
        /// Filtrer par statut
        #[arg(short, long, value_enum)]
        status: Option<CliStatus>,
        /// Filtrer par priorité
        #[arg(short, long, value_enum)]
        priority: Option<CliPriority>,
        /// Filtrer par tags (séparés par des virgules)
        #[arg(short, long)]
        tags: Option<String>,
        /// Afficher seulement les tâches en retard
        #[arg(long)]
        overdue: bool,
        /// Nombre maximum de tâches à afficher
        #[arg(short, long)]
        limit: Option<usize>,
    },
    /// Marquer une tâche comme terminée
    Complete {
        /// ID de la tâche (peut être partiel)
        id: String,
    },
    /// Démarrer une tâche (marquer comme en cours)
    Start {
        /// ID de la tâche (peut être partiel)
        id: String,
    },
    /// Annuler une tâche
    Cancel {
        /// ID de la tâche (peut être partiel)
        id: String,
    },
    /// Supprimer une tâche
    Delete {
        /// ID de la tâche (peut être partiel)
        id: String,
        /// Forcer la suppression sans confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Modifier une tâche
    Edit {
        /// ID de la tâche (peut être partiel)
        id: String,
        /// Nouveau titre
        #[arg(short, long)]
        title: Option<String>,
        /// Nouvelle description
        #[arg(short, long)]
        description: Option<String>,
        /// Nouvelle priorité
        #[arg(short, long, value_enum)]
        priority: Option<CliPriority>,
        /// Nouvelle date d'échéance
        #[arg(short = 'u', long)]
        due: Option<String>,
    },
    /// Rechercher des tâches
    Search {
        /// Terme de recherche
        query: String,
        /// Nombre maximum de résultats
        #[arg(short, long)]
        limit: Option<usize>,
    },
    /// Afficher les détails d'une tâche
    Show {
        /// ID de la tâche (peut être partiel)
        id: String,
    },
    /// Gérer les tags
    Tag {
        #[command(subcommand)]
        action: TagCommands,
    },
    /// Afficher les statistiques
    Stats,
    /// Lister tous les tags
    Tags,
    /// Exporter les tâches
    Export {
        /// Fichier de sortie
        #[arg(short, long)]
        output: Option<String>,
        /// Format d'export
        #[arg(short, long, value_enum, default_value = "json")]
        format: ExportFormat,
    },
    /// Importer des tâches
    Import {
        /// Fichier d'entrée
        file: String,
        /// Format d'import
        #[arg(short, long, value_enum, default_value = "json")]
        format: ExportFormat,
    },
    /// Nettoyer les tâches terminées
    Clean {
        /// Supprimer les tâches terminées depuis plus de N jours
        #[arg(short, long, default_value = "30")]
        days: u32,
        /// Forcer le nettoyage sans confirmation
        #[arg(short, long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum TagCommands {
    /// Ajouter un tag à une tâche
    Add {
        /// ID de la tâche
        id: String,
        /// Tag à ajouter
        tag: String,
    },
    /// Supprimer un tag d'une tâche
    Remove {
        /// ID de la tâche
        id: String,
        /// Tag à supprimer
        tag: String,
    },
}

#[derive(Clone, ValueEnum)]
pub enum CliStatus {
    Todo,
    InProgress,
    Completed,
    Cancelled,
}

impl From<CliStatus> for Status {
    fn from(cli_status: CliStatus) -> Self {
        match cli_status {
            CliStatus::Todo => Status::Todo,
            CliStatus::InProgress => Status::InProgress,
            CliStatus::Completed => Status::Completed,
            CliStatus::Cancelled => Status::Cancelled,
        }
    }
}

impl From<Status> for CliStatus {
    fn from(status: Status) -> Self {
        match status {
            Status::Todo => CliStatus::Todo,
            Status::InProgress => CliStatus::InProgress,
            Status::Completed => CliStatus::Completed,
            Status::Cancelled => CliStatus::Cancelled,
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum CliPriority {
    Low,
    Medium,
    High,
    Critical,
}

impl From<CliPriority> for Priority {
    fn from(cli_priority: CliPriority) -> Self {
        match cli_priority {
            CliPriority::Low => Priority::Low,
            CliPriority::Medium => Priority::Medium,
            CliPriority::High => Priority::High,
            CliPriority::Critical => Priority::Critical,
        }
    }
}

impl From<Priority> for CliPriority {
    fn from(priority: Priority) -> Self {
        match priority {
            Priority::Low => CliPriority::Low,
            Priority::Medium => CliPriority::Medium,
            Priority::High => CliPriority::High,
            Priority::Critical => CliPriority::Critical,
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum ExportFormat {
    Json,
    Csv,
}

/// Utilitaires pour parser les dates
pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
    // Essaie d'abord le format avec heure
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M") {
        return Ok(dt.and_utc());
    }
    
    // Essaie le format date seulement
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        return Ok(date.and_hms_opt(23, 59, 59).unwrap().and_utc());
    }
    
    // Essaie d'autres formats courants
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(date_str, "%d/%m/%Y %H:%M") {
        return Ok(dt.and_utc());
    }
    
    if let Ok(date) = chrono::NaiveDate::parse_from_str(date_str, "%d/%m/%Y") {
        return Ok(date.and_hms_opt(23, 59, 59).unwrap().and_utc());
    }
    
    Err(anyhow::anyhow!("Format de date invalide. Utilisez DD/MM/YYYY ou DD/MM/YYYY HH:MM"))
}

/// Utilitaires pour parser les tags
pub fn parse_tags(tags_str: &str) -> Vec<String> {
    tags_str
        .split(',')
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty())
        .collect()
}

/// Utilitaires pour trouver une tâche par ID partiel
pub fn find_task_by_partial_id(partial_id: &str, tasks: &[uuid::Uuid]) -> Option<Uuid> {
    let partial_id = partial_id.to_lowercase();
    
    // Recherche exacte d'abord
    if let Ok(uuid) = Uuid::parse_str(&partial_id) {
        if tasks.contains(&uuid) {
            return Some(uuid);
        }
    }
    
    // Recherche par préfixe
    let matches: Vec<&Uuid> = tasks
        .iter()
        .filter(|id| id.to_string().to_lowercase().starts_with(&partial_id))
        .collect();
    
    match matches.len() {
        1 => Some(*matches[0]),
        _ => None, // Aucun match ou plusieurs matches ambigus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        assert!(parse_date("2024-01-15").is_ok());
        assert!(parse_date("2024-01-15 14:30").is_ok());
        assert!(parse_date("15/01/2024").is_ok());
        assert!(parse_date("15/01/2024 14:30").is_ok());
        assert!(parse_date("invalid-date").is_err());
    }

    #[test]
    fn test_parse_tags() {
        let tags = parse_tags("work, urgent, important");
        assert_eq!(tags, vec!["work", "urgent", "important"]);
        
        let tags = parse_tags("single");
        assert_eq!(tags, vec!["single"]);
        
        let tags = parse_tags("");
        assert!(tags.is_empty());
    }

    #[test]
    fn test_find_task_by_partial_id() {
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();
        let tasks = vec![uuid1, uuid2];
        
        // Test avec ID complet
        assert_eq!(find_task_by_partial_id(&uuid1.to_string(), &tasks), Some(uuid1));
        
        // Test avec préfixe
        let prefix = &uuid1.to_string()[..8];
        assert_eq!(find_task_by_partial_id(prefix, &tasks), Some(uuid1));
    }
}