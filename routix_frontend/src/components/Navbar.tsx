import { Show } from 'solid-js';
import type { Component } from 'solid-js';
import { A } from '@solidjs/router';
import { useAuth } from '../context/auth';

export const Navbar: Component = () => {
  const { user, logout } = useAuth();

  return (
    <nav class="bg-slate-800 border-b border-slate-700 px-6 py-3.5 flex items-center justify-between shadow-md">
      <div class="flex items-center space-x-6">
        <A href="/" class="flex items-center space-x-2 text-xl font-bold tracking-tight text-white">
          <span class="bg-indigo-600 text-white p-1.5 rounded-lg text-sm font-black">RX</span>
          <span>Routix</span>
          <span class="text-xs bg-indigo-500/20 text-indigo-400 px-2 py-0.5 rounded-full border border-indigo-500/30">Lab Arq</span>
        </A>

        <Show when={user()}>
          <div class="hidden md:flex space-x-4 text-sm font-medium">
            <A href="/dashboard" class="text-slate-300 hover:text-white transition-colors" activeClass="text-indigo-400 font-semibold">
              Panel
            </A>
            <A href="/estaciones" class="text-slate-300 hover:text-white transition-colors" activeClass="text-indigo-400 font-semibold">
              Estaciones
            </A>
            <A href="/dispositivos" class="text-slate-300 hover:text-white transition-colors" activeClass="text-indigo-400 font-semibold">
              Inventario
            </A>
            <A href="/incidentes" class="text-slate-300 hover:text-white transition-colors" activeClass="text-indigo-400 font-semibold">
              Tickets
            </A>
          </div>
        </Show>
      </div>

      <div class="flex items-center space-x-4">
        <Show
          when={user()}
          fallback={
            <A
              href="/login"
              class="bg-indigo-600 hover:bg-indigo-500 text-white px-4 py-1.5 rounded-lg text-sm font-medium transition-colors shadow"
            >
              Iniciar Sesión
            </A>
          }
        >
          <div class="flex items-center space-x-3">
            <div class="text-right">
              <div class="text-sm font-semibold text-white leading-tight">{user()?.nombre}</div>
              <div class="text-xs text-indigo-400 capitalize">{user()?.rol} ({user()?.matricula})</div>
            </div>
            <button
              onClick={logout}
              class="bg-slate-700 hover:bg-rose-600/80 text-slate-300 hover:text-white px-3 py-1.5 rounded-lg text-xs font-medium transition-colors cursor-pointer"
            >
              Salir
            </button>
          </div>
        </Show>
      </div>
    </nav>
  );
};
