import axios from 'axios';
import type { Task, CreateTaskRequest, UpdateTaskRequest, LoginRequest, RegisterRequest, AuthResponse } from '../types';

// Configuration de l'URL de base selon l'environnement
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add auth token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('auth_token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// Auth API
export const authAPI = {
  login: async (credentials: LoginRequest): Promise<AuthResponse> => {
    const response = await api.post('/api/v1/auth/login', credentials);
    return response.data;
  },

  register: async (userData: RegisterRequest): Promise<AuthResponse> => {
    const response = await api.post('/api/v1/auth/register', userData);
    return response.data;
  },

  logout: () => {
    localStorage.removeItem('auth_token');
  },
};

// Tasks API
export const tasksAPI = {
  getTasks: async (): Promise<Task[]> => {
    const response = await api.get('/api/v1/tasks');
    return response.data;
  },

  createTask: async (task: CreateTaskRequest): Promise<Task> => {
    const response = await api.post('/api/v1/tasks', task);
    return response.data;
  },

  updateTask: async (id: string, updates: UpdateTaskRequest): Promise<Task> => {
    const response = await api.put(`/api/v1/tasks/${id}`, updates);
    return response.data;
  },

  deleteTask: async (id: string): Promise<void> => {
    await api.delete(`/api/v1/tasks/${id}`);
  },
};

export default api;