use crate::task::{Priority, Status, Task};
use crate::task_manager::TaskStats;
use chrono::{DateTime, Local, Utc};
use colored::*;


pub struct TaskDisplay;

impl TaskDisplay {
    /// Affiche une liste de tâches dans un format tabulaire
    pub fn print_task_list(tasks: &[&Task]) {
        if tasks.is_empty() {
            return;
        }

        println!();
        println!(
            "{:<10} {:<30} {:<12} {:<10} {:<15} {:<10}",
            "ID".bold().underline(),
            "TITRE".bold().underline(),
            "STATUT".bold().underline(),
            "PRIORITÉ".bold().underline(),
            "TAGS".bold().underline(),
            "ÉCHÉANCE".bold().underline()
        );
        println!("{}", "─".repeat(90).bright_black());

        for task in tasks {
            let id_short = task.id.to_string()[..8].to_uppercase();
            let title = if task.title.len() > 28 {
                format!("{}...", &task.title[..25])
            } else {
                task.title.clone()
            };

            let status_colored = Self::colorize_status(&task.status);
            let priority_colored = Self::colorize_priority(&task.priority);
            
            let tags_str = if task.tags.is_empty() {
                "-".bright_black().to_string()
            } else if task.tags.len() <= 2 {
                task.tags.join(", ").bright_blue().to_string()
            } else {
                format!(
                    "{}, +{}",
                    task.tags[..2].join(", ").bright_blue(),
                    (task.tags.len() - 2).to_string().bright_black()
                )
            };

            let due_str = if let Some(due_date) = task.due_date {
                let local_due = due_date.with_timezone(&Local);
                let now = Utc::now();
                
                if due_date < now && task.status != Status::Completed {
                    format!(
                        "{} {}",
                        "⚠".red(),
                        local_due.format("%m/%d").to_string().red()
                    )
                } else {
                    local_due.format("%m/%d").to_string().normal().to_string()
                }
            } else {
                "-".bright_black().to_string()
            };

            println!(
                "{:<10} {:<30} {:<20} {:<18} {:<25} {:<10}",
                id_short.bright_blue(),
                title,
                status_colored,
                priority_colored,
                tags_str,
                due_str
            );
        }
        
        println!();
        println!(
            "{} {} tâche(s) affichée(s)",
            "📋".bright_blue(),
            tasks.len().to_string().bright_white().bold()
        );
    }

