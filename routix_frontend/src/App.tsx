import type { Component, JSX } from 'solid-js';
import { Router, Route } from '@solidjs/router';
import { AuthProvider } from './context/auth';
import { Navbar } from './components/Navbar';
import { Dashboard } from './pages/Dashboard';
import { Login } from './pages/Login';

const Layout = (props: { children?: JSX.Element }) => {
  return (
    <div class="min-h-screen bg-slate-900 text-slate-100 flex flex-col">
      <Navbar />
      <main class="flex-1">{props.children}</main>
    </div>
  );
};

const App: Component = () => {
  return (
    <AuthProvider>
      <Router root={Layout}>
        <Route path="/" component={Dashboard} />
        <Route path="/dashboard" component={Dashboard} />
        <Route path="/login" component={Login} />
      </Router>
    </AuthProvider>
  );
};

export default App;
