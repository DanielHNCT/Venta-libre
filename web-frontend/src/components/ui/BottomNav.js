import { router } from '../../utils/router.js';

// BottomNav - Navegación inferior para móvil
export const BottomNav = {
  
  // Rutas de navegación
  navItems: [
    {
      id: 'home',
      path: '/',
      icon: '🏠',
      label: 'Inicio',
      badge: null
    },
    {
      id: 'search',
      path: '/search',
      icon: '🔍',
      label: 'Buscar',
      badge: null
    },
    {
      id: 'add',
      path: '/add-product',
      icon: '➕',
      label: 'Publicar',
      badge: null,
      primary: true // Botón destacado
    },
    {
      id: 'messages',
      path: '/messages',
      icon: '💬',
      label: 'Mensajes',
      badge: 0 // Número de mensajes no leídos
    },
    {
      id: 'profile',
      path: '/profile',
      icon: '👤',
      label: 'Perfil',
      badge: null
    }
  ],

  currentPath: '/',
  onNavigate: null,

  // Template principal
  template() {
    return `
      <nav class="bottom-nav" role="navigation" aria-label="Navegación principal">
        <div class="bottom-nav-container">
          ${this.navItems.map(item => this.navItemTemplate(item)).join('')}
        </div>
      </nav>
    `;
  },

  // Template de item individual
  navItemTemplate(item) {
    const isActive = this.currentPath === item.path;
    const hasBadge = item.badge !== null && item.badge > 0;
    
    return `
      <button 
        class="nav-item ${isActive ? 'active' : ''} ${item.primary ? 'primary' : ''}"
        data-path="${item.path}"
        data-nav-id="${item.id}"
        role="tab"
        aria-selected="${isActive}"
        aria-label="${item.label}"
      >
        <div class="nav-icon-container">
          <span class="nav-icon">${item.icon}</span>
          ${hasBadge ? `<span class="nav-badge">${item.badge > 99 ? '99+' : item.badge}</span>` : ''}
        </div>
        <span class="nav-label">${item.label}</span>
        
        ${isActive ? '<div class="nav-indicator"></div>' : ''}
      </button>
    `;
  },

  // Renderizar componente
  render(container, options = {}) {
    this.currentPath = window.location.pathname;
    this.onNavigate = options.onNavigate || null;
    
    // Insertar al final del body (navegación fija)
    if (!document.querySelector('.bottom-nav')) {
      document.body.insertAdjacentHTML('beforeend', this.template());
      this.attachEvents();
    }
    
    this.updateActiveItem(this.currentPath);
  },

  // Event listeners
  attachEvents() {
    const bottomNav = document.querySelector('.bottom-nav');
    
    bottomNav?.addEventListener('click', (event) => {
      const navItem = event.target.closest('.nav-item');
      
      if (navItem) {
        const path = navItem.dataset.path;
        const navId = navItem.dataset.navId;
        
        this.navigateToPath(path, navId);
      }
    });

    // Actualizar navegación cuando cambie la ruta
    window.addEventListener('popstate', () => {
      this.updateActiveItem(window.location.pathname);
    });
  },

  // Navegar a ruta
  navigateToPath(path, navId) {
    // Prevenir navegación si ya estamos en esa ruta
    if (this.currentPath === path) return;
    
    // Callback personalizado antes de navegar
    if (this.onNavigate) {
      const shouldNavigate = this.onNavigate(path, navId);
      if (shouldNavigate === false) return;
    }
    
    // Actualizar estado
    const previousPath = this.currentPath;
    this.currentPath = path;
    
    // Actualizar UI
    this.updateActiveItem(path, previousPath);
    
    // Navegar usando el router
    router.navigateTo(path);
    
    // Vibración táctil en móviles (si está disponible)
    this.hapticFeedback();
  },

  // Actualizar item activo
  updateActiveItem(newPath, previousPath = null) {
    const navItems = document.querySelectorAll('.nav-item');
    
    navItems.forEach(item => {
      const itemPath = item.dataset.path;
      const isActive = itemPath === newPath;
      
      // Actualizar clases y atributos
      item.classList.toggle('active', isActive);
      item.setAttribute('aria-selected', isActive);
      
      // Agregar/quitar indicador
      const existingIndicator = item.querySelector('.nav-indicator');
      if (isActive && !existingIndicator) {
        item.insertAdjacentHTML('beforeend', '<div class="nav-indicator"></div>');
      } else if (!isActive && existingIndicator) {
        existingIndicator.remove();
      }
    });
    
    // Actualizar estado interno
    this.currentPath = newPath;
  },

  // Actualizar badge de mensajes
  updateMessagesBadge(count) {
    const messagesItem = this.navItems.find(item => item.id === 'messages');
    if (messagesItem) {
      messagesItem.badge = count;
      
      // Actualizar UI si está renderizado
      const messagesNav = document.querySelector('[data-nav-id="messages"]');
      if (messagesNav) {
        const badgeElement = messagesNav.querySelector('.nav-badge');
        
        if (count > 0) {
          const badgeText = count > 99 ? '99+' : count;
          if (badgeElement) {
            badgeElement.textContent = badgeText;
          } else {
            const iconContainer = messagesNav.querySelector('.nav-icon-container');
            iconContainer?.insertAdjacentHTML('beforeend', `<span class="nav-badge">${badgeText}</span>`);
          }
        } else {
          badgeElement?.remove();
        }
      }
    }
  },

  // Actualizar badge personalizado
  updateBadge(navId, count) {
    const navItem = this.navItems.find(item => item.id === navId);
    if (navItem) {
      navItem.badge = count;
      
      // Actualizar UI
      const navElement = document.querySelector(`[data-nav-id="${navId}"]`);
      if (navElement) {
        const badgeElement = navElement.querySelector('.nav-badge');
        
        if (count > 0) {
          const badgeText = count > 99 ? '99+' : count;
          if (badgeElement) {
            badgeElement.textContent = badgeText;
          } else {
            const iconContainer = navElement.querySelector('.nav-icon-container');
            iconContainer?.insertAdjacentHTML('beforeend', `<span class="nav-badge">${badgeText}</span>`);
          }
        } else {
          badgeElement?.remove();
        }
      }
    }
  },

  // Feedback haptico para móviles
  hapticFeedback() {
    if ('vibrate' in navigator) {
      navigator.vibrate(50); // Vibración corta
    }
  },

  // Mostrar/ocultar navegación (para scroll automático)
  show() {
    const bottomNav = document.querySelector('.bottom-nav');
    bottomNav?.classList.remove('hidden');
  },

  hide() {
    const bottomNav = document.querySelector('.bottom-nav');
    bottomNav?.classList.add('hidden');
  },

  // Auto-hide en scroll (opcional)
  setupAutoHide() {
    let lastScrollY = window.scrollY;
    let ticking = false;
    
    const handleScroll = () => {
      const currentScrollY = window.scrollY;
      
      if (currentScrollY > lastScrollY && currentScrollY > 100) {
        // Scrolling down
        this.hide();
      } else {
        // Scrolling up
        this.show();
      }
      
      lastScrollY = currentScrollY;
      ticking = false;
    };
    
    window.addEventListener('scroll', () => {
      if (!ticking) {
        requestAnimationFrame(handleScroll);
        ticking = true;
      }
    });
  },

  // Remover navegación (cleanup)
  destroy() {
    const bottomNav = document.querySelector('.bottom-nav');
    bottomNav?.remove();
  }
};