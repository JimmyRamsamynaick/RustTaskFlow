import React, { createContext, useContext, useEffect, useState } from 'react';
import type { ReactNode } from 'react';
import { useWebSocket, type TypedWebSocketMessage } from '../hooks/useWebSocket';
import { useAuth } from './AuthContext';
import type { Task } from '../types';

interface WebSocketContextType {
  isConnected: boolean;
  connectionError: string | null;
  onlineUsers: Set<string>;
  sendMessage: (message: any) => boolean;
  connect: () => void;
  disconnect: () => void;
}

const WebSocketContext = createContext<WebSocketContextType | undefined>(undefined);

export const useWebSocketContext = () => {
  const context = useContext(WebSocketContext);
  if (context === undefined) {
    throw new Error('useWebSocketContext must be used within a WebSocketProvider');
  }
  return context;
};

interface WebSocketProviderProps {
  children: ReactNode;
  onTaskCreated?: (task: Task, userId: string) => void;
  onTaskUpdated?: (task: Task, userId: string) => void;
  onTaskDeleted?: (taskId: string, userId: string) => void;
  onNotification?: (message: string, type: 'info' | 'success' | 'warning' | 'error', userId?: string) => void;
}

export const WebSocketProvider: React.FC<WebSocketProviderProps> = ({ 
  children, 
  onTaskCreated,
  onTaskUpdated,
  onTaskDeleted,
  onNotification
}) => {
  const { user } = useAuth();
  const [onlineUsers, setOnlineUsers] = useState<Set<string>>(new Set());

  const handleMessage = (message: TypedWebSocketMessage) => {
    console.log('WebSocket message received in context:', message);
  };

  const handleTaskCreated = (task: Task, userId: string) => {
    console.log('Task created:', task, 'by user:', userId);
    onTaskCreated?.(task, userId);
    
    // Show notification if it's not the current user
    if (user && userId !== user.id) {
      onNotification?.(
        `New task "${task.title}" was created`,
        'info',
        userId
      );
    }
  };

  const handleTaskUpdated = (task: Task, userId: string) => {
    console.log('Task updated:', task, 'by user:', userId);
    onTaskUpdated?.(task, userId);
    
    // Show notification if it's not the current user
    if (user && userId !== user.id) {
      onNotification?.(
        `Task "${task.title}" was updated`,
        'info',
        userId
      );
    }
  };

  const handleTaskDeleted = (taskId: string, userId: string) => {
    console.log('Task deleted:', taskId, 'by user:', userId);
    onTaskDeleted?.(taskId, userId);
    
    // Show notification if it's not the current user
    if (user && userId !== user.id) {
      onNotification?.(
        'A task was deleted',
        'info',
        userId
      );
    }
  };

  const handleUserConnected = (userId: string, username: string) => {
    console.log('User connected:', userId, username);
    setOnlineUsers(prev => new Set([...prev, userId]));
    
    // Show notification if it's not the current user
    if (user && userId !== user.id) {
      onNotification?.(
        `${username} joined the workspace`,
        'success',
        userId
      );
    }
  };

  const handleUserDisconnected = (userId: string) => {
    console.log('User disconnected:', userId);
    setOnlineUsers(prev => {
      const newSet = new Set(prev);
      newSet.delete(userId);
      return newSet;
    });
  };

  const handleNotification = (message: string, type: 'info' | 'success' | 'warning' | 'error', userId?: string) => {
    console.log('Notification received:', message, type, userId);
    onNotification?.(message, type, userId);
  };

  const {
    isConnected,
    connectionError,
    connect,
    disconnect,
    sendMessage,
  } = useWebSocket({
    onMessage: handleMessage,
    onTaskCreated: handleTaskCreated,
    onTaskUpdated: handleTaskUpdated,
    onTaskDeleted: handleTaskDeleted,
    onUserConnected: handleUserConnected,
    onUserDisconnected: handleUserDisconnected,
    onNotification: handleNotification,
  });

  // Clear online users when disconnected
  useEffect(() => {
    if (!isConnected) {
      setOnlineUsers(new Set());
    }
  }, [isConnected]);

  const value: WebSocketContextType = {
    isConnected,
    connectionError,
    onlineUsers,
    sendMessage,
    connect,
    disconnect,
  };

  return (
    <WebSocketContext.Provider value={value}>
      {children}
    </WebSocketContext.Provider>
  );
};