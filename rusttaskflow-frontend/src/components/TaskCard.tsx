import React, { useState } from 'react';
import { Calendar, Trash2, Clock, Check } from 'lucide-react';
import type { Task, TaskStatus } from '../types';
import { useTask } from '../contexts/TaskContext';

interface TaskCardProps {
  task: Task;
}

const TaskCard: React.FC<TaskCardProps> = ({ task }) => {
  const { updateTask, deleteTask } = useTask();
  const [isUpdating, setIsUpdating] = useState(false);

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'Critical':
        return 'bg-red-100 text-red-800 border-red-200';
      case 'High':
        return 'bg-orange-100 text-orange-800 border-orange-200';
      case 'Medium':
        return 'bg-yellow-100 text-yellow-800 border-yellow-200';
      case 'Low':
        return 'bg-green-100 text-green-800 border-green-200';
      default:
        return 'bg-gray-100 text-gray-800 border-gray-200';
    }
  };

  const getPriorityLabel = (priority: string) => {
    switch (priority) {
      case 'high':
        return 'Haute';
      case 'medium':
        return 'Moyenne';
      case 'low':
        return 'Basse';
      default:
        return priority;
    }
  };



  const handleStatusChange = async (newStatus: TaskStatus) => {
    setIsUpdating(true);
    try {
      await updateTask(task.id, { status: newStatus });
    } catch (error) {
      console.error('Error updating task status:', error);
    } finally {
      setIsUpdating(false);
    }
  };

  const handleDelete = async () => {
    if (window.confirm('Êtes-vous sûr de vouloir supprimer cette tâche ?')) {
      try {
        await deleteTask(task.id);
      } catch (error) {
        console.error('Error deleting task:', error);
      }
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('fr-FR', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
    });
  };

  return (
    <>
      <div className="bg-white rounded-lg border border-gray-200 p-4 hover:shadow-md transition-shadow">
        <div className="flex items-start justify-between mb-3">
          <div className="flex items-start space-x-3 flex-1">
            <button
               onClick={() => handleStatusChange(task.status === 'Completed' ? 'Todo' : 'Completed')}
               className={`mt-1 p-2 rounded-md transition-all duration-200 transform hover:scale-105 ${
                 task.status === 'Completed'
                   ? 'bg-green-500 text-white hover:bg-green-600 shadow-md'
                   : 'border-2 border-gray-300 hover:border-green-500 hover:bg-green-50'
               }`}
               title={task.status === 'Completed' ? 'Marquer comme non terminé' : 'Marquer comme terminé'}
               disabled={isUpdating}
             >
               {task.status === 'Completed' ? (
                 <Check className="h-4 w-4" />
               ) : (
                 <div className="h-4 w-4" />
               )}
             </button>
             <h3 className={`font-medium flex-1 transition-all duration-200 ${
               task.status === 'Completed' ? 'text-gray-500 line-through' : 'text-gray-900'
             }`}>
              {task.title}
            </h3>
          </div>
          <div className="flex items-center space-x-1 ml-2">
            <button
              onClick={handleDelete}
              className="p-1 text-gray-400 hover:text-red-600 transition-colors"
            >
              <Trash2 className="h-4 w-4" />
            </button>
          </div>
        </div>

        {task.description && (
          <p className="text-gray-600 text-sm mb-3 line-clamp-2">{task.description}</p>
        )}

        <div className="flex items-center justify-between mb-3">
          <span
            className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border ${
              getPriorityColor(task.priority)
            }`}
          >
            {getPriorityLabel(task.priority)}
          </span>

          {task.due_date && (
            <div className="flex items-center text-xs text-gray-500">
              <Calendar className="h-3 w-3 mr-1" />
              {formatDate(task.due_date)}
            </div>
          )}
        </div>

        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <select
              value={task.status}
              onChange={(e) => handleStatusChange(e.target.value as TaskStatus)}
              disabled={isUpdating}
              className={`text-xs border rounded-md px-3 py-1.5 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-colors ${
                task.status === 'Completed' ? 'bg-green-50 border-green-300' :
                task.status === 'InProgress' ? 'bg-blue-50 border-blue-300' :
                'bg-gray-50 border-gray-300'
              }`}
            >
              <option value="Todo">📋 À faire</option>
              <option value="InProgress">⚡ En cours</option>
              <option value="Completed">✅ Terminé</option>
            </select>
            {isUpdating && (
              <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-500"></div>
            )}
          </div>

          <div className="flex items-center text-xs text-gray-400">
            <Clock className="h-3 w-3 mr-1" />
            {formatDate(task.created_at)}
          </div>
        </div>
      </div>


    </>
  );
};

export default TaskCard;