import { router } from '../utils/router.js';

// Navbar con sintaxis moderna
export const navbar = {
  
  // Template con template literals y emojis bolivianos
  template() {
    return `
      <nav class="navbar">
        <div class="nav-container">
          <h1 class="nav-logo">🇧🇴 Venta Libre</h1>
          
          <ul class="nav-menu">
            <li class="nav-item">
              <a href="/" class="nav-link" data-path="/">
                🏠 Inicio
              </a>
            </li>
            <li class="nav-item">
              <a href="/users" class="nav-link" data-path="/users">
                👥 Usuarios
              </a>
            </li>
            <li class="nav-item">
              <a href="/products" class="nav-link" data-path="/products">
                🛍️ Productos
              </a>
            </li>
          </ul>
          
          <div class="nav-actions">
            <button class="btn btn-primary">
              ➕ Publicar
            </button>
          </div>
        </div>
      </nav>
    `;
  },

  // Renderizar navbar
  render(container) {
    container.insertAdjacentHTML('beforebegin', this.template());
    this.attachEvents();
  },

  // Event listeners con delegation moderna
  attachEvents() {
    const navbar = document.querySelector('.navbar');
    
    // Delegación de eventos para links SPA
    navbar?.addEventListener('click', (event) => {
      const link = event.target.closest('[data-path]');
      
      if (link) {
        event.preventDefault();
        const path = link.dataset.path;
        router.navigateTo(path);
        this.setActiveLink(path);
      }
    });
  },

  // Marcar link activo con sintaxis moderna
  setActiveLink(currentPath) {
    document.querySelectorAll('.nav-link').forEach(link => {
      const isActive = link.dataset.path === currentPath;
      link.classList.toggle('active', isActive);
    });
  }
};