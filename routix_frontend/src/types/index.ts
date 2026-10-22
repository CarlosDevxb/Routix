export type RolUsuario = 'administrador' | 'profesor' | 'alumno' | 'tecnico';
export type EstadoDispositivo = 'activo' | 'mantenimiento' | 'baja' | 'extraviado';
export type EstadoIncidente = 'abierto' | 'en_revision' | 'resuelto' | 'cerrado';
export type SeveridadIncidente = 'baja' | 'media' | 'alta' | 'critica';

export interface Usuario {
  id: number;
  matricula: string;
  nombre: string;
  email: string;
  rol: RolUsuario;
  creado_en?: string;
}

export interface LoginResponse {
  token: string;
  tipo_token: string;
  usuario: Usuario;
}

export interface ApiError {
  success: boolean;
  status: number;
  error: string;
}

