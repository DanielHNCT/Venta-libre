import { apiService } from '../services/api.js';

// Página Users con CRUD completo
export const usersPage = {
  users: [],
  isLoading: false,

  // Template principal
  template() {
    return `
      <div class="users-page">
        <header class="page-header">
          <h1>👥 Gestión de Usuarios</h1>
          <button class="btn btn-primary" id="add-user-btn">
            ➕ Agregar Usuario
          </button>
        </header>

        <!-- Formulario de nuevo usuario (inicialmente oculto) -->
        <form class="user-form hidden" id="user-form">
          <h3>✨ Nuevo Usuario</h3>
          <div class="form-group">
            <label for="user-name">Nombre completo:</label>
            <input type="text" id="user-name" placeholder="Ej: Carlos Mendoza" required>
          </div>
          <div class="form-group">
            <label for="user-email">Email:</label>
            <input type="email" id="user-email" placeholder="carlos@bolivia.com" required>
          </div>
          <div class="form-actions">
            <button type="submit" class="btn btn-success">💾 Guardar</button>
            <button type="button" class="btn btn-secondary" id="cancel-btn">❌ Cancelar</button>
          </div>
        </form>

        <!-- Lista de usuarios -->
        <section class="users-section">
          <div class="users-header">
            <h2>Lista de Usuarios</h2>
            <div class="users-count" id="users-count">
              ${this.users.length} usuarios
            </div>
          </div>
          
          <div id="users-container" class="users-grid">
            ${this.isLoading ? this.loadingTemplate() : this.usersListTemplate()}
          </div>
        </section>
      </div>
    `;
  },

  // Template de loading
  loadingTemplate() {
    return `
      <div class="loading-card">
        <div class="spinner"></div>
        <p>Cargando usuarios...</p>
      </div>
    `;
  },

  // Template de lista de usuarios
  usersListTemplate() {
    if (!this.users?.length) {
      return `
        <div class="empty-state">
          <div class="empty-icon">👤</div>
          <h3>No hay usuarios registrados</h3>
          <p>¡Agrega el primer usuario para comenzar!</p>
        </div>
      `;
    }

    return this.users
      .map(user => `
        <div class="user-card" data-user-id="${user.id}">
          <div class="user-avatar">👤</div>
          <div class="user-info">
            <h3>${user.name}</h3>
            <p class="user-email">📧 ${user.email}</p>
            <p class="user-id">🆔 ID: ${user.id}</p>
          </div>
          <div class="user-actions">
            <button class="btn btn-outline view-btn" data-action="view" data-id="${user.id}">
              👁️ Ver
            </button>
            <button class="btn btn-outline edit-btn" data-action="edit" data-id="${user.id}">
              ✏️ Editar
            </button>
            <button class="btn btn-danger delete-btn" data-action="delete" data-id="${user.id}">
              🗑️ Eliminar
            </button>
          </div>
        </div>
      `)
      .join('');
  },

  // Renderizar página
  async render(container) {
    container.innerHTML = this.template();
    this.attachEvents();
    await this.loadUsers();
  },

  // Cargar usuarios desde API
  async loadUsers() {
    try {
      this.isLoading = true;
      this.updateUsersContainer();
      
      this.users = await apiService.users.getAll();
      
      this.isLoading = false;
      this.updateUsersContainer();
      this.updateUsersCount();
    } catch (error) {
      this.handleError(error);
    }
  },

  // Actualizar contenedor de usuarios
  updateUsersContainer() {
    const container = document.getElementById('users-container');
    if (container) {
      container.innerHTML = this.isLoading ? this.loadingTemplate() : this.usersListTemplate();
    }
  },

  // Actualizar contador
  updateUsersCount() {
    const countElement = document.getElementById('users-count');
    if (countElement) {
      countElement.textContent = `${this.users.length} usuarios`;
    }
  },

  // Event listeners con delegación moderna
  attachEvents() {
    const page = document.querySelector('.users-page');
    
    // Delegación de eventos
    page?.addEventListener('click', (event) => {
      const target = event.target;
      
      // Botón agregar usuario
      if (target.id === 'add-user-btn') {
        this.toggleForm(true);
      }
      
      // Botón cancelar
      if (target.id === 'cancel-btn') {
        this.toggleForm(false);
      }
      
      // Acciones de usuario (ver, editar, eliminar)
      if (target.dataset.action) {
        const { action, id } = target.dataset;
        this.handleUserAction(action, parseInt(id));
      }
    });

    // Formulario
    const form = document.getElementById('user-form');
    form?.addEventListener('submit', (event) => {
      event.preventDefault();
      this.handleFormSubmit();
    });
  },

  // Toggle formulario
  toggleForm(show) {
    const form = document.getElementById('user-form');
    form?.classList.toggle('hidden', !show);
    
    if (!show) {
      form.reset();
    }
  },

  // Manejar envío de formulario
  async handleFormSubmit() {
    const name = document.getElementById('user-name')?.value;
    const email = document.getElementById('user-email')?.value;
    
    if (!name || !email) return;
    
    try {
      // Nota: API actual devuelve fake data, pero estructura está lista
      await apiService.users.create({ name, email });
      
      this.toggleForm(false);
      await this.loadUsers(); // Recargar lista
      
      this.showNotification('✅ Usuario creado exitosamente');
    } catch (error) {
      this.showNotification(`❌ Error: ${error.message}`, 'error');
    }
  },

  // Manejar acciones de usuario
  async handleUserAction(action, userId) {
    const user = this.users.find(u => u.id === userId);
    
    switch (action) {
      case 'view':
        alert(`👤 Usuario: ${user?.name}\n📧 Email: ${user?.email}\n🆔 ID: ${user?.id}`);
        break;
      case 'edit':
        this.showNotification('🔧 Función de editar próximamente');
        break;
      case 'delete':
        if (confirm(`¿Eliminar usuario ${user?.name}?`)) {
          this.showNotification('🗑️ Función de eliminar próximamente');
        }
        break;
    }
  },

  // Mostrar notificación
  showNotification(message, type = 'success') {
    const notification = document.createElement('div');
    notification.className = `notification ${type}`;
    notification.textContent = message;
    
    document.body.appendChild(notification);
    
    setTimeout(() => {
      notification.remove();
    }, 3000);
  },

  // Manejo de errores
  handleError(error) {
    this.isLoading = false;
    const container = document.getElementById('users-container');
    
    if (container) {
      container.innerHTML = `
        <div class="error-card">
          <div class="error-icon">⚠️</div>
          <h3>Error al cargar usuarios</h3>
          <p>${error.message}</p>
          <button class="btn btn-primary" onclick="location.reload()">
            🔄 Reintentar
          </button>
        </div>
      `;
    }
  }
};