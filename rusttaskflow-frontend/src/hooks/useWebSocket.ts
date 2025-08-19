import { useEffect, useRef, useState } from 'react';
import { useAuth } from '../contexts/AuthContext';
import type { Task } from '../types';

export interface WebSocketMessage {
  type: 'task_created' | 'task_updated' | 'task_deleted' | 'user_connected' | 'user_disconnected' | 'notification';
  data: any;
}

export interface TaskCreatedMessage {
  type: 'task_created';
  data: {
    task: Task;
    user_id: string;
  };
}

export interface TaskUpdatedMessage {
  type: 'task_updated';
  data: {
    task: Task;
    user_id: string;
  };
}

export interface TaskDeletedMessage {
  type: 'task_deleted';
  data: {
    task_id: string;
    user_id: string;
  };
}

export interface UserConnectedMessage {
  type: 'user_connected';
  data: {
    user_id: string;
    username: string;
  };
}

export interface UserDisconnectedMessage {
  type: 'user_disconnected';
  data: {
    user_id: string;
  };
}

export interface NotificationMessage {
  type: 'notification';
  data: {
    message: string;
    notification_type: 'info' | 'success' | 'warning' | 'error';
    user_id?: string;
  };
}

export type TypedWebSocketMessage = 
  | TaskCreatedMessage 
  | TaskUpdatedMessage 
  | TaskDeletedMessage 
  | UserConnectedMessage 
  | UserDisconnectedMessage 
  | NotificationMessage;

export interface UseWebSocketOptions {
  onMessage?: (message: TypedWebSocketMessage) => void;
  onTaskCreated?: (task: Task, userId: string) => void;
  onTaskUpdated?: (task: Task, userId: string) => void;
  onTaskDeleted?: (taskId: string, userId: string) => void;
  onUserConnected?: (userId: string, username: string) => void;
  onUserDisconnected?: (userId: string) => void;
  onNotification?: (message: string, type: 'info' | 'success' | 'warning' | 'error', userId?: string) => void;
}

export const useWebSocket = (options: UseWebSocketOptions = {}) => {
  const { user } = useAuth();
  const getToken = () => localStorage.getItem('auth_token');
  const [isConnected, setIsConnected] = useState(false);
  const [connectionError, setConnectionError] = useState<string | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const reconnectTimeoutRef = useRef<number | null>(null);
  const reconnectAttempts = useRef(0);
  const maxReconnectAttempts = 5;

  const connect = () => {
    const token = getToken();
    if (!token || !user) {
      console.log('No token or user available for WebSocket connection');
      return;
    }

    try {
      const wsUrl = `ws://localhost:3000/ws?token=${encodeURIComponent(token)}`;
      console.log('Connecting to WebSocket:', wsUrl);
      
      const ws = new WebSocket(wsUrl);
      wsRef.current = ws;

      ws.onopen = () => {
        console.log('WebSocket connected');
        setIsConnected(true);
        setConnectionError(null);
        reconnectAttempts.current = 0;
      };

      ws.onmessage = (event) => {
        try {
          const message: TypedWebSocketMessage = JSON.parse(event.data);
          console.log('WebSocket message received:', message);
          
          // Call general message handler
          options.onMessage?.(message);
          
          // Call specific handlers based on message type
          switch (message.type) {
            case 'task_created':
              options.onTaskCreated?.(message.data.task, message.data.user_id);
              break;
            case 'task_updated':
              options.onTaskUpdated?.(message.data.task, message.data.user_id);
              break;
            case 'task_deleted':
              options.onTaskDeleted?.(message.data.task_id, message.data.user_id);
              break;
            case 'user_connected':
              options.onUserConnected?.(message.data.user_id, message.data.username);
              break;
            case 'user_disconnected':
              options.onUserDisconnected?.(message.data.user_id);
              break;
            case 'notification':
              options.onNotification?.(
                message.data.message,
                message.data.notification_type,
                message.data.user_id
              );
              break;
          }
        } catch (error) {
          console.error('Error parsing WebSocket message:', error);
        }
      };

      ws.onclose = (event) => {
        console.log('WebSocket disconnected:', event.code, event.reason);
        setIsConnected(false);
        wsRef.current = null;
        
        // Attempt to reconnect if not a normal closure
        if (event.code !== 1000 && reconnectAttempts.current < maxReconnectAttempts) {
          const delay = Math.min(1000 * Math.pow(2, reconnectAttempts.current), 30000);
          console.log(`Attempting to reconnect in ${delay}ms (attempt ${reconnectAttempts.current + 1}/${maxReconnectAttempts})`);
          
          reconnectTimeoutRef.current = window.setTimeout(() => {
            reconnectAttempts.current++;
            connect();
          }, delay);
        } else if (reconnectAttempts.current >= maxReconnectAttempts) {
          setConnectionError('Failed to reconnect after multiple attempts');
        }
      };

      ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        setConnectionError('WebSocket connection error');
      };
    } catch (error) {
      console.error('Error creating WebSocket connection:', error);
      setConnectionError('Failed to create WebSocket connection');
    }
  };

  const disconnect = () => {
    if (reconnectTimeoutRef.current) {
      clearTimeout(reconnectTimeoutRef.current);
      reconnectTimeoutRef.current = null;
    }
    
    if (wsRef.current) {
      wsRef.current.close(1000, 'User disconnected');
      wsRef.current = null;
    }
    
    setIsConnected(false);
    setConnectionError(null);
    reconnectAttempts.current = 0;
  };

  const sendMessage = (message: any) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
      return true;
    }
    console.warn('WebSocket is not connected. Cannot send message:', message);
    return false;
  };

  useEffect(() => {
    const token = getToken();
    if (token && user) {
      connect();
    }

    return () => {
      disconnect();
    };
  }, [user]);

  return {
    isConnected,
    connectionError,
    connect,
    disconnect,
    sendMessage,
  };
};