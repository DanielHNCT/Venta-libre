import { apiService } from '../services/api.js';
import { ProductCard } from '../components/ui/ProductCard.js';
import { CategoryList } from '../components/ui/CategoryList.js';
import { SearchBar } from '../components/ui/SearchBar.js';
import { BottomNav } from '../components/ui/BottomNav.js';

// Página Home con marketplace completo
export const homePage = {
  
  // Estado de la página
  products: [],
  filteredProducts: [],
  selectedCategory: 'all',
  searchTerm: '',
  filters: {},
  isLoading: false,

  // Template principal del home
  template() {
    return `
      <div class="home-page">
        <!-- Header con ubicación -->
        <header class="home-header">
          <div class="location-info">
            <h1 class="app-title">🇧🇴 Venta Libre</h1>
            <div class="user-location">
              <span class="location-icon">📍</span>
              <span class="location-text">La Paz, Bolivia</span>
              <button class="change-location" aria-label="Cambiar ubicación">📍</button>
            </div>
          </div>
          <button class="notifications-btn" aria-label="Notificaciones">
            🔔
            <span class="notification-badge">3</span>
          </button>
        </header>

        <!-- Barra de búsqueda -->
        <section class="search-section">
          <div id="search-container"></div>
        </section>

        <!-- Lista de categorías -->
        <section class="categories-section">
          <div id="categories-container"></div>
        </section>

        <!-- Estadísticas rápidas -->
        <section class="stats-section">
          <div class="stats-container">
            <div class="stat-item">
              <span class="stat-number" id="products-count">${this.products.length}</span>
              <span class="stat-label">Productos</span>
            </div>
            <div class="stat-item">
              <span class="stat-number">24</span>
              <span class="stat-label">Usuarios activos</span>
            </div>
            <div class="stat-item">
              <span class="stat-number">156</span>
              <span class="stat-label">Ventas hoy</span>
            </div>
          </div>
        </section>

        <!-- Lista de productos -->
        <section class="products-section">
          <div class="products-header">
            <h2>
              ${this.selectedCategory === 'all' ? 'Todos los productos' : this.getCategoryName()}
              <span class="products-count">(${this.filteredProducts.length})</span>
            </h2>
            <button class="view-toggle" id="view-toggle" aria-label="Cambiar vista">
              <span class="toggle-icon">⊞</span>
            </button>
          </div>
          
          <div id="products-container" class="products-grid">
            ${this.isLoading ? this.loadingTemplate() : ''}
          </div>
        </section>
      </div>
    `;
  },

  // Template de loading
  loadingTemplate() {
    return `
      <div class="loading-products">
        <div class="loading-skeleton"></div>
        <div class="loading-skeleton"></div>
        <div class="loading-skeleton"></div>
        <div class="loading-text">Cargando productos...</div>
      </div>
    `;
  },

  // Renderizar página completa
  async render(container) {
    container.innerHTML = this.template();
    
    // Renderizar componentes UI
    this.renderComponents();
    
    // Cargar datos
    await this.loadData();
    
    // Configurar eventos
    this.attachEvents();
    
    // Renderizar navegación inferior
    BottomNav.render(container);
  },

  // Renderizar componentes UI
  renderComponents() {
    // SearchBar
    const searchContainer = document.getElementById('search-container');
    SearchBar.render(searchContainer, {
      onSearch: (term, filters) => this.handleSearch(term, filters),
      onFilterChange: (filters) => this.handleFilterChange(filters)
    });

    // CategoryList
    const categoriesContainer = document.getElementById('categories-container');
    CategoryList.render(categoriesContainer, {
      selectedCategory: this.selectedCategory,
      onCategoryChange: (categoryId, category) => this.handleCategoryChange(categoryId, category)
    });
  },

  // Cargar datos desde API y generar productos fake
  async loadData() {
    try {
      this.isLoading = true;
      this.updateProductsContainer();
      
      // Cargar usuarios reales de la API
      const users = await apiService.users.getAll();
      
      // Generar productos fake usando usuarios reales
      this.products = this.generateFakeProducts(users);
      this.filteredProducts = [...this.products];
      
      this.isLoading = false;
      this.renderProducts();
      this.updateStats();
      
    } catch (error) {
      this.handleError(error);
    }
  },

  // Generar productos fake para demostración
  generateFakeProducts(users) {
    const categories = ['tech', 'clothing', 'home', 'vehicles', 'sports'];
    const products = [];
    const productTemplates = [
      { title: 'iPhone 13 Pro', price: 5500, category: 'tech', condition: 'Usado' },
      { title: 'MacBook Air M1', price: 8500, category: 'tech', condition: 'Nuevo' },
      { title: 'Mesa de madera', price: 800, category: 'home', condition: 'Usado' },
      { title: 'Bicicleta de montaña', price: 2500, category: 'sports', condition: 'Nuevo' },
      { title: 'Jeans Levis', price: 350, category: 'clothing', condition: 'Usado' },
      { title: 'Toyota Corolla 2018', price: 85000, category: 'vehicles', condition: 'Usado' },
      { title: 'PlayStation 5', price: 4500, category: 'tech', condition: 'Nuevo' },
      { title: 'Sofá 3 plazas', price: 1200, category: 'home', condition: 'Usado' },
      { title: 'Chaqueta de cuero', price: 450, category: 'clothing', condition: 'Nuevo' },
      { title: 'Laptop HP Pavilion', price: 3200, category: 'tech', condition: 'Usado' }
    ];

    productTemplates.forEach((template, index) => {
      const user = users[index % users.length] || { name: 'Usuario Anónimo' };
      const timeAgo = Math.floor(Math.random() * 24) + 1;
      const distance = (Math.random() * 20 + 0.5).toFixed(1);
      
      products.push({
        id: index + 1,
        title: template.title,
        price: template.price,
        category: template.category,
        condition: template.condition,
        seller: user.name,
        sellerId: user.id,
        location: 'La Paz',
        distance: `${distance}km`,
        timeAgo: `Hace ${timeAgo}h`,
        image: `/images/products/product-${index + 1}.jpg`,
        description: `${template.title} en excelente estado. Contactar para más detalles.`
      });
    });

    return products;
  },

  // Manejar búsqueda
  handleSearch(searchTerm, filters) {
    this.searchTerm = searchTerm;
    this.filters = filters;
    this.applyFilters();
  },

  // Manejar cambio de filtros
  handleFilterChange(filters) {
    this.filters = filters;
    this.applyFilters();
  },

  // Manejar cambio de categoría
  handleCategoryChange(categoryId, category) {
    this.selectedCategory = categoryId;
    this.applyFilters();
    this.updateProductsHeader();
  },

  // Aplicar filtros y búsqueda
  applyFilters() {
    let filtered = [...this.products];

    // Filtro por categoría
    if (this.selectedCategory !== 'all') {
      filtered = filtered.filter(product => product.category === this.selectedCategory);
    }

    // Filtro por término de búsqueda
    if (this.searchTerm) {
      const term = this.searchTerm.toLowerCase();
      filtered = filtered.filter(product => 
        product.title.toLowerCase().includes(term) ||
        product.description.toLowerCase().includes(term) ||
        product.seller.toLowerCase().includes(term)
      );
    }

    // Filtros adicionales
    if (this.filters.condition && this.filters.condition !== 'all') {
      filtered = filtered.filter(product => 
        product.condition.toLowerCase() === this.filters.condition.toLowerCase()
      );
    }

    if (this.filters.minPrice || this.filters.maxPrice !== 10000) {
      filtered = filtered.filter(product => 
        product.price >= (this.filters.minPrice || 0) &&
        product.price <= (this.filters.maxPrice || Infinity)
      );
    }

    // Ordenamiento
    if (this.filters.sortBy) {
      filtered = this.sortProducts(filtered, this.filters.sortBy);
    }

    this.filteredProducts = filtered;
    this.renderProducts();
    this.updateProductsHeader();
  },

  // Ordenar productos
  sortProducts(products, sortBy) {
    const sorted = [...products];
    
    switch (sortBy) {
      case 'price_low':
        return sorted.sort((a, b) => a.price - b.price);
      case 'price_high':
        return sorted.sort((a, b) => b.price - a.price);
      case 'distance':
        return sorted.sort((a, b) => parseFloat(a.distance) - parseFloat(b.distance));
      case 'recent':
      default:
        return sorted.sort((a, b) => a.id - b.id); // Más recientes primero
    }
  },

  // Renderizar productos
  renderProducts() {
    const container = document.getElementById('products-container');
    ProductCard.renderList(this.filteredProducts, container);
  },

  // Actualizar contenedor de productos
  updateProductsContainer() {
    const container = document.getElementById('products-container');
    if (container && this.isLoading) {
      container.innerHTML = this.loadingTemplate();
    }
  },

  // Actualizar header de productos
  updateProductsHeader() {
    const header = document.querySelector('.products-header h2');
    if (header) {
      const categoryName = this.selectedCategory === 'all' ? 'Todos los productos' : this.getCategoryName();
      header.innerHTML = `
        ${categoryName}
        <span class="products-count">(${this.filteredProducts.length})</span>
      `;
    }
  },

  // Obtener nombre de categoría
  getCategoryName() {
    const category = CategoryList.getCategoryById(this.selectedCategory);
    return category ? `${category.icon} ${category.name}` : 'Categoría';
  },

  // Actualizar estadísticas
  updateStats() {
    const productsCount = document.getElementById('products-count');
    if (productsCount) {
      productsCount.textContent = this.products.length;
    }
  },

  // Event listeners de la página
  attachEvents() {
    // Cambio de ubicación
    const changeLocationBtn = document.querySelector('.change-location');
    changeLocationBtn?.addEventListener('click', () => {
      this.showLocationPicker();
    });

    // Notificaciones
    const notificationsBtn = document.querySelector('.notifications-btn');
    notificationsBtn?.addEventListener('click', () => {
      this.showNotifications();
    });

    // Toggle vista de productos
    const viewToggle = document.getElementById('view-toggle');
    viewToggle?.addEventListener('click', () => {
      this.toggleProductsView();
    });
  },

  // Mostrar selector de ubicación
  showLocationPicker() {
    // TODO: Implementar modal de ubicación
    alert('🚧 Selector de ubicación próximamente');
  },

  // Mostrar notificaciones
  showNotifications() {
    // TODO: Implementar panel de notificaciones
    alert('🔔 3 notificaciones nuevas');
  },

  // Toggle entre vista grid y lista
  toggleProductsView() {
    const container = document.getElementById('products-container');
    const toggleIcon = document.querySelector('.toggle-icon');
    
    if (container.classList.contains('products-list')) {
      container.classList.remove('products-list');
      container.classList.add('products-grid');
      toggleIcon.textContent = '☰';
    } else {
      container.classList.remove('products-grid');
      container.classList.add('products-list');
      toggleIcon.textContent = '⊞';
    }
  },

  // Manejo de errores
  handleError(error) {
    this.isLoading = false;
    const container = document.getElementById('products-container');
    
    if (container) {
      container.innerHTML = `
        <div class="error-state">
          <div class="error-icon">⚠️</div>
          <h3>Error al cargar productos</h3>
          <p>${error.message}</p>
          <button class="btn btn-primary" onclick="location.reload()">
            🔄 Reintentar
          </button>
        </div>
      `;
    }
  }
};