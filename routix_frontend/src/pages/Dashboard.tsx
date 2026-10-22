import { createSignal, onMount, Show, For } from 'solid-js';
import type { Component } from 'solid-js';
import { useAuth } from '../context/auth';
import { apiFetch } from '../api/client';
import type { Usuario } from '../types';

interface HealthData {
  status: string;
  service: string;
  database: string;
}

export const Dashboard: Component = () => {
  const { user } = useAuth();
  const [health, setHealth] = createSignal<HealthData | null>(null);
  const [usuarios, setUsuarios] = createSignal<Usuario[]>([]);

  onMount(async () => {
    try {
      const h = await apiFetch<HealthData>('/health');
      setHealth(h);

      if (user() && user()?.rol !== 'alumno') {
        const u = await apiFetch<Usuario[]>('/usuarios');
        setUsuarios(u);
      }
    } catch (e) {
      console.error('Error cargando datos del dashboard:', e);
    }
  });

  return (
    <div class="max-w-7xl mx-auto px-6 py-8">
      {/* Header */}
      <div class="flex flex-col md:flex-row md:items-center justify-between pb-6 border-b border-slate-800 gap-4">
        <div>
          <h1 class="text-3xl font-bold tracking-tight text-white">Panel de Control</h1>
          <p class="text-slate-400 text-sm mt-1">
            Laboratorio de Arquitectura de Computadoras &bull; Bienvenido, <span class="text-indigo-400 font-semibold">{user()?.nombre || 'Usuario'}</span>
          </p>
        </div>

        {/* Server & DB Status Badge */}
        <div class="flex items-center space-x-3 bg-slate-800/80 border border-slate-700 px-4 py-2 rounded-xl text-xs">
          <div class="flex items-center space-x-1.5">
            <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse"></span>
            <span class="text-slate-300 font-medium">Backend:</span>
            <span class="text-emerald-400 font-semibold">{health()?.status || 'Conectando...'}</span>
          </div>
          <span class="text-slate-600">|</span>
          <div class="flex items-center space-x-1.5">
            <span class="text-slate-300 font-medium">PostgreSQL:</span>
            <span class="text-emerald-400 font-semibold">{health()?.database || 'OK'}</span>
          </div>
        </div>
      </div>

      {/* Metrics Cards */}
      <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mt-8">
        <div class="bg-slate-800/60 border border-slate-700/60 p-5 rounded-2xl shadow-sm">
          <div class="text-xs uppercase tracking-wider font-semibold text-slate-400">Estaciones de Trabajo</div>
          <div class="text-3xl font-extrabold text-white mt-2">12</div>
          <div class="text-xs text-emerald-400 mt-2 font-medium">&bull; 10 Activas / 2 Mantenimiento</div>
        </div>

        <div class="bg-slate-800/60 border border-slate-700/60 p-5 rounded-2xl shadow-sm">
          <div class="text-xs uppercase tracking-wider font-semibold text-slate-400">Dispositivos en Inventario</div>
          <div class="text-3xl font-extrabold text-white mt-2">48</div>
          <div class="text-xs text-indigo-400 mt-2 font-medium">&bull; FPGAs, Osciloscopios, PCs</div>
        </div>

        <div class="bg-slate-800/60 border border-slate-700/60 p-5 rounded-2xl shadow-sm">
          <div class="text-xs uppercase tracking-wider font-semibold text-slate-400">Tickets de Soporte</div>
          <div class="text-3xl font-extrabold text-amber-400 mt-2">3</div>
          <div class="text-xs text-slate-400 mt-2 font-medium">&bull; 1 Abierto / 2 En Revisión</div>
        </div>

        <div class="bg-slate-800/60 border border-slate-700/60 p-5 rounded-2xl shadow-sm">
          <div class="text-xs uppercase tracking-wider font-semibold text-slate-400">Usuarios Registrados</div>
          <div class="text-3xl font-extrabold text-white mt-2">{usuarios().length || 2}</div>
          <div class="text-xs text-indigo-400 mt-2 font-medium">&bull; Administradores y Alumnos</div>
        </div>
      </div>

      {/* User Directory Table for Staff */}
      <Show when={user() && user()?.rol !== 'alumno'}>
        <div class="mt-10 bg-slate-800/50 border border-slate-700/70 rounded-2xl p-6 shadow-sm">
          <div class="flex items-center justify-between mb-4">
            <h2 class="text-lg font-bold text-white">Directorio de Usuarios del Laboratorio</h2>
            <span class="text-xs bg-indigo-500/10 text-indigo-400 border border-indigo-500/20 px-2.5 py-1 rounded-full font-medium">
              Vista Administrativa (RBAC)
            </span>
          </div>

          <div class="overflow-x-auto">
            <table class="w-full text-left text-sm">
              <thead class="text-xs uppercase tracking-wider text-slate-400 border-b border-slate-700/80">
                <tr>
                  <th class="py-3 px-4">ID</th>
                  <th class="py-3 px-4">Matrícula</th>
                  <th class="py-3 px-4">Nombre</th>
                  <th class="py-3 px-4">Correo</th>
                  <th class="py-3 px-4">Rol</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-slate-700/40">
                <For each={usuarios()}>
                  {(u) => (
                    <tr class="hover:bg-slate-750/50 transition-colors">
                      <td class="py-3 px-4 font-mono text-xs text-slate-400">#{u.id}</td>
                      <td class="py-3 px-4 font-medium text-white">{u.matricula}</td>
                      <td class="py-3 px-4 text-slate-200">{u.nombre}</td>
                      <td class="py-3 px-4 text-slate-400">{u.email}</td>
                      <td class="py-3 px-4">
                        <span class="inline-block px-2.5 py-0.5 rounded-full text-xs font-semibold uppercase tracking-wider bg-slate-700 text-indigo-300">
                          {u.rol}
                        </span>
                      </td>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
          </div>
        </div>
      </Show>
    </div>
  );
};
