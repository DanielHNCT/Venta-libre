// Configuración de la API
const API_URL = 'http://192.168.1.120:3000/api/v1';

// Servicio API con objetos literales y sintaxis moderna
export const apiService = {
  
  // Método base para requests con async/await moderno
  async request(endpoint, options = {}) {
    try {
      const url = `${API_URL}${endpoint}`;
      const config = {
        headers: {
          'Content-Type': 'application/json',
          ...options.headers
        },
        ...options
      };

      const response = await fetch(url, config);
      
      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('🚨 API Error:', error);
      throw error;
    }
  },

  // Usuarios - sintaxis moderna con destructuring
  users: {
    async getAll() {
      return apiService.request('/users');
    },

    async getById(id) {
      return apiService.request(`/users/${id}`);
    },

    async create(userData) {
      return apiService.request('/users', {
        method: 'POST',
        body: JSON.stringify(userData)
      });
    },

    async update(id, userData) {
      return apiService.request(`/users/${id}`, {
        method: 'PUT',
        body: JSON.stringify(userData)
      });
    },

    async delete(id) {
      return apiService.request(`/users/${id}`, {
        method: 'DELETE'
      });
    }
  },

  // Futuro: productos, categorías, etc.
  products: {
    async getAll() {
      return apiService.request('/products');
    }
    // Más métodos cuando expandamos
  }
};