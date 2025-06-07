import { apiService } from '../services/api.js';

// Página Home con sintaxis moderna
export const homePage = {
  
  // Template del home
  template() {
    return `
      <div class="home-page">
        <header class="hero">
          <h1>🇧🇴 Bienvenido a Venta Libre Bolivia</h1>
          <p>El marketplace que conecta a toda Bolivia</p>
          <div class="hero-stats" id="stats-container">
            <div class="stat-card">
              <div class="stat-number" id="users-count">...</div>
              <div class="stat-label">Usuarios Registrados</div>
            </div>
            <div class="stat-card">
              <div class="stat-number">0</div>
              <div class="stat-label">Productos</div>
            </div>
            <div class="stat-card">
              <div class="stat-number">0</div>
              <div class="stat-label">Ventas</div>
            </div>
          </div>
        </header>

        <section class="recent-users">
          <h2>👥 Últimos Usuarios</h2>
          <div id="users-preview" class="users-grid">
            <div class="loading">Cargando usuarios...</div>
          </div>
        </section>

        <section class="features">
          <h2>✨ Características</h2>
          <div class="features-grid">
            <div class="feature-card">
              <div class="feature-icon">⚡</div>
              <h3>Súper Rápido</h3>
              <p>Optimizado para conexiones lentas en Bolivia</p>
            </div>
            <div class="feature-card">
              <div class="feature-icon">📱</div>
              <h3>Mobile First</h3>
              <p>Diseñado para smartphones y tablets</p>
            </div>
            <div class="feature-card">
              <div class="feature-icon">🔒</div>
              <h3>Seguro</h3>
              <p>Transacciones protegidas</p>
            </div>
          </div>
        </section>
      </div>
    `;
  },

  // Renderizar página
  async render(container) {
    container.innerHTML = this.template();
    await this.loadData();
  },

  // Cargar datos con async/await moderno
  async loadData() {
    try {
      // Destructuring con await
      const users = await apiService.users.getAll();
      
      this.updateStats(users);
      this.renderUsersPreview(users);
    } catch (error) {
      this.handleError(error);
    }
  },

  // Actualizar estadísticas
  updateStats(users) {
    const usersCount = document.getElementById('users-count');
    if (usersCount) {
      usersCount.textContent = users.length;
    }
  },

  // Preview de usuarios con sintaxis moderna
  renderUsersPreview(users) {
    const container = document.getElementById('users-preview');
    
    if (!users?.length) {
      container.innerHTML = '<p>No hay usuarios registrados aún.</p>';
      return;
    }

    // Solo mostrar primeros 3 usuarios
    const recentUsers = users.slice(-3).reverse();
    
    container.innerHTML = recentUsers
      .map(user => `
        <div class="user-preview-card">
          <div class="user-avatar">👤</div>
          <h4>${user.name}</h4>
          <p>📧 ${user.email}</p>
        </div>
      `)
      .join('');
  },

  // Manejo de errores moderno
  handleError(error) {
    const container = document.getElementById('users-preview');
    if (container) {
      container.innerHTML = `
        <div class="error-message">
          ⚠️ Error al cargar datos: ${error.message}
        </div>
      `;
    }
  }
};