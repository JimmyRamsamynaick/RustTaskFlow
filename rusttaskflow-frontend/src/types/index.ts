export interface User {
  id: string;
  username: string;
  email: string;
  created_at: string;
}

export interface Task {
  id: string;
  title: string;
  description?: string;
  status: TaskStatus;
  priority: TaskPriority;
  tags: string[];
  due_date?: string;
  assigned_to?: string;
  created_by: string;
  created_at: string;
  updated_at: string;
  started_at?: string;
  completed_at?: string;
}

export type TaskStatus = 'Todo' | 'InProgress' | 'Completed' | 'Cancelled';

export type TaskPriority = 'Low' | 'Medium' | 'High' | 'Critical';

export interface CreateTaskRequest {
  title: string;
  description?: string;
  priority: TaskPriority;
  tags: string[];
  due_date?: string;
  assigned_to?: string;
}

export interface UpdateTaskRequest {
  title?: string;
  description?: string;
  status?: TaskStatus;
  priority?: TaskPriority;
  tags?: string[];
  due_date?: string;
  assigned_to?: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface Project {
  id: string;
  name: string;
  description?: string;
  created_by: string;
  created_at: string;
  updated_at: string;
  members: ProjectMember[];
  tasks: Task[];
}

export interface ProjectMember {
  user_id: string;
  username: string;
  email: string;
  role: ProjectRole;
  joined_at: string;
}

export type ProjectRole = 'Owner' | 'Admin' | 'Member' | 'Viewer';

export interface CreateProjectRequest {
  name: string;
  description?: string;
}

export interface InviteUserRequest {
  project_id: string;
  email: string;
  role: ProjectRole;
}

export interface ProjectInvitation {
  id: string;
  project_id: string;
  project_name: string;
  invited_by: string;
  invited_user_email: string;
  role: ProjectRole;
  status: InvitationStatus;
  created_at: string;
  expires_at: string;
}

export type InvitationStatus = 'Pending' | 'Accepted' | 'Declined' | 'Expired';