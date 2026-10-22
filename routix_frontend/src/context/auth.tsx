import {
  createContext,
  useContext,
  createSignal,
  onMount,
} from 'solid-js';
import type { JSX, ParentProps } from 'solid-js';
import type { LoginResponse, Usuario, RolUsuario } from '../types';
import { apiFetch } from '../api/client';

interface AuthContextType {
  user: () => Usuario | null;
  token: () => string | null;
  loading: () => boolean;
  login: (identificador: string, password: string) => Promise<void>;
  logout: () => void;
  hasRole: (roles: RolUsuario[]) => boolean;
}

const AuthContext = createContext<AuthContextType>();

export function AuthProvider(props: ParentProps): JSX.Element {
  const [user, setUser] = createSignal<Usuario | null>(null);
  const [token, setToken] = createSignal<string | null>(
    localStorage.getItem('routix_token')
  );
  const [loading, setLoading] = createSignal<boolean>(true);

  onMount(async () => {
    const savedToken = localStorage.getItem('routix_token');
    if (savedToken) {
      try {
        const currentUser = await apiFetch<Usuario>('/auth/me');
        setUser(currentUser);
      } catch (err) {
        console.error('Sesión expirada o inválida:', err);
        logout();
      }
    }
    setLoading(false);
  });

  const login = async (identificador: string, password: string) => {
    const res = await apiFetch<LoginResponse>('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ identificador, password }),
    });

    localStorage.setItem('routix_token', res.token);
    setToken(res.token);
    setUser(res.usuario);
  };

  const logout = () => {
    localStorage.removeItem('routix_token');
    setToken(null);
    setUser(null);
  };

  const hasRole = (roles: RolUsuario[]): boolean => {
    const u = user();
    if (!u) return false;
    return roles.includes(u.rol);
  };

  return (
    <AuthContext.Provider
      value={{
        user,
        token,
        loading,
        login,
        logout,
        hasRole,
      }}
    >
      {props.children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth debe ser utilizado dentro de un AuthProvider');
  }
  return context;
}
