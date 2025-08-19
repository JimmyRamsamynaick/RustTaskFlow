import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { AuthProvider } from './contexts/AuthContext';
import { TaskProvider } from './contexts/TaskContext';
import { WebSocketProvider } from './contexts/WebSocketContext';
import { Notifications, useNotifications } from './components/Notifications';
import Header from './components/Header';
import Dashboard from './pages/Dashboard';
import Projects from './pages/Projects';
import Login from './pages/Login';
import Register from './pages/Register';
import ProtectedRoute from './components/ProtectedRoute';
import type { Task } from './types';

function App() {
  const { notifications, addNotification, removeNotification } = useNotifications();

  const handleTaskCreated = (task: Task) => {
    // Task creation will be handled by TaskContext
    console.log('Task created via WebSocket:', task);
  };

  const handleTaskUpdated = (task: Task) => {
    // Task update will be handled by TaskContext
    console.log('Task updated via WebSocket:', task);
  };

  const handleTaskDeleted = (taskId: string) => {
    // Task deletion will be handled by TaskContext
    console.log('Task deleted via WebSocket:', taskId);
  };

  const handleNotification = (message: string, type: 'info' | 'success' | 'warning' | 'error', userId?: string) => {
    addNotification({ message, type, userId });
  };

  return (
    <AuthProvider>
      <TaskProvider>
        <WebSocketProvider
          onTaskCreated={handleTaskCreated}
          onTaskUpdated={handleTaskUpdated}
          onTaskDeleted={handleTaskDeleted}
          onNotification={handleNotification}
        >
          <Router>
            <div className="min-h-screen bg-gray-50">
              <Header />
              <main className="container mx-auto px-4 py-8">
                <Routes>
                  <Route path="/login" element={<Login />} />
                  <Route path="/register" element={<Register />} />
                  <Route
                    path="/"
                    element={
                      <ProtectedRoute>
                        <Dashboard />
                      </ProtectedRoute>
                    }
                  />
                  <Route
                    path="/projects"
                    element={
                      <ProtectedRoute>
                        <Projects />
                      </ProtectedRoute>
                    }
                  />
                </Routes>
              </main>
              <Notifications 
                notifications={notifications} 
                onRemove={removeNotification} 
              />
            </div>
          </Router>
        </WebSocketProvider>
      </TaskProvider>
    </AuthProvider>
  );
}

export default App;
