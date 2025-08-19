import React, { useState } from 'react';
import { Users, Calendar, UserPlus, Trash2 } from 'lucide-react';
import type { Project } from '../types';
import InviteUserModal from './InviteUserModal';

interface ProjectCardProps {
  project: Project;
  onInviteUser: (invitation: any) => Promise<void>;
  onDeleteProject?: (projectId: string) => Promise<void>;
  currentUserId: string;
}

const ProjectCard: React.FC<ProjectCardProps> = ({
  project,
  onInviteUser,
  onDeleteProject,
  currentUserId,
}) => {
  const [showInviteModal, setShowInviteModal] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const currentUserRole = project.members.find(m => m.user_id === currentUserId)?.role;
  const canInvite = currentUserRole === 'Owner' || currentUserRole === 'Admin';
  const canDelete = currentUserRole === 'Owner';

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleDateString('fr-FR', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
    });
  };

  const handleDelete = async () => {
    if (!onDeleteProject || !canDelete) return;
    
    if (window.confirm(`Êtes-vous sûr de vouloir supprimer le projet "${project.name}" ? Cette action est irréversible.`)) {
      setIsDeleting(true);
      try {
        await onDeleteProject(project.id);
      } catch (error) {
        console.error('Error deleting project:', error);
      } finally {
        setIsDeleting(false);
      }
    }
  };

  const getRoleColor = (role: string) => {
    switch (role) {
      case 'Owner':
        return 'bg-purple-100 text-purple-800';
      case 'Admin':
        return 'bg-blue-100 text-blue-800';
      case 'Member':
        return 'bg-green-100 text-green-800';
      case 'Viewer':
        return 'bg-gray-100 text-gray-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getRoleIcon = (role: string) => {
    switch (role) {
      case 'Owner':
        return '👑';
      case 'Admin':
        return '⚡';
      case 'Member':
        return '👤';
      case 'Viewer':
        return '👁️';
      default:
        return '👤';
    }
  };

  return (
    <>
      <div className="bg-white rounded-lg border border-gray-200 p-6 hover:shadow-md transition-shadow">
        <div className="flex items-start justify-between mb-4">
          <div className="flex-1">
            <h3 className="text-lg font-semibold text-gray-900 mb-2">
              {project.name}
            </h3>
            {project.description && (
              <p className="text-gray-600 text-sm mb-3">
                {project.description}
              </p>
            )}
          </div>
          
          <div className="flex items-center space-x-2 ml-4">
            {canInvite && (
              <button
                onClick={() => setShowInviteModal(true)}
                className="p-2 text-blue-600 hover:bg-blue-50 rounded-md transition-colors"
                title="Inviter un utilisateur"
              >
                <UserPlus className="h-4 w-4" />
              </button>
            )}
            
            {canDelete && onDeleteProject && (
              <button
                onClick={handleDelete}
                disabled={isDeleting}
                className="p-2 text-red-600 hover:bg-red-50 rounded-md transition-colors disabled:opacity-50"
                title="Supprimer le projet"
              >
                {isDeleting ? (
                  <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-red-600"></div>
                ) : (
                  <Trash2 className="h-4 w-4" />
                )}
              </button>
            )}
          </div>
        </div>

        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center space-x-4">
            <div className="flex items-center text-sm text-gray-500">
              <Users className="h-4 w-4 mr-1" />
              <span>{project.members.length} membre{project.members.length > 1 ? 's' : ''}</span>
            </div>
            
            <div className="flex items-center text-sm text-gray-500">
              <Calendar className="h-4 w-4 mr-1" />
              <span>{formatDate(project.created_at)}</span>
            </div>
          </div>
          
          {currentUserRole && (
            <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${getRoleColor(currentUserRole)}`}>
              <span className="mr-1">{getRoleIcon(currentUserRole)}</span>
              {currentUserRole}
            </span>
          )}
        </div>

        <div className="border-t pt-4">
          <div className="flex items-center justify-between">
            <div className="text-sm text-gray-600">
              <span className="font-medium">{project.tasks.length}</span> tâche{project.tasks.length > 1 ? 's' : ''}
            </div>
            
            <div className="flex -space-x-2">
              {project.members.slice(0, 3).map((member) => (
                <div
                  key={member.user_id}
                  className="w-8 h-8 bg-gradient-to-br from-blue-400 to-purple-500 rounded-full flex items-center justify-center text-white text-xs font-medium border-2 border-white"
                  title={`${member.username} (${member.role})`}
                >
                  {member.username.charAt(0).toUpperCase()}
                </div>
              ))}
              {project.members.length > 3 && (
                <div className="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center text-gray-600 text-xs font-medium border-2 border-white">
                  +{project.members.length - 3}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>

      {showInviteModal && (
        <InviteUserModal
          projectId={project.id}
          projectName={project.name}
          onClose={() => setShowInviteModal(false)}
          onInvite={onInviteUser}
        />
      )}
    </>
  );
};

export default ProjectCard;