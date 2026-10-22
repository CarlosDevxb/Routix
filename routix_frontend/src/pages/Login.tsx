import { createSignal, Show } from 'solid-js';
import type { Component } from 'solid-js';
import { useNavigate } from '@solidjs/router';
import { useAuth } from '../context/auth';

export const Login: Component = () => {
  const { login } = useAuth();
  const navigate = useNavigate();

  const [identificador, setIdentificador] = createSignal('admin@routix.edu');
  const [password, setPassword] = createSignal('admin_password_2026');
  const [error, setError] = createSignal<string | null>(null);
  const [loading, setLoading] = createSignal(false);

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setError(null);
    setLoading(true);

    try {
      await login(identificador(), password());
      navigate('/dashboard');
    } catch (err: unknown) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError('Ocurrió un error inesperado al iniciar sesión');
      }
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="min-h-[85vh] flex items-center justify-center px-4">
      <div class="max-w-md w-full bg-slate-800/90 border border-slate-700/80 rounded-2xl p-8 shadow-2xl backdrop-blur-sm">
        <div class="text-center mb-8">
          <div class="inline-flex items-center justify-center w-12 h-12 bg-indigo-600 rounded-xl mb-3 shadow-lg shadow-indigo-500/20">
            <span class="text-white text-xl font-bold">RX</span>
          </div>
          <h2 class="text-2xl font-bold text-white tracking-tight">Acceso a Routix</h2>
          <p class="text-slate-400 text-sm mt-1">Laboratorio de Arquitectura de Computadoras</p>
        </div>

        <Show when={error()}>
          <div class="mb-6 p-3.5 bg-rose-500/10 border border-rose-500/30 rounded-xl text-rose-400 text-sm flex items-center space-x-2">
            <span>⚠️</span>
            <span>{error()}</span>
          </div>
        </Show>

        <form onSubmit={handleSubmit} class="space-y-5">
          <div>
            <label class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-2">
              Matrícula o Correo
            </label>
            <input
              type="text"
              required
              value={identificador()}
              onInput={(e) => setIdentificador(e.currentTarget.value)}
              class="w-full bg-slate-900/80 border border-slate-700 rounded-xl px-4 py-2.5 text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-all text-sm"
              placeholder="Ej. ADM001 o admin@routix.edu"
            />
          </div>

          <div>
            <label class="block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-2">
              Contraseña
            </label>
            <input
              type="password"
              required
              value={password()}
              onInput={(e) => setPassword(e.currentTarget.value)}
              class="w-full bg-slate-900/80 border border-slate-700 rounded-xl px-4 py-2.5 text-white placeholder-slate-500 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-all text-sm"
              placeholder="••••••••"
            />
          </div>

          <button
            type="submit"
            disabled={loading()}
            class="w-full bg-indigo-600 hover:bg-indigo-500 disabled:opacity-50 text-white font-medium py-2.5 rounded-xl transition-all shadow-lg shadow-indigo-600/30 text-sm flex justify-center items-center cursor-pointer"
          >
            <Show when={loading()} fallback="Iniciar Sesión">
              <span>Ingresando...</span>
            </Show>
          </button>
        </form>

        <div class="mt-6 pt-6 border-t border-slate-700/60 text-xs text-slate-400 text-center">
          <p>Credenciales de prueba cargadas automáticamente.</p>
        </div>
      </div>
    </div>
  );
};

