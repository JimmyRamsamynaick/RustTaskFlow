use crate::task::{Priority, Status, Task};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TaskFilter {
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub tags: Vec<String>,
    pub text: Option<String>,
    pub overdue_only: bool,
}

impl Default for TaskFilter {
    fn default() -> Self {
        Self {
            status: None,
            priority: None,
            tags: Vec::new(),
            text: None,
            overdue_only: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TaskStats {
    pub total: usize,
    pub todo: usize,
    pub in_progress: usize,
    pub completed: usize,
    pub cancelled: usize,
    pub overdue: usize,
}

#[derive(Debug)]
pub struct TaskManager {
    tasks: HashMap<Uuid, Task>,
}

impl TaskManager {
    /// Crée un nouveau gestionnaire de tâches
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }

    /// Ajoute une nouvelle tâche
    pub fn add_task(&mut self, title: String) -> Uuid {
        let task = Task::new(title);
        let id = task.id;
        self.tasks.insert(id, task);
        id
    }

    /// Récupère une tâche par son ID
    pub fn get_task(&self, id: &Uuid) -> Option<&Task> {
        self.tasks.get(id)
    }

    /// Récupère une tâche mutable par son ID
    pub fn get_task_mut(&mut self, id: &Uuid) -> Option<&mut Task> {
        self.tasks.get_mut(id)
    }

    /// Supprime une tâche
    pub fn delete_task(&mut self, id: &Uuid) -> Result<Task> {
        self.tasks
            .remove(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))
    }

    /// Met à jour le titre d'une tâche
    pub fn update_task_title(&mut self, id: &Uuid, title: String) -> Result<()> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))?;
        task.set_title(title);
        Ok(())
    }

    /// Met à jour la description d'une tâche
    pub fn update_task_description(&mut self, id: &Uuid, description: Option<String>) -> Result<()> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))?;
        task.set_description(description);
        Ok(())
    }

    /// Met à jour la priorité d'une tâche
    pub fn update_task_priority(&mut self, id: &Uuid, priority: Priority) -> Result<()> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))?;
        task.set_priority(priority);
        Ok(())
    }

    /// Marque une tâche comme terminée
    pub fn complete_task(&mut self, id: &Uuid) -> Result<()> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))?;
        task.complete();
        Ok(())
    }

    /// Marque une tâche comme en cours
    pub fn start_task(&mut self, id: &Uuid) -> Result<()> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))?;
        task.start();
        Ok(())
    }

    /// Annule une tâche
    pub fn cancel_task(&mut self, id: &Uuid) -> Result<()> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))?;
        task.cancel();
        Ok(())
    }

    /// Ajoute un tag à une tâche
    pub fn add_tag_to_task(&mut self, id: &Uuid, tag: String) -> Result<()> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))?;
        task.add_tag(tag);
        Ok(())
    }

    /// Supprime un tag d'une tâche
    pub fn remove_tag_from_task(&mut self, id: &Uuid, tag: &str) -> Result<()> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))?;
        task.remove_tag(tag);
        Ok(())
    }

    /// Met à jour la date d'échéance d'une tâche
    pub fn set_task_due_date(&mut self, id: &Uuid, due_date: Option<DateTime<Utc>>) -> Result<()> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| anyhow!("Tâche avec l'ID {} non trouvée", id))?;
        task.set_due_date(due_date);
        Ok(())
    }

    /// Récupère toutes les tâches
    pub fn get_all_tasks(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    /// Filtre les tâches selon les critères donnés
    pub fn filter_tasks(&self, filter: &TaskFilter) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| {
                // Filtre par statut
                if let Some(status) = &filter.status {
                    if &task.status != status {
                        return false;
                    }
                }

                // Filtre par priorité
                if let Some(priority) = &filter.priority {
                    if &task.priority != priority {
                        return false;
                    }
                }

                // Filtre par tags
                if !filter.tags.is_empty() && !task.has_tags(&filter.tags) {
                    return false;
                }

                // Filtre par texte
                if let Some(text) = &filter.text {
                    if !task.matches_text(text) {
                        return false;
                    }
                }

                // Filtre par tâches en retard
                if filter.overdue_only && !task.is_overdue() {
                    return false;
                }

                true
            })
            .collect()
    }

    /// Recherche des tâches par texte
    pub fn search_tasks(&self, query: &str) -> Vec<&Task> {
        let filter = TaskFilter {
            text: Some(query.to_string()),
            ..Default::default()
        };
        self.filter_tasks(&filter)
    }

    /// Récupère les tâches en retard
    pub fn get_overdue_tasks(&self) -> Vec<&Task> {
        let filter = TaskFilter {
            overdue_only: true,
            ..Default::default()
        };
        self.filter_tasks(&filter)
    }

    /// Récupère les tâches par statut
    pub fn get_tasks_by_status(&self, status: Status) -> Vec<&Task> {
        let filter = TaskFilter {
            status: Some(status),
            ..Default::default()
        };
        self.filter_tasks(&filter)
    }

    /// Récupère les tâches par priorité
    pub fn get_tasks_by_priority(&self, priority: Priority) -> Vec<&Task> {
        let filter = TaskFilter {
            priority: Some(priority),
            ..Default::default()
        };
        self.filter_tasks(&filter)
    }

    /// Récupère les tâches par tags
    pub fn get_tasks_by_tags(&self, tags: Vec<String>) -> Vec<&Task> {
        let filter = TaskFilter {
            tags,
            ..Default::default()
        };
        self.filter_tasks(&filter)
    }

    /// Génère des statistiques sur les tâches
    pub fn get_stats(&self) -> TaskStats {
        let mut stats = TaskStats {
            total: self.tasks.len(),
            todo: 0,
            in_progress: 0,
            completed: 0,
            cancelled: 0,
            overdue: 0,
        };

        for task in self.tasks.values() {
            match task.status {
                Status::Todo => stats.todo += 1,
                Status::InProgress => stats.in_progress += 1,
                Status::Completed => stats.completed += 1,
                Status::Cancelled => stats.cancelled += 1,
            }

            if task.is_overdue() {
                stats.overdue += 1;
            }
        }

        stats
    }

    /// Récupère tous les tags uniques
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self
            .tasks
            .values()
            .flat_map(|task| task.tags.iter())
            .cloned()
            .collect();
        tags.sort();
        tags.dedup();
        tags
    }

    /// Compte le nombre de tâches
    pub fn count_tasks(&self) -> usize {
        self.tasks.len()
    }

    /// Vide toutes les tâches
    pub fn clear_all_tasks(&mut self) {
        self.tasks.clear();
    }

    /// Charge les tâches depuis un HashMap
    pub fn load_tasks(&mut self, tasks: HashMap<Uuid, Task>) {
        self.tasks = tasks;
    }

    /// Exporte toutes les tâches
    pub fn export_tasks(&self) -> &HashMap<Uuid, Task> {
        &self.tasks
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_task() {
        let mut manager = TaskManager::new();
        let id = manager.add_task("Test task".to_string());
        
        let task = manager.get_task(&id).unwrap();
        assert_eq!(task.title, "Test task");
        assert_eq!(task.status, Status::Todo);
    }

    #[test]
    fn test_complete_task() {
        let mut manager = TaskManager::new();
        let id = manager.add_task("Test task".to_string());
        
        manager.complete_task(&id).unwrap();
        let task = manager.get_task(&id).unwrap();
        assert_eq!(task.status, Status::Completed);
    }

    #[test]
    fn test_filter_tasks() {
        let mut manager = TaskManager::new();
        let id1 = manager.add_task("Task 1".to_string());
        let id2 = manager.add_task("Task 2".to_string());
        
        manager.complete_task(&id1).unwrap();
        
        let completed_tasks = manager.get_tasks_by_status(Status::Completed);
        assert_eq!(completed_tasks.len(), 1);
        assert_eq!(completed_tasks[0].title, "Task 1");
        
        let todo_tasks = manager.get_tasks_by_status(Status::Todo);
        assert_eq!(todo_tasks.len(), 1);
        assert_eq!(todo_tasks[0].title, "Task 2");
    }

    #[test]
    fn test_search_tasks() {
        let mut manager = TaskManager::new();
        manager.add_task("Important work task".to_string());
        manager.add_task("Personal shopping".to_string());
        
        let work_tasks = manager.search_tasks("work");
        assert_eq!(work_tasks.len(), 1);
        assert_eq!(work_tasks[0].title, "Important work task");
    }

    #[test]
    fn test_stats() {
        let mut manager = TaskManager::new();
        let id1 = manager.add_task("Task 1".to_string());
        let id2 = manager.add_task("Task 2".to_string());
        let id3 = manager.add_task("Task 3".to_string());
        
        manager.complete_task(&id1).unwrap();
        manager.start_task(&id2).unwrap();
        
        let stats = manager.get_stats();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.completed, 1);
        assert_eq!(stats.in_progress, 1);
        assert_eq!(stats.todo, 1);
    }
}