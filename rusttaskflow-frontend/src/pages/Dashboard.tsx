import React, { useState } from 'react';
import { Plus, Search, Filter } from 'lucide-react';
import { useTask } from '../contexts/TaskContext';
import TaskCard from '../components/TaskCard';
import TaskForm from '../components/TaskForm';
import type { TaskStatus, TaskPriority } from '../types';

const Dashboard: React.FC = () => {
  const { tasks, isLoading } = useTask();
  const [showTaskForm, setShowTaskForm] = useState(false);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState<TaskStatus | 'all'>('all');
  const [priorityFilter, setPriorityFilter] = useState<TaskPriority | 'all'>('all');

  const filteredTasks = tasks.filter(task => {
    const matchesSearch = task.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         task.description?.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesStatus = statusFilter === 'all' || task.status === statusFilter;
    const matchesPriority = priorityFilter === 'all' || task.priority === priorityFilter;
    
    return matchesSearch && matchesStatus && matchesPriority;
  });

  const tasksByStatus = {
    todo: filteredTasks.filter(task => task.status === 'todo'),
    in_progress: filteredTasks.filter(task => task.status === 'in_progress'),
    done: filteredTasks.filter(task => task.status === 'done'),
  };

  if (isLoading) {
    return (
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: '100vh' }}>
        <div style={{ 
          animation: 'spin 1s linear infinite', 
          borderRadius: '50%', 
          height: '8rem', 
          width: '8rem', 
          borderBottom: '2px solid rgb(37 99 235)' 
        }}></div>
      </div>
    );
  }

  return (
    <div style={{ padding: '1.5rem' }}>
      {/* Header */}
      <div style={{ display: 'flex', flexDirection: 'column', justifyContent: 'space-between', alignItems: 'flex-start', gap: '1rem', marginBottom: '1.5rem' }}>
        <div>
          <h1 style={{ fontSize: '1.875rem', fontWeight: '700', color: 'rgb(17 24 39)' }}>Tableau de bord</h1>
          <p style={{ color: 'rgb(75 85 99)', marginTop: '0.25rem' }}>
            Gérez vos tâches efficacement
          </p>
        </div>
        <button
          onClick={() => setShowTaskForm(true)}
          className="btn-primary"
          style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}
        >
          <Plus className="h-4 w-4" />
          <span>Nouvelle tâche</span>
        </button>
      </div>

      {/* Filters */}
      <div className="card">
        <div className="flex flex-col sm:flex-row gap-4">
          <div className="flex-1">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
              <input
                type="text"
                placeholder="Rechercher des tâches..."
                className="input-field pl-10"
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
              />
            </div>
          </div>
          <div className="flex gap-2">
            <select
              className="input-field"
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value as TaskStatus | 'all')}
            >
              <option value="all">Tous les statuts</option>
              <option value="todo">À faire</option>
              <option value="in_progress">En cours</option>
              <option value="done">Terminé</option>
            </select>
            <select
              className="input-field"
              value={priorityFilter}
              onChange={(e) => setPriorityFilter(e.target.value as TaskPriority | 'all')}
            >
              <option value="all">Toutes les priorités</option>
              <option value="high">Haute</option>
              <option value="medium">Moyenne</option>
              <option value="low">Basse</option>
            </select>
          </div>
        </div>
      </div>

      {/* Task Columns */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="space-y-4">
          <h2 className="text-lg font-semibold text-gray-900 flex items-center">
            À faire
            <span className="ml-2 bg-gray-100 text-gray-600 text-sm px-2 py-1 rounded-full">
              {tasksByStatus.todo.length}
            </span>
          </h2>
          <div className="space-y-3">
            {tasksByStatus.todo.map(task => (
              <TaskCard key={task.id} task={task} />
            ))}
            {tasksByStatus.todo.length === 0 && (
              <p className="text-gray-500 text-center py-8">Aucune tâche à faire</p>
            )}
          </div>
        </div>

        <div className="space-y-4">
          <h2 className="text-lg font-semibold text-gray-900 flex items-center">
            En cours
            <span className="ml-2 bg-blue-100 text-blue-600 text-sm px-2 py-1 rounded-full">
              {tasksByStatus.in_progress.length}
            </span>
          </h2>
          <div className="space-y-3">
            {tasksByStatus.in_progress.map(task => (
              <TaskCard key={task.id} task={task} />
            ))}
            {tasksByStatus.in_progress.length === 0 && (
              <p className="text-gray-500 text-center py-8">Aucune tâche en cours</p>
            )}
          </div>
        </div>

        <div className="space-y-4">
          <h2 className="text-lg font-semibold text-gray-900 flex items-center">
            Terminé
            <span className="ml-2 bg-green-100 text-green-600 text-sm px-2 py-1 rounded-full">
              {tasksByStatus.done.length}
            </span>
          </h2>
          <div className="space-y-3">
            {tasksByStatus.done.map(task => (
              <TaskCard key={task.id} task={task} />
            ))}
            {tasksByStatus.done.length === 0 && (
              <p className="text-gray-500 text-center py-8">Aucune tâche terminée</p>
            )}
          </div>
        </div>
      </div>

      {/* Task Form Modal */}
      {showTaskForm && (
        <TaskForm
          onClose={() => setShowTaskForm(false)}
          onSuccess={() => setShowTaskForm(false)}
        />
      )}
    </div>
  );
};

export default Dashboard;