    /// Affiche les détails complets d'une tâche
    pub fn print_task_details(task: &Task) {
        println!();
        println!("{}", "═".repeat(60).bright_blue());
        println!(
            "{} {}",
            "📋 DÉTAILS DE LA TÂCHE".bright_blue().bold(),
            format!("[{}]", task.id.to_string()[..8].to_uppercase()).bright_black()
        );
        println!("{}", "═".repeat(60).bright_blue());
        println!();

        // Titre
        println!(
            "{:<15} {}",
            "Titre:".bold(),
            task.title.bright_white().bold()
        );

        // Description
        if let Some(description) = &task.description {
            println!(
                "{:<15} {}",
                "Description:".bold(),
                description.normal()
            );
        }

        // Statut et priorité
        println!(
            "{:<15} {}",
            "Statut:".bold(),
            Self::colorize_status(&task.status)
        );
        println!(
            "{:<15} {}",
            "Priorité:".bold(),
            Self::colorize_priority(&task.priority)
        );

        // Tags
        if !task.tags.is_empty() {
            println!(
                "{:<15} {}",
                "Tags:".bold(),
                task.tags
                    .iter()
                    .map(|tag| format!("#{}", tag.bright_blue()))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }

        // Dates
        println!();
        println!(
            "{:<15} {}",
            "Créée le:".bold(),
            task.created_at
                .with_timezone(&Local)
                .format("%d/%m/%Y à %H:%M")
                .to_string()
                .bright_black()
        );
        println!(
            "{:<15} {}",
            "Modifiée le:".bold(),
            task.updated_at
                .with_timezone(&Local)
                .format("%d/%m/%Y à %H:%M")
                .to_string()
                .bright_black()
        );

        if let Some(due_date) = task.due_date {
            let local_due = due_date.with_timezone(&Local);
            let now = Utc::now();
            let due_str = if due_date < now && task.status != Status::Completed {
                format!(
                    "{} {}",
                    "⚠ EN RETARD:".red().bold(),
                    local_due.format("%d/%m/%Y à %H:%M").to_string().red()
                )
            } else {
                local_due.format("%d/%m/%Y à %H:%M").to_string().normal().to_string()
            };
            
            println!(
                "{:<15} {}",
                "Échéance:".bold(),
                due_str
            );
        }

        if let Some(completed_at) = task.completed_at {
            println!(
                "{:<15} {}",
                "Terminée le:".bold(),
                completed_at
                    .with_timezone(&Local)
                    .format("%d/%m/%Y à %H:%M")
                    .to_string()
                    .green()
            );
        }

        println!();
        println!("{}", "═".repeat(60).bright_blue());
    }

    /// Affiche les statistiques des tâches
    pub fn print_stats(stats: &TaskStats) {
        println!();
        println!("{}", "═".repeat(50).bright_blue());
        println!(
            "{} {}",
            "📊 STATISTIQUES".bright_blue().bold(),
            "RustTaskFlow".bright_black()
        );
        println!("{}", "═".repeat(50).bright_blue());
        println!();

        // Statistiques générales
        println!(
            "{:<20} {}",
            "Total des tâches:".bold(),
            stats.total.to_string().bright_white().bold()
        );
        println!();

        // Répartition par statut
        println!("{}", "Répartition par statut:".bold().underline());
        println!(
            "  {:<15} {}",
            "À faire:".normal(),
            format!(
                "{} ({}%)",
                stats.todo.to_string().bright_yellow(),
                Self::percentage(stats.todo, stats.total).bright_black()
            )
        );
        println!(
            "  {:<15} {}",
            "En cours:".normal(),
            format!(
                "{} ({}%)",
                stats.in_progress.to_string().bright_blue(),
                Self::percentage(stats.in_progress, stats.total).bright_black()
            )
        );
        println!(
            "  {:<15} {}",
            "Terminées:".normal(),
            format!(
                "{} ({}%)",
                stats.completed.to_string().bright_green(),
                Self::percentage(stats.completed, stats.total).bright_black()
            )
        );
        println!(
            "  {:<15} {}",
            "Annulées:".normal(),
            format!(
                "{} ({}%)",
                stats.cancelled.to_string().bright_red(),
                Self::percentage(stats.cancelled, stats.total).bright_black()
            )
        );
        println!();

        // Tâches en retard
        if stats.overdue > 0 {
            println!(
                "{} {} tâche(s) en retard",
                "⚠".red().bold(),
                stats.overdue.to_string().red().bold()
            );
        } else {
            println!(
                "{} Aucune tâche en retard",
                "✓".green().bold()
            );
        }

        // Barre de progression
        if stats.total > 0 {
            println!();
            let progress = (stats.completed as f64 / stats.total as f64 * 100.0) as usize;
            let bar_length = 30;
            let filled = (progress * bar_length / 100).min(bar_length);
            let empty = bar_length - filled;
            
            println!(
                "{} [{}{}] {}%",
                "Progression:".bold(),
                "█".repeat(filled).green(),
                "░".repeat(empty).bright_black(),
                progress.to_string().bright_white().bold()
            );
        }

        println!();
        println!("{}", "═".repeat(50).bright_blue());
    }

    /// Colorie le statut selon sa valeur
    fn colorize_status(status: &Status) -> ColoredString {
        match status {
            Status::Todo => "📝 À faire".bright_yellow(),
            Status::InProgress => "⚡ En cours".bright_blue(),
            Status::Completed => "✅ Terminé".bright_green(),
            Status::Cancelled => "❌ Annulé".bright_red(),
        }
    }

    /// Colorie la priorité selon sa valeur
    fn colorize_priority(priority: &Priority) -> ColoredString {
        match priority {
            Priority::Low => "🟢 Faible".green(),
            Priority::Medium => "🟡 Moyenne".yellow(),
            Priority::High => "🟠 Élevée".bright_red(),
            Priority::Critical => "🔴 Critique".red().bold(),
        }
    }

    /// Calcule le pourcentage
    fn percentage(value: usize, total: usize) -> String {
        if total == 0 {
            "0".to_string()
        } else {
            format!("{:.1}", (value as f64 / total as f64) * 100.0)
        }
    }

    /// Affiche un message de succès
    pub fn success(message: &str) {
        println!("{} {}", "✓".green().bold(), message);
    }

    /// Affiche un message d'erreur
    pub fn error(message: &str) {
        eprintln!("{} {}", "✗".red().bold(), message);
    }

    /// Affiche un message d'avertissement
    pub fn warning(message: &str) {
        println!("{} {}", "⚠".yellow().bold(), message);
    }

    /// Affiche un message d'information
    pub fn info(message: &str) {
        println!("{} {}", "ℹ".blue().bold(), message);
    }
}

/// Trait pour formater les durées de manière lisible
pub trait DurationFormat {
    fn human_duration(&self) -> String;
}

impl DurationFormat for chrono::Duration {
    fn human_duration(&self) -> String {
        let total_seconds = self.num_seconds().abs();
        
        if total_seconds < 60 {
            format!("{}s", total_seconds)
        } else if total_seconds < 3600 {
            format!("{}m", total_seconds / 60)
        } else if total_seconds < 86400 {
            format!("{}h", total_seconds / 3600)
        } else {
            format!("{}j", total_seconds / 86400)
        }
    }
}

/// Utilitaires pour l'affichage des dates relatives
pub fn relative_time(datetime: &DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(*datetime);
    
    if duration.num_seconds() < 0 {
        format!("dans {}", (-duration).human_duration())
    } else if duration.num_seconds() < 60 {
        "à l'instant".to_string()
    } else {
        format!("il y a {}", duration.human_duration())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Task;

    #[test]
    fn test_colorize_status() {
        let status = Status::Completed;
        let colored = TaskDisplay::colorize_status(&status);
        assert!(colored.to_string().contains("Terminé"));
    }

    #[test]
    fn test_percentage() {
        assert_eq!(TaskDisplay::percentage(25, 100), "25.0");
        assert_eq!(TaskDisplay::percentage(0, 100), "0.0");
        assert_eq!(TaskDisplay::percentage(10, 0), "0");
    }

    #[test]
    fn test_duration_format() {
        let duration = chrono::Duration::seconds(3661); // 1h 1m 1s
        assert_eq!(duration.human_duration(), "1h");
        
        let duration = chrono::Duration::seconds(61); // 1m 1s
        assert_eq!(duration.human_duration(), "1m");
        
        let duration = chrono::Duration::seconds(30);
        assert_eq!(duration.human_duration(), "30s");
    }
}