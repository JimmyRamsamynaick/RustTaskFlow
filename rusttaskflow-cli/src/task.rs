use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
            Priority::Critical => write!(f, "Critical"),
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Todo,
    InProgress,
    Completed,
    Cancelled,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::InProgress => write!(f, "In Progress"),
            Status::Completed => write!(f, "Completed"),
            Status::Cancelled => write!(f, "Cancelled"),
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Status::Todo
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub priority: Priority,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    /// Crée une nouvelle tâche
    pub fn new(title: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            status: Status::default(),
            priority: Priority::default(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            due_date: None,
            completed_at: None,
        }
    }

    /// Marque la tâche comme terminée
    pub fn complete(&mut self) {
        self.status = Status::Completed;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Marque la tâche comme en cours
    pub fn start(&mut self) {
        self.status = Status::InProgress;
        self.updated_at = Utc::now();
    }

    /// Annule la tâche
    pub fn cancel(&mut self) {
        self.status = Status::Cancelled;
        self.updated_at = Utc::now();
    }

    /// Met à jour le titre
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    /// Met à jour la description
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    /// Met à jour la priorité
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.updated_at = Utc::now();
    }

    /// Ajoute un tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    /// Supprime un tag
    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }

    /// Met à jour la date d'échéance
    pub fn set_due_date(&mut self, due_date: Option<DateTime<Utc>>) {
        self.due_date = due_date;
        self.updated_at = Utc::now();
    }

    /// Vérifie si la tâche est en retard
    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            return due_date < Utc::now() && self.status != Status::Completed;
        }
        false
    }

    /// Vérifie si la tâche correspond aux tags donnés
    pub fn has_tags(&self, tags: &[String]) -> bool {
        tags.iter().all(|tag| self.tags.contains(tag))
    }

    /// Vérifie si la tâche correspond au filtre de texte
    pub fn matches_text(&self, text: &str) -> bool {
        let text_lower = text.to_lowercase();
        self.title.to_lowercase().contains(&text_lower)
            || self
                .description
                .as_ref()
                .map_or(false, |desc| desc.to_lowercase().contains(&text_lower))
            || self.tags.iter().any(|tag| tag.to_lowercase().contains(&text_lower))
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} - {} ({})",
            self.id.to_string()[..8].to_uppercase(),
            self.title,
            self.status,
            self.priority
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task() {
        let task = Task::new("Test task".to_string());
        assert_eq!(task.title, "Test task");
        assert_eq!(task.status, Status::Todo);
        assert_eq!(task.priority, Priority::Medium);
        assert!(task.tags.is_empty());
    }

    #[test]
    fn test_complete_task() {
        let mut task = Task::new("Test task".to_string());
        task.complete();
        assert_eq!(task.status, Status::Completed);
        assert!(task.completed_at.is_some());
    }

    #[test]
    fn test_add_remove_tags() {
        let mut task = Task::new("Test task".to_string());
        task.add_tag("work".to_string());
        task.add_tag("urgent".to_string());
        assert_eq!(task.tags.len(), 2);
        
        task.remove_tag("work");
        assert_eq!(task.tags.len(), 1);
        assert_eq!(task.tags[0], "urgent");
    }

    #[test]
    fn test_text_matching() {
        let mut task = Task::new("Important work task".to_string());
        task.set_description(Some("This is a critical task".to_string()));
        task.add_tag("urgent".to_string());
        
        assert!(task.matches_text("work"));
        assert!(task.matches_text("critical"));
        assert!(task.matches_text("urgent"));
        assert!(!task.matches_text("personal"));
    }
}