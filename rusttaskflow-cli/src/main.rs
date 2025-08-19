mod cli;
mod storage;
mod task;
mod task_manager;
mod ui;

use anyhow::{anyhow, Result};
use clap::Parser;
use cli::{parse_date, parse_tags, find_task_by_partial_id, Cli, Commands, TagCommands};
use colored::*;
use storage::{create_storage};
use task::{Status};
use task_manager::{TaskFilter, TaskManager};
use ui::TaskDisplay;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Erreur:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    // Créer le stockage (par défaut JSON)
    let storage = create_storage("json")?;
    
    // Charger les tâches existantes
    let mut task_manager = TaskManager::new();
    let tasks = storage.load_tasks()?;
    task_manager.load_tasks(tasks);
    
    // Traiter la commande
    match cli.command {
        Commands::Add {
            title,
            description,
            priority,
            tags,
            due,
        } => {
            let task_id = task_manager.add_task(title.clone());
            
            // Ajouter la description si fournie
            if let Some(desc) = description {
                task_manager.update_task_description(&task_id, Some(desc))?;
            }
            
            // Ajouter la priorité si fournie
            if let Some(prio) = priority {
                task_manager.update_task_priority(&task_id, prio.into())?;
            }
            
            // Ajouter les tags si fournis
            if let Some(tags_str) = tags {
                let tag_list = parse_tags(&tags_str);
                for tag in tag_list {
                    task_manager.add_tag_to_task(&task_id, tag)?;
                }
            }
            
            // Ajouter la date d'échéance si fournie
            if let Some(due_str) = due {
                let due_date = parse_date(&due_str)
                    .map_err(|_| anyhow!("Format de date invalide: {}", due_str))?;
                task_manager.set_task_due_date(&task_id, Some(due_date))?;
            }
            
            storage.save_tasks(task_manager.export_tasks())?;
            
            println!(
                "{} Tâche '{}' ajoutée avec l'ID {}",
                "✓".green().bold(),
                title,
                task_id.to_string()[..8].to_uppercase().bright_blue()
            );
        }
        
        Commands::List {
            status,
            priority,
            tags,
            overdue,
            limit,
        } => {
            let mut filter = TaskFilter::default();
            
            if let Some(s) = status {
                filter.status = Some(s.into());
            }
            
            if let Some(p) = priority {
                filter.priority = Some(p.into());
            }
            
            if let Some(tags_str) = tags {
                filter.tags = parse_tags(&tags_str);
            }
            
            filter.overdue_only = overdue;
            
            let mut tasks = task_manager.filter_tasks(&filter);
            
            // Trier par date de création (plus récent en premier)
            tasks.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            
            // Limiter le nombre de résultats
            if let Some(limit_count) = limit {
                tasks.truncate(limit_count);
            }
            
            if tasks.is_empty() {
                println!("{}", "Aucune tâche trouvée.".yellow());
            } else {
                TaskDisplay::print_task_list(&tasks);
            }
        }
        
        Commands::Complete { id } => {
            let task_ids: Vec<_> = task_manager.get_all_tasks().iter().map(|t| t.id).collect();
            let task_id = find_task_by_partial_id(&id, &task_ids)
                .ok_or_else(|| anyhow!("Tâche avec l'ID '{}' non trouvée", id))?;
            
            task_manager.complete_task(&task_id)?;
            storage.save_tasks(task_manager.export_tasks())?;
            
            let task = task_manager.get_task(&task_id).unwrap();
            println!(
                "{} Tâche '{}' marquée comme terminée",
                "✓".green().bold(),
                task.title
            );
        }
        
        Commands::Start { id } => {
            let task_ids: Vec<_> = task_manager.get_all_tasks().iter().map(|t| t.id).collect();
            let task_id = find_task_by_partial_id(&id, &task_ids)
                .ok_or_else(|| anyhow!("Tâche avec l'ID '{}' non trouvée", id))?;
            
            task_manager.start_task(&task_id)?;
            storage.save_tasks(task_manager.export_tasks())?;
            
            let task = task_manager.get_task(&task_id).unwrap();
            println!(
                "{} Tâche '{}' démarrée",
                "▶".blue().bold(),
                task.title
            );
        }
        
        Commands::Cancel { id } => {
            let task_ids: Vec<_> = task_manager.get_all_tasks().iter().map(|t| t.id).collect();
            let task_id = find_task_by_partial_id(&id, &task_ids)
                .ok_or_else(|| anyhow!("Tâche avec l'ID '{}' non trouvée", id))?;
            
            task_manager.cancel_task(&task_id)?;
            storage.save_tasks(task_manager.export_tasks())?;
            
            let task = task_manager.get_task(&task_id).unwrap();
            println!(
                "{} Tâche '{}' annulée",
                "✗".red().bold(),
                task.title
            );
        }
        
        Commands::Delete { id, force } => {
            let task_ids: Vec<_> = task_manager.get_all_tasks().iter().map(|t| t.id).collect();
            let task_id = find_task_by_partial_id(&id, &task_ids)
                .ok_or_else(|| anyhow!("Tâche avec l'ID '{}' non trouvée", id))?;
            
            let task = task_manager.get_task(&task_id).unwrap();
            let task_title = task.title.clone();
            
            if !force {
                print!("Êtes-vous sûr de vouloir supprimer la tâche '{}' ? (y/N): ", task_title);
                use std::io::{self, Write};
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Suppression annulée.");
                    return Ok(());
                }
            }
            
            task_manager.delete_task(&task_id)?;
            storage.save_tasks(task_manager.export_tasks())?;
            
            println!(
                "{} Tâche '{}' supprimée",
                "🗑".bold(),
                task_title
            );
        }
        
        Commands::Edit {
            id,
            title,
            description,
            priority,
            due,
        } => {
            let task_ids: Vec<_> = task_manager.get_all_tasks().iter().map(|t| t.id).collect();
            let task_id = find_task_by_partial_id(&id, &task_ids)
                .ok_or_else(|| anyhow!("Tâche avec l'ID '{}' non trouvée", id))?;
            
            if let Some(new_title) = title {
                task_manager.update_task_title(&task_id, new_title)?;
            }
            
            if let Some(new_desc) = description {
                task_manager.update_task_description(&task_id, Some(new_desc))?;
            }
            
            if let Some(new_priority) = priority {
                task_manager.update_task_priority(&task_id, new_priority.into())?;
            }
            
            if let Some(due_str) = due {
                let due_date = parse_date(&due_str)
                    .map_err(|_| anyhow!("Format de date invalide: {}", due_str))?;
                task_manager.set_task_due_date(&task_id, Some(due_date))?;
            }
            
            storage.save_tasks(task_manager.export_tasks())?;
            
            let task = task_manager.get_task(&task_id).unwrap();
            println!(
                "{} Tâche '{}' modifiée",
                "✏".yellow().bold(),
                task.title
            );
        }
        
        Commands::Search { query, limit } => {
            let mut tasks = task_manager.search_tasks(&query);
            
            // Trier par pertinence (titre exact en premier)
            tasks.sort_by(|a, b| {
                let a_exact = a.title.to_lowercase().contains(&query.to_lowercase());
                let b_exact = b.title.to_lowercase().contains(&query.to_lowercase());
                b_exact.cmp(&a_exact)
            });
            
            if let Some(limit_count) = limit {
                tasks.truncate(limit_count);
            }
            
            if tasks.is_empty() {
                println!(
                    "{} Aucune tâche trouvée pour '{}'",
                    "🔍".yellow(),
                    query.bright_blue()
                );
            } else {
                println!(
                    "{} {} tâche(s) trouvée(s) pour '{}'",
                    "🔍".green(),
                    tasks.len(),
                    query.bright_blue()
                );
                TaskDisplay::print_task_list(&tasks);
            }
        }
        
        Commands::Show { id } => {
            let task_ids: Vec<_> = task_manager.get_all_tasks().iter().map(|t| t.id).collect();
            let task_id = find_task_by_partial_id(&id, &task_ids)
                .ok_or_else(|| anyhow!("Tâche avec l'ID '{}' non trouvée", id))?;
            
            let task = task_manager.get_task(&task_id).unwrap();
            TaskDisplay::print_task_details(task);
        }
        
        Commands::Tag { action } => {
            match action {
                TagCommands::Add { id, tag } => {
                    let task_ids: Vec<_> = task_manager.get_all_tasks().iter().map(|t| t.id).collect();
                    let task_id = find_task_by_partial_id(&id, &task_ids)
                        .ok_or_else(|| anyhow!("Tâche avec l'ID '{}' non trouvée", id))?;
                    
                    task_manager.add_tag_to_task(&task_id, tag.clone())?;
                    storage.save_tasks(task_manager.export_tasks())?;
                    
                    println!(
                        "{} Tag '{}' ajouté à la tâche",
                        "🏷".green().bold(),
                        tag.bright_blue()
                    );
                }
                
                TagCommands::Remove { id, tag } => {
                    let task_ids: Vec<_> = task_manager.get_all_tasks().iter().map(|t| t.id).collect();
                    let task_id = find_task_by_partial_id(&id, &task_ids)
                        .ok_or_else(|| anyhow!("Tâche avec l'ID '{}' non trouvée", id))?;
                    
                    task_manager.remove_tag_from_task(&task_id, &tag)?;
                    storage.save_tasks(task_manager.export_tasks())?;
                    
                    println!(
                        "{} Tag '{}' supprimé de la tâche",
                        "🏷".red().bold(),
                        tag.bright_blue()
                    );
                }
            }
        }
        
        Commands::Stats => {
            let stats = task_manager.get_stats();
            TaskDisplay::print_stats(&stats);
        }
        
        Commands::Tags => {
            let tags = task_manager.get_all_tags();
            if tags.is_empty() {
                println!("{}", "Aucun tag trouvé.".yellow());
            } else {
                println!("{} Tags disponibles:", "🏷".bright_blue().bold());
                for tag in tags {
                    println!("  • {}", tag.bright_blue());
                }
            }
        }
        
        Commands::Export { output, format } => {
            let tasks = task_manager.export_tasks();
            
            match format {
                cli::ExportFormat::Json => {
                    let json = serde_json::to_string_pretty(tasks)?;
                    
                    if let Some(file_path) = output {
                        std::fs::write(&file_path, json)?;
                        println!(
                            "{} Tâches exportées vers '{}'",
                            "💾".green().bold(),
                            file_path.bright_blue()
                        );
                    } else {
                        println!("{}", json);
                    }
                }
                
                cli::ExportFormat::Csv => {
                    // Implémentation CSV basique
                    let mut csv_content = String::from("ID,Title,Description,Status,Priority,Tags,Created,Updated,Due,Completed\n");
                    
                    for task in tasks.values() {
                        let tags_str = task.tags.join(";");
                        let description = task.description.as_deref().unwrap_or("");
                        let due_date = task.due_date.map(|d| d.to_rfc3339()).unwrap_or_default();
                        let completed_at = task.completed_at.map(|d| d.to_rfc3339()).unwrap_or_default();
                        
                        csv_content.push_str(&format!(
                            "{},{},{},{},{},{},{},{},{},{}\n",
                            task.id,
                            task.title,
                            description,
                            task.status,
                            task.priority,
                            tags_str,
                            task.created_at.to_rfc3339(),
                            task.updated_at.to_rfc3339(),
                            due_date,
                            completed_at
                        ));
                    }
                    
                    if let Some(file_path) = output {
                        std::fs::write(&file_path, csv_content)?;
                        println!(
                            "{} Tâches exportées vers '{}'",
                            "💾".green().bold(),
                            file_path.bright_blue()
                        );
                    } else {
                        println!("{}", csv_content);
                    }
                }
            }
        }
        
        Commands::Import { file, format: _ } => {
            let content = std::fs::read_to_string(&file)
                .map_err(|_| anyhow!("Impossible de lire le fichier '{}'", file))?;
            
            let imported_tasks: std::collections::HashMap<uuid::Uuid, task::Task> = 
                serde_json::from_str(&content)
                    .map_err(|_| anyhow!("Format JSON invalide dans le fichier '{}'", file))?;
            
            let count = imported_tasks.len();
            task_manager.load_tasks(imported_tasks);
            storage.save_tasks(task_manager.export_tasks())?;
            
            println!(
                "{} {} tâche(s) importée(s) depuis '{}'",
                "📥".green().bold(),
                count,
                file.bright_blue()
            );
        }
        
        Commands::Clean { days, force } => {
            let cutoff_date = chrono::Utc::now() - chrono::Duration::days(days as i64);
            let completed_tasks: Vec<_> = task_manager
                .get_tasks_by_status(Status::Completed)
                .into_iter()
                .filter(|task| {
                    task.completed_at
                        .map(|completed| completed < cutoff_date)
                        .unwrap_or(false)
                })
                .collect();
            
            if completed_tasks.is_empty() {
                println!(
                    "{} Aucune tâche terminée depuis plus de {} jours.",
                    "🧹".yellow(),
                    days
                );
                return Ok(());
            }
            
            if !force {
                println!(
                    "Supprimer {} tâche(s) terminée(s) depuis plus de {} jours ? (y/N): ",
                    completed_tasks.len(),
                    days
                );
                
                use std::io::{self, Write};
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Nettoyage annulé.");
                    return Ok(());
                }
            }
            
            let task_ids_to_delete: Vec<_> = completed_tasks.iter().map(|task| task.id).collect();
            let mut deleted_count = 0;
            for task_id in task_ids_to_delete {
                task_manager.delete_task(&task_id)?;
                deleted_count += 1;
            }
            
            storage.save_tasks(task_manager.export_tasks())?;
            
            println!(
                "{} {} tâche(s) supprimée(s)",
                "🧹".green().bold(),
                deleted_count
            );
        }
    }
    
    Ok(())
}