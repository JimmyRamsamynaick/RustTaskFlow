import React from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { LogOut, CheckSquare } from 'lucide-react';
import { useAuth } from '../contexts/AuthContext';

const Header: React.FC = () => {
  const { user, logout, isAuthenticated } = useAuth();
  const navigate = useNavigate();

  const handleLogout = () => {
    logout();
    navigate('/login');
  };

  return (
    <header className="bg-white shadow-sm border-b border-gray-200">
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          <Link to="/" className="flex items-center space-x-2">
            <CheckSquare className="h-8 w-8 text-primary-600" />
            <span className="text-xl font-bold text-gray-900">RustTaskFlow</span>
          </Link>

          {isAuthenticated ? (
            <div className="flex items-center space-x-4">
              <span className="text-gray-700">Bonjour, {user?.username}</span>
              <button
                onClick={handleLogout}
                className="flex items-center space-x-1 text-gray-600 hover:text-gray-900 transition-colors"
              >
                <LogOut className="h-4 w-4" />
                <span>Déconnexion</span>
              </button>
            </div>
          ) : (
            <div className="flex items-center space-x-4">
              <Link
                to="/login"
                className="text-gray-600 hover:text-gray-900 transition-colors"
              >
                Connexion
              </Link>
              <Link
                to="/register"
                className="btn-primary"
              >
                Inscription
              </Link>
            </div>
          )}
        </div>
      </div>
    </header>
  );
};

export default Header;