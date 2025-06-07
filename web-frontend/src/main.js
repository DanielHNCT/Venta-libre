import { router } from './utils/router.js';
import { navbar } from './components/navbar.js';
import { homePage } from './pages/home.js';
import { usersPage } from './pages/users.js';

// SPA Principal - Venta Libre Bolivia
const app = {
  
  // Inicializar aplicación
  init() {
    this.setupRoutes();
    this.renderNavbar();
    this.startRouter();
    this.setupGlobalEvents();
  },

  // Configurar rutas de la SPA
  setupRoutes() {
    router.addRoute('/', homePage);
    router.addRoute('/users', usersPage);
    
    // Ruta 404 (página no encontrada)
    router.addRoute('/404', {
      render(container) {
        container.innerHTML = `
          <div class="not-found-page">
            <div class="not-found-content">
              <h1>🚫 404</h1>
              <h2>Página no encontrada</h2>
              <p>La página que buscas no existe en Venta Libre Bolivia.</p>
              <button class="btn btn-primary" onclick="router.navigateTo('/')">
                🏠 Volver al inicio
              </button>
            </div>
          </div>
        `;
      }
    });
  },

  // Renderizar navbar
  renderNavbar() {
    const appContainer = document.getElementById('app');
    navbar.render(appContainer);
  },

  // Iniciar router
  startRouter() {
    router.init();
  },

  // Eventos globales
  setupGlobalEvents() {
    // Manejar errores globales
    window.addEventListener('error', (event) => {
      console.error('🚨 Error global:', event.error);
      this.showGlobalError('Ha ocurrido un error inesperado');
    });

    // Manejar errores de API no capturados
    window.addEventListener('unhandledrejection', (event) => {
      console.error('🚨 Promise rechazada:', event.reason);
      this.showGlobalError('Error de conexión con el servidor');
    });

    // Indicador de estado de conexión
    window.addEventListener('online', () => {
      this.showConnectionStatus(true);
    });

    window.addEventListener('offline', () => {
      this.showConnectionStatus(false);
    });
  },

  // Mostrar error global
  showGlobalError(message) {
    const errorDiv = document.createElement('div');
    errorDiv.className = 'global-error';
    errorDiv.innerHTML = `
      <div class="error-content">
        <span>⚠️ ${message}</span>
        <button onclick="this.parentElement.parentElement.remove()">✕</button>
      </div>
    `;
    
    document.body.insertAdjacentElement('afterbegin', errorDiv);
    
    // Auto-remover después de 5 segundos
    setTimeout(() => {
      errorDiv?.remove();
    }, 5000);
  },

  // Indicador de conexión
  showConnectionStatus(isOnline) {
    const statusDiv = document.createElement('div');
    statusDiv.className = `connection-status ${isOnline ? 'online' : 'offline'}`;
    statusDiv.innerHTML = `
      <div class="status-content">
        <span>${isOnline ? '🟢 Conectado' : '🔴 Sin conexión'}</span>
      </div>
    `;
    
    document.body.insertAdjacentElement('afterbegin', statusDiv);
    
    setTimeout(() => {
      statusDiv?.remove();
    }, 3000);
  }
};

// Auto-inicializar cuando el DOM esté listo
document.addEventListener('DOMContentLoaded', () => {
  console.log('🇧🇴 Iniciando Venta Libre Bolivia SPA...');
  app.init();
  console.log('✅ SPA inicializada correctamente');
});

// Exponer router globalmente para debugging
window.router = router;
window.app = app;

// Exponer en consola para desarrollo
if (import.meta.env?.DEV) {
  console.log('🔧 Modo desarrollo activado');
  console.log('💡 Usa router.navigateTo("/path") para navegar');
  console.log('💡 Usa app para acceder a funciones principales');
}