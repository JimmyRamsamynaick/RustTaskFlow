use axum::{
    extract::{ws::{WebSocket, Message}, WebSocketUpgrade, State},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rusttaskflow_core::models::Task;

// Types de messages WebSocket
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    TaskCreated { task: Task, user_id: Uuid },
    TaskUpdated { task: Task, user_id: Uuid },
    TaskDeleted { task_id: Uuid, user_id: Uuid },
    UserConnected { user_id: Uuid, username: String },
    UserDisconnected { user_id: Uuid },
    Notification { message: String, notification_type: String, user_id: Option<Uuid> },
}

// Messages entrants des clients
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Ping,
    Subscribe { room: String },
    Unsubscribe { room: String },
    Authenticate { token: String },
}

// Handler pour la mise à niveau WebSocket
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<crate::AppState>,
) -> Response {
    ws.on_upgrade(move |socket| websocket_connection(socket, app_state))
}

// Gestion d'une connexion WebSocket individuelle
async fn websocket_connection(socket: WebSocket, app_state: crate::AppState) {
    let _connection_id = Uuid::new_v4().to_string();
    let mut receiver = app_state.websocket_tx.subscribe();
    
    let (mut sender, mut receiver_ws) = socket.split();
    
    // Pour l'instant, pas d'authentification automatique
    // L'authentification se fera via les messages ClientMessage::Authenticate
    let mut user_id: Option<Uuid> = None;
    let mut username: Option<String> = None;
    
    // Task pour envoyer les messages broadcast
    let _app_state_clone = app_state.clone();
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = receiver.recv().await {
            let json_msg = serde_json::to_string(&msg).unwrap_or_default();
            if sender.send(Message::Text(json_msg)).await.is_err() {
                break;
            }
        }
    });
    
    // Task pour recevoir les messages du client
    let app_state_clone2 = app_state.clone();
    let receive_task = tokio::spawn(async move {
        while let Some(msg) = receiver_ws.next().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(text) => {
                        if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                            handle_client_message(client_msg, &app_state_clone2).await;
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            } else {
                break;
            }
        }
    });
    
    // Envoyer une notification de connexion si l'utilisateur est authentifié
    if let (Some(uid), Some(uname)) = (user_id, &username) {
        let _ = app_state.websocket_tx.send(WebSocketMessage::UserConnected {
            user_id: uid,
            username: uname.clone(),
        });
    }
    
    // Attendre que l'une des tâches se termine
    tokio::select! {
        _ = send_task => {},
        _ = receive_task => {},
    }
    
    // Envoyer une notification de déconnexion si l'utilisateur était authentifié
    if let Some(uid) = user_id {
        let _ = app_state.websocket_tx.send(WebSocketMessage::UserDisconnected {
            user_id: uid,
        });
    }
}

// Traitement des messages clients
async fn handle_client_message(
    message: ClientMessage,
    app_state: &crate::AppState,
) {
    match message {
        ClientMessage::Authenticate { token } => {
            // Valider le token JWT et associer l'utilisateur à la connexion
            if let Ok(claims) = app_state.auth_service.verify_token(&token) {
                if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
                    let username = format!("user_{}", claims.sub);
                    
                    // Notifier les autres utilisateurs
                    let _ = app_state.websocket_tx.send(WebSocketMessage::UserConnected { user_id, username });
                }
            }
        }
        ClientMessage::Ping => {
            // Répondre au ping (keep-alive)
            let pong_msg = WebSocketMessage::Notification {
                message: "pong".to_string(),
                notification_type: "system".to_string(),
                user_id: None,
            };
            let _ = app_state.websocket_tx.send(pong_msg);
        }
        ClientMessage::Subscribe { room } => {
            // Gérer l'abonnement à une room (par exemple, un projet spécifique)
            println!("Client s'abonne à la room: {}", room);
        }
        ClientMessage::Unsubscribe { room } => {
            // Gérer le désabonnement d'une room
            println!("Client se désabonne de la room: {}", room);
        }
    }
}