import React, { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { Mail, Lock, User, AlertCircle } from 'lucide-react';
import { useAuth } from '../contexts/AuthContext';
import type { RegisterRequest } from '../types';

const Register: React.FC = () => {
  const [formData, setFormData] = useState<RegisterRequest>({
    username: '',
    email: '',
    password: '',
  });
  const [error, setError] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const { register, isAuthenticated } = useAuth();
  const navigate = useNavigate();

  React.useEffect(() => {
    if (isAuthenticated) {
      navigate('/');
    }
  }, [isAuthenticated, navigate]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value,
    });
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');
    setIsLoading(true);

    try {
      await register(formData);
      navigate('/');
    } catch (err) {
      setError('Erreur lors de la création du compte');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div style={{ minHeight: '100vh', display: 'flex', alignItems: 'center', justifyContent: 'center', padding: '3rem 1rem' }}>
      <div style={{ maxWidth: '28rem', width: '100%' }}>
        <div style={{ marginBottom: '2rem' }}>
          <h2 style={{ marginTop: '1.5rem', textAlign: 'center', fontSize: '1.875rem', fontWeight: '800', color: 'rgb(17 24 39)' }}>
            Créer votre compte
          </h2>
          <p style={{ marginTop: '0.5rem', textAlign: 'center', fontSize: '0.875rem', color: 'rgb(75 85 99)' }}>
            Ou{' '}
            <Link
              to="/login"
              style={{ fontWeight: '500', color: 'rgb(37 99 235)', textDecoration: 'none' }}
              onMouseEnter={(e) => (e.target as HTMLElement).style.color = 'rgb(29 78 216)'}
              onMouseLeave={(e) => (e.target as HTMLElement).style.color = 'rgb(37 99 235)'}
            >
              connectez-vous à votre compte existant
            </Link>
          </p>
        </div>
        <form style={{ marginTop: '2rem' }} onSubmit={handleSubmit}>
          {error && (
            <div style={{ backgroundColor: 'rgb(254 242 242)', border: '1px solid rgb(254 202 202)', borderRadius: '0.375rem', padding: '1rem', marginBottom: '1.5rem' }}>
              <div style={{ display: 'flex' }}>
                <AlertCircle style={{ height: '1.25rem', width: '1.25rem', color: 'rgb(248 113 113)' }} />
                <div style={{ marginLeft: '0.75rem' }}>
                  <p style={{ fontSize: '0.875rem', color: 'rgb(153 27 27)' }}>{error}</p>
                </div>
              </div>
            </div>
          )}
          
          <div style={{ marginBottom: '1.5rem' }}>
            <div style={{ marginBottom: '1rem' }}>
              <label htmlFor="username" style={{ display: 'block', fontSize: '0.875rem', fontWeight: '500', color: 'rgb(55 65 81)' }}>
                Nom d'utilisateur
              </label>
              <div style={{ marginTop: '0.25rem', position: 'relative' }}>
                <div style={{ position: 'absolute', top: 0, left: 0, paddingLeft: '0.75rem', display: 'flex', alignItems: 'center', height: '100%', pointerEvents: 'none' }}>
                  <User style={{ height: '1.25rem', width: '1.25rem', color: 'rgb(156 163 175)' }} />
                </div>
                <input
                  id="username"
                  name="username"
                  type="text"
                  required
                  className="input-field"
                  style={{ paddingLeft: '2.5rem' }}
                  placeholder="Votre nom d'utilisateur"
                  value={formData.username}
                  onChange={handleChange}
                />
              </div>
            </div>
            
            <div style={{ marginBottom: '1rem' }}>
              <label htmlFor="email" style={{ display: 'block', fontSize: '0.875rem', fontWeight: '500', color: 'rgb(55 65 81)' }}>
                Email
              </label>
              <div style={{ marginTop: '0.25rem', position: 'relative' }}>
                <div style={{ position: 'absolute', top: 0, left: 0, paddingLeft: '0.75rem', display: 'flex', alignItems: 'center', height: '100%', pointerEvents: 'none' }}>
                  <Mail style={{ height: '1.25rem', width: '1.25rem', color: 'rgb(156 163 175)' }} />
                </div>
                <input
                  id="email"
                  name="email"
                  type="email"
                  required
                  className="input-field"
                  style={{ paddingLeft: '2.5rem' }}
                  placeholder="votre@email.com"
                  value={formData.email}
                  onChange={handleChange}
                />
              </div>
            </div>
            
            <div>
              <label htmlFor="password" style={{ display: 'block', fontSize: '0.875rem', fontWeight: '500', color: 'rgb(55 65 81)' }}>
                Mot de passe
              </label>
              <div style={{ marginTop: '0.25rem', position: 'relative' }}>
                <div style={{ position: 'absolute', top: 0, left: 0, paddingLeft: '0.75rem', display: 'flex', alignItems: 'center', height: '100%', pointerEvents: 'none' }}>
                  <Lock style={{ height: '1.25rem', width: '1.25rem', color: 'rgb(156 163 175)' }} />
                </div>
                <input
                  id="password"
                  name="password"
                  type="password"
                  required
                  className="input-field"
                  style={{ paddingLeft: '2.5rem' }}
                  placeholder="Choisissez un mot de passe"
                  value={formData.password}
                  onChange={handleChange}
                />
              </div>
            </div>
          </div>

          <div style={{ marginTop: '1.5rem' }}>
            <button
              type="submit"
              disabled={isLoading}
              className="btn-primary"
              style={{ 
                width: '100%', 
                opacity: isLoading ? 0.5 : 1, 
                cursor: isLoading ? 'not-allowed' : 'pointer' 
              }}
            >
              {isLoading ? 'Création...' : 'Créer le compte'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Register;