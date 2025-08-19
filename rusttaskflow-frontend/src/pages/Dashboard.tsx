import React, { useState } from 'react';
import { Plus, Search, Filter } from 'lucide-react';
import { useTask } from '../contexts/TaskContext';
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
    todo: filteredTasks.filter(task => task.status === 'Todo'),
    in_progress: filteredTasks.filter(task => task.status === 'InProgress'),
    done: filteredTasks.filter(task => task.status === 'Completed'),
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
              <option value="Todo">À faire</option>
              <option value="InProgress">En cours</option>
              <option value="Completed">Terminé</option>
              <option value="Cancelled">Annulé</option>
            </select>
            <select
              className="input-field"
              value={priorityFilter}
              onChange={(e) => setPriorityFilter(e.target.value as TaskPriority | 'all')}
            >
              <option value="all">Toutes les priorités</option>
              <option value="Low">Basse</option>
              <option value="Medium">Moyenne</option>
              <option value="High">Haute</option>
              <option value="Critical">Critique</option>
            </select>
          </div>
        </div>
      </div>

      {/* Task Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {filteredTasks.map((task) => (
          <div key={task.id} className="bg-white rounded-lg shadow-md p-6">
            <h3 className="font-medium text-gray-900">{task.title}</h3>
            <p className="text-gray-600 mt-2">{task.description}</p>
            <div className="mt-4 flex items-center justify-between">
              <span className={`px-2 py-1 rounded-full text-xs font-medium ${
                task.status === 'Todo' ? 'bg-gray-100 text-gray-800' :
                task.status === 'InProgress' ? 'bg-blue-100 text-blue-800' :
                task.status === 'Completed' ? 'bg-green-100 text-green-800' :
                'bg-red-100 text-red-800'
              }`}>
                {task.status}
              </span>
              <span className={`px-2 py-1 rounded-full text-xs font-medium ${
                task.priority === 'Critical' ? 'bg-red-100 text-red-800' :
                task.priority === 'High' ? 'bg-orange-100 text-orange-800' :
                task.priority === 'Medium' ? 'bg-yellow-100 text-yellow-800' :
                'bg-green-100 text-green-800'
              }`}>
                {task.priority}
              </span>
            </div>
          </div>
        ))}
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