# ==========================================
# 1. Builder Stage
# ==========================================
FROM node:22-alpine AS builder

WORKDIR /app

# Instalar dependencias
COPY package.json package-lock.json ./
RUN npm ci

# Copiar archivos fuente y compilar la aplicación
COPY tsconfig*.json vite.config.ts index.html ./
COPY public ./public
COPY src ./src

RUN npm run build

# ==========================================
# 2. Production Stage (Nginx)
# ==========================================
FROM nginx:alpine AS runtime

# Eliminar configuración por defecto de Nginx y copiar la personalizada
RUN rm -rf /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copiar los assets estáticos generados por Vite
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

