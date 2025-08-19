import React, { createContext, useContext, useState, useEffect } from 'react';
import type { ReactNode } from 'react';
import type { Task, CreateTaskRequest, UpdateTaskRequest } from '../types';
import { tasksAPI } from '../services/api';
import { useAuth } from './AuthContext';

interface TaskContextType {
  tasks: Task[];
  isLoading: boolean;
  createTask: (task: CreateTaskRequest) => Promise<void>;
  updateTask: (id: string, updates: UpdateTaskRequest) => Promise<void>;
  deleteTask: (id: string) => Promise<void>;
  refreshTasks: () => Promise<void>;
}

const TaskContext = createContext<TaskContextType | undefined>(undefined);

export const useTask = () => {
  const context = useContext(TaskContext);
  if (context === undefined) {
    throw new Error('useTask must be used within a TaskProvider');
  }
  return context;
};

interface TaskProviderProps {
  children: ReactNode;
}

export const TaskProvider: React.FC<TaskProviderProps> = ({ children }) => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const { isAuthenticated } = useAuth();

  const refreshTasks = async () => {
    if (!isAuthenticated) return;
    
    setIsLoading(true);
    try {
      const fetchedTasks = await tasksAPI.getTasks();
      setTasks(fetchedTasks);
    } catch (error) {
      console.error('Error fetching tasks:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const createTask = async (taskData: CreateTaskRequest) => {
    try {
      const newTask = await tasksAPI.createTask(taskData);
      setTasks(prev => [...prev, newTask]);
    } catch (error) {
      console.error('Error creating task:', error);
      throw error;
    }
  };

  const updateTask = async (id: string, updates: UpdateTaskRequest) => {
    try {
      const updatedTask = await tasksAPI.updateTask(id, updates);
      setTasks(prev => prev.map(task => task.id === id ? updatedTask : task));
    } catch (error) {
      console.error('Error updating task:', error);
      throw error;
    }
  };

  const deleteTask = async (id: string) => {
    try {
      await tasksAPI.deleteTask(id);
      setTasks(prev => prev.filter(task => task.id !== id));
    } catch (error) {
      console.error('Error deleting task:', error);
      throw error;
    }
  };

  useEffect(() => {
    if (isAuthenticated) {
      refreshTasks();
    } else {
      setTasks([]);
    }
  }, [isAuthenticated]);

  const value = {
    tasks,
    isLoading,
    createTask,
    updateTask,
    deleteTask,
    refreshTasks,
  };

  return <TaskContext.Provider value={value}>{children}</TaskContext.Provider>;
};