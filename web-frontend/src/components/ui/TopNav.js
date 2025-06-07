import { router } from '../../utils/router.js';

// TopNav - Navegación superior para desktop
export const TopNav = {
  
  // Estado del usuario (fake por ahora)
  user: {
    name: 'Daniel',
    avatar: '👤',
    notifications: 3,
    isLoggedIn: true
  },

  // Template principal
  template() {
    return `
      <header class="top-nav" role="banner">
        <nav class="top-nav-container">
          <!-- Logo y título -->
          <div class="nav-brand">
            <button class="brand-link" data-path="/">
              <span class="brand-logo">🇧🇴</span>
              <span class="brand-text">Venta Libre</span>
            </button>
          </div>

          <!-- Navegación principal -->
          <div class="nav-menu">
            <a href="/" class="nav-link" data-path="/">
              <span class="nav-icon">🏠</span>
              <span class="nav-text">Inicio</span>
            </a>
            <a href="/search" class="nav-link" data-path="/search">
              <span class="nav-icon">🔍</span>
              <span class="nav-text">Buscar</span>
            </a>
            <a href="/categories" class="nav-link" data-path="/categories">
              <span class="nav-icon">📂</span>
              <span class="nav-text">Categorías</span>
            </a>
            <a href="/help" class="nav-link" data-path="/help">
              <span class="nav-icon">❓</span>
              <span class="nav-text">Ayuda</span>
            </a>
          </div>

          <!-- Acciones del usuario -->
          <div class="nav-actions">
            <!-- Publicar producto (CTA principal) -->
            <button class="btn-publish" data-path="/add-product">
              <span class="publish-icon">➕</span>
              <span class="publish-text">Publicar</span>
            </button>

            <!-- Mensajes -->
            <button class="nav-action-btn" data-path="/messages" aria-label="Mensajes">
              <span class="action-icon">💬</span>
              ${this.user.notifications > 0 ? `<span class="action-badge">${this.user.notifications}</span>` : ''}
            </button>

            <!-- Notificaciones -->
            <button class="nav-action-btn" id="notifications-btn" aria-label="Notificaciones">
              <span class="action-icon">🔔</span>
              <span class="action-badge">3</span>
            </button>

            <!-- Menú de usuario -->
            <div class="user-menu">
              <button class="user-menu-trigger" id="user-menu-trigger">
                <span class="user-avatar">${this.user.avatar}</span>
                <span class="user-name">${this.user.name}</span>
                <span class="dropdown-arrow">▼</span>
              </button>
              
              <!-- Dropdown menu (inicialmente oculto) -->
              <div class="user-dropdown" id="user-dropdown">
                <div class="dropdown-header">
                  <div class="user-info">
                    <span class="user-avatar-large">${this.user.avatar}</span>
                    <div class="user-details">
                      <div class="user-name-large">${this.user.name}</div>
                      <div class="user-email">daniel@ejemplo.com</div>
                    </div>
                  </div>
                </div>
                
                <div class="dropdown-menu">
                  <a href="/profile" class="dropdown-item" data-path="/profile">
                    <span class="item-icon">👤</span>
                    <span class="item-text">Mi perfil</span>
                  </a>
                  <a href="/my-products" class="dropdown-item" data-path="/my-products">
                    <span class="item-icon">📦</span>
                    <span class="item-text">Mis productos</span>
                  </a>
                  <a href="/favorites" class="dropdown-item" data-path="/favorites">
                    <span class="item-icon">❤️</span>
                    <span class="item-text">Favoritos</span>
                  </a>
                  <a href="/purchases" class="dropdown-item" data-path="/purchases">
                    <span class="item-icon">🛒</span>
                    <span class="item-text">Mis compras</span>
                  </a>
                  <div class="dropdown-divider"></div>
                  <a href="/settings" class="dropdown-item" data-path="/settings">
                    <span class="item-icon">⚙️</span>
                    <span class="item-text">Configuración</span>
                  </a>
                  <button class="dropdown-item logout-btn" id="logout-btn">
                    <span class="item-icon">🚪</span>
                    <span class="item-text">Cerrar sesión</span>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </nav>
      </header>
    `;
  },

  // Renderizar componente
  render(container, options = {}) {
    // Solo renderizar en desktop
    if (window.innerWidth < 768) return;
    
    // Insertar antes del contenido principal
    if (!document.querySelector('.top-nav')) {
      document.body.insertAdjacentHTML('afterbegin', this.template());
      this.attachEvents();
      this.updateActiveLink(window.location.pathname);
    }
  },

  // Event listeners
  attachEvents() {
    const topNav = document.querySelector('.top-nav');
    
    // Navegación por clicks
    topNav?.addEventListener('click', (event) => {
      const target = event.target.closest('[data-path]');
      
      if (target) {
        event.preventDefault();
        const path = target.dataset.path;
        this.navigateToPath(path);
      }
    });

    // Toggle user dropdown
    const userMenuTrigger = document.getElementById('user-menu-trigger');
    const userDropdown = document.getElementById('user-dropdown');
    
    userMenuTrigger?.addEventListener('click', (event) => {
      event.stopPropagation();
      this.toggleUserDropdown();
    });

    // Cerrar dropdown al hacer click fuera
    document.addEventListener('click', (event) => {
      if (!event.target.closest('.user-menu')) {
        this.closeUserDropdown();
      }
    });

    // Notificaciones
    const notificationsBtn = document.getElementById('notifications-btn');
    notificationsBtn?.addEventListener('click', () => {
      this.showNotifications();
    });

    // Logout
    const logoutBtn = document.getElementById('logout-btn');
    logoutBtn?.addEventListener('click', () => {
      this.handleLogout();
    });

    // Responsive: ocultar/mostrar según tamaño de pantalla
    window.addEventListener('resize', () => {
      this.handleResize();
    });
  },

  // Navegar a ruta
  navigateToPath(path) {
    this.updateActiveLink(path);
    router.navigateTo(path);
    this.closeUserDropdown();
  },

  // Actualizar link activo
  updateActiveLink(currentPath) {
    const navLinks = document.querySelectorAll('.top-nav .nav-link');
    
    navLinks.forEach(link => {
      const isActive = link.dataset.path === currentPath;
      link.classList.toggle('active', isActive);
    });
  },

  // Toggle dropdown de usuario
  toggleUserDropdown() {
    const dropdown = document.getElementById('user-dropdown');
    const trigger = document.getElementById('user-menu-trigger');
    
    const isOpen = dropdown?.classList.contains('show');
    
    if (isOpen) {
      this.closeUserDropdown();
    } else {
      this.openUserDropdown();
    }
  },

  // Abrir dropdown
  openUserDropdown() {
    const dropdown = document.getElementById('user-dropdown');
    const trigger = document.getElementById('user-menu-trigger');
    
    dropdown?.classList.add('show');
    trigger?.classList.add('active');
  },

  // Cerrar dropdown
  closeUserDropdown() {
    const dropdown = document.getElementById('user-dropdown');
    const trigger = document.getElementById('user-menu-trigger');
    
    dropdown?.classList.remove('show');
    trigger?.classList.remove('active');
  },

  // Mostrar notificaciones
  showNotifications() {
    // TODO: Implementar panel de notificaciones
    alert('🔔 3 notificaciones nuevas:\n• Nuevo mensaje de Carlos\n• Tu producto "iPhone" tiene interés\n• Actualización de precios');
  },

  // Manejar logout
  handleLogout() {
    if (confirm('¿Estás seguro que quieres cerrar sesión?')) {
      // TODO: Limpiar datos de usuario
      this.user.isLoggedIn = false;
      router.navigateTo('/login');
    }
  },

  // Manejar cambios de tamaño de pantalla
  handleResize() {
    const topNav = document.querySelector('.top-nav');
    
    if (window.innerWidth < 768) {
      // Móvil: ocultar top nav
      topNav?.classList.add('hidden');
    } else {
      // Desktop: mostrar top nav
      topNav?.classList.remove('hidden');
    }
  },

  // Actualizar badge de notificaciones
  updateNotificationBadge(count) {
    const badge = document.querySelector('#notifications-btn .action-badge');
    if (badge) {
      if (count > 0) {
        badge.textContent = count > 99 ? '99+' : count;
        badge.style.display = 'flex';
      } else {
        badge.style.display = 'none';
      }
    }
  },

  // Actualizar badge de mensajes
  updateMessagesBadge(count) {
    const messagesBtn = document.querySelector('[data-path="/messages"] .action-badge');
    if (messagesBtn) {
      if (count > 0) {
        messagesBtn.textContent = count > 99 ? '99+' : count;
        messagesBtn.style.display = 'flex';
      } else {
        messagesBtn.style.display = 'none';
      }
    }
  },

  // Actualizar información del usuario
  updateUser(userData) {
    this.user = { ...this.user, ...userData };
    
    // Re-renderizar si está visible
    const topNav = document.querySelector('.top-nav');
    if (topNav) {
      const container = document.body;
      topNav.remove();
      this.render(container);
    }
  },

  // Remover componente
  destroy() {
    const topNav = document.querySelector('.top-nav');
    topNav?.remove();
  }
};