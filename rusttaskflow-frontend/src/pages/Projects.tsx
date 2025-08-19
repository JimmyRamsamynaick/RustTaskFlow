import React, { useState, useEffect } from 'react';
import { Plus, Search, FolderPlus } from 'lucide-react';
import ProjectCard from '../components/ProjectCard';
import type { Project, CreateProjectRequest, InviteUserRequest } from '../types';
import { useAuth } from '../contexts/AuthContext';

const Projects: React.FC = () => {
  const { user } = useAuth();
  const [projects, setProjects] = useState<Project[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');
  const [showCreateForm, setShowCreateForm] = useState(false);
  const [newProject, setNewProject] = useState<CreateProjectRequest>({
    name: '',
    description: '',
  });
  const [isCreating, setIsCreating] = useState(false);

  // Mock data pour la démonstration
  useEffect(() => {
    // Simuler le chargement des projets
    setTimeout(() => {
      const mockProjects: Project[] = [
        {
          id: '1',
          name: 'Application Mobile',
          description: 'Développement d\'une application mobile pour la gestion de tâches',
          created_by: user?.id || '1',
          created_at: '2024-01-15T10:00:00Z',
          updated_at: '2024-01-20T15:30:00Z',
          members: [
            {
              user_id: user?.id || '1',
              username: user?.username || 'Vous',
              email: user?.email || 'vous@exemple.com',
              role: 'Owner',
              joined_at: '2024-01-15T10:00:00Z',
            },
            {
              user_id: '2',
              username: 'Alice',
              email: 'alice@exemple.com',
              role: 'Admin',
              joined_at: '2024-01-16T09:00:00Z',
            },
            {
              user_id: '3',
              username: 'Bob',
              email: 'bob@exemple.com',
              role: 'Member',
              joined_at: '2024-01-17T14:00:00Z',
            },
          ],
          tasks: [
            {
              id: '1',
              title: 'Conception UI/UX',
              description: 'Créer les maquettes de l\'application',
              status: 'Completed',
              priority: 'High',
              tags: ['design', 'ui'],
              created_by: '2',
              created_at: '2024-01-15T10:00:00Z',
              updated_at: '2024-01-18T16:00:00Z',
            },
            {
              id: '2',
              title: 'Développement API',
              description: 'Implémenter les endpoints REST',
              status: 'InProgress',
              priority: 'High',
              tags: ['backend', 'api'],
              created_by: user?.id || '1',
              created_at: '2024-01-16T11:00:00Z',
              updated_at: '2024-01-20T10:00:00Z',
            },
          ],
        },
        {
          id: '2',
          name: 'Site Web Corporate',
          description: 'Refonte du site web de l\'entreprise',
          created_by: '2',
          created_at: '2024-01-10T08:00:00Z',
          updated_at: '2024-01-19T12:00:00Z',
          members: [
            {
              user_id: user?.id || '1',
              username: user?.username || 'Vous',
              email: user?.email || 'vous@exemple.com',
              role: 'Member',
              joined_at: '2024-01-12T10:00:00Z',
            },
            {
              user_id: '2',
              username: 'Alice',
              email: 'alice@exemple.com',
              role: 'Owner',
              joined_at: '2024-01-10T08:00:00Z',
            },
          ],
          tasks: [
            {
              id: '3',
              title: 'Audit SEO',
              description: 'Analyser le référencement actuel',
              status: 'Todo',
              priority: 'Medium',
              tags: ['seo', 'audit'],
              created_by: '2',
              created_at: '2024-01-10T08:00:00Z',
              updated_at: '2024-01-10T08:00:00Z',
            },
          ],
        },
      ];
      setProjects(mockProjects);
      setIsLoading(false);
    }, 1000);
  }, [user]);

  const filteredProjects = projects.filter(project =>
    project.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    project.description?.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleCreateProject = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newProject.name.trim()) return;

    setIsCreating(true);
    try {
      // Simuler la création d'un projet
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      const createdProject: Project = {
        id: Date.now().toString(),
        name: newProject.name,
        description: newProject.description,
        created_by: user?.id || '1',
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString(),
        members: [
          {
            user_id: user?.id || '1',
            username: user?.username || 'Vous',
            email: user?.email || 'vous@exemple.com',
            role: 'Owner',
            joined_at: new Date().toISOString(),
          },
        ],
        tasks: [],
      };
      
      setProjects(prev => [createdProject, ...prev]);
      setNewProject({ name: '', description: '' });
      setShowCreateForm(false);
    } catch (error) {
      console.error('Error creating project:', error);
    } finally {
      setIsCreating(false);
    }
  };

  const handleInviteUser = async (invitation: InviteUserRequest) => {
    try {
      // Simuler l'envoi d'invitation
      await new Promise(resolve => setTimeout(resolve, 1000));
      console.log('Invitation envoyée:', invitation);
      // Ici, vous pourriez mettre à jour l'état local ou recharger les projets
    } catch (error) {
      console.error('Error sending invitation:', error);
      throw error;
    }
  };

  const handleDeleteProject = async (projectId: string) => {
    try {
      // Simuler la suppression
      await new Promise(resolve => setTimeout(resolve, 500));
      setProjects(prev => prev.filter(p => p.id !== projectId));
    } catch (error) {
      console.error('Error deleting project:', error);
      throw error;
    }
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="animate-spin rounded-full h-32 w-32 border-b-2 border-blue-500"></div>
      </div>
    );
  }

  return (
    <div className="p-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 mb-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Projets</h1>
          <p className="text-gray-600 mt-1">
            Gérez vos projets et collaborez avec votre équipe
          </p>
        </div>
        <button
          onClick={() => setShowCreateForm(true)}
          className="flex items-center space-x-2 bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition-colors"
        >
          <Plus className="h-4 w-4" />
          <span>Nouveau projet</span>
        </button>
      </div>

      {/* Search */}
      <div className="mb-6">
        <div className="relative max-w-md">
          <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-gray-400" />
          <input
            type="text"
            placeholder="Rechercher des projets..."
            className="w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
        </div>
      </div>

      {/* Projects Grid */}
      {filteredProjects.length > 0 ? (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredProjects.map((project) => (
            <ProjectCard
              key={project.id}
              project={project}
              onInviteUser={handleInviteUser}
              onDeleteProject={handleDeleteProject}
              currentUserId={user?.id || '1'}
            />
          ))}
        </div>
      ) : (
        <div className="text-center py-12">
          <FolderPlus className="h-12 w-12 text-gray-400 mx-auto mb-4" />
          <h3 className="text-lg font-medium text-gray-900 mb-2">
            {searchTerm ? 'Aucun projet trouvé' : 'Aucun projet'}
          </h3>
          <p className="text-gray-600 mb-4">
            {searchTerm
              ? 'Essayez de modifier votre recherche'
              : 'Créez votre premier projet pour commencer à collaborer'}
          </p>
          {!searchTerm && (
            <button
              onClick={() => setShowCreateForm(true)}
              className="inline-flex items-center space-x-2 bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 transition-colors"
            >
              <Plus className="h-4 w-4" />
              <span>Créer un projet</span>
            </button>
          )}
        </div>
      )}

      {/* Create Project Modal */}
      {showCreateForm && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg shadow-xl w-full max-w-md mx-4">
            <div className="flex items-center justify-between p-6 border-b">
              <h2 className="text-lg font-semibold text-gray-900">
                Nouveau projet
              </h2>
              <button
                onClick={() => setShowCreateForm(false)}
                className="text-gray-400 hover:text-gray-600 transition-colors"
              >
                ×
              </button>
            </div>

            <form onSubmit={handleCreateProject} className="p-6">
              <div className="mb-4">
                <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-2">
                  Nom du projet *
                </label>
                <input
                  type="text"
                  id="name"
                  value={newProject.name}
                  onChange={(e) => setNewProject(prev => ({ ...prev, name: e.target.value }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="Mon super projet"
                  required
                />
              </div>

              <div className="mb-6">
                <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-2">
                  Description
                </label>
                <textarea
                  id="description"
                  value={newProject.description}
                  onChange={(e) => setNewProject(prev => ({ ...prev, description: e.target.value }))}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                  placeholder="Description du projet..."
                  rows={3}
                />
              </div>

              <div className="flex justify-end space-x-3">
                <button
                  type="button"
                  onClick={() => setShowCreateForm(false)}
                  className="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 hover:bg-gray-200 rounded-md transition-colors"
                >
                  Annuler
                </button>
                <button
                  type="submit"
                  disabled={isCreating || !newProject.name.trim()}
                  className="px-4 py-2 text-sm font-medium text-white bg-blue-600 hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed rounded-md transition-colors flex items-center space-x-2"
                >
                  {isCreating ? (
                    <>
                      <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white"></div>
                      <span>Création...</span>
                    </>
                  ) : (
                    <span>Créer le projet</span>
                  )}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default Projects;