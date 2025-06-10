// CategoryList - Navegación horizontal de categorías
export const CategoryList = {
  
  // Categorías predefinidas para Bolivia
  categories: [
    { id: 'all', name: 'Todas', icon: '📂', color: '#d32f2f' },
    { id: 'tech', name: 'Tecnología', icon: '📱', color: '#1976d2' },
    { id: 'clothing', name: 'Ropa', icon: '👕', color: '#e91e63' },
    { id: 'home', name: 'Hogar', icon: '🏠', color: '#4caf50' },
    { id: 'vehicles', name: 'Vehículos', icon: '🚗', color: '#ff5722' },
    { id: 'sports', name: 'Deportes', icon: '⚽', color: '#3f51b5' },
    { id: 'books', name: 'Libros', icon: '📚', color: '#795548' },
    { id: 'music', name: 'Música', icon: '🎵', color: '#9c27b0' },
    { id: 'tools', name: 'Herramientas', icon: '🔧', color: '#607d8b' },
    { id: 'beauty', name: 'Belleza', icon: '💄', color: '#e91e63' }
  ],

  selectedCategory: 'all',
  onCategoryChange: null,

  // Template de la lista de categorías
  template() {
    return `
      <div class="category-list-container">
        <div class="category-list" role="tablist">
          ${this.categories.map(category => this.categoryTemplate(category)).join('')}
        </div>
      </div>
    `;
  },

  // Template de categoría individual
  categoryTemplate(category) {
    const isActive = this.selectedCategory === category.id;
    
    return `
      <button 
        class="category-item ${isActive ? 'active' : ''}"
        data-category="${category.id}"
        role="tab"
        aria-selected="${isActive}"
        style="--category-color: ${category.color}"
      >
        <span class="category-icon">${category.icon}</span>
        <span class="category-name">${category.name}</span>
      </button>
    `;
  },

  // Renderizar componente
  render(container, options = {}) {
    this.selectedCategory = options.selectedCategory || 'all';
    this.onCategoryChange = options.onCategoryChange || null;
    
    container.innerHTML = this.template();
    this.attachEvents(container);
    
    // Auto-scroll para mostrar categoría activa
    this.scrollToActive();
  },

  // Event listeners
  attachEvents(container) {
    const categoryList = container.querySelector('.category-list');
    
    categoryList?.addEventListener('click', (event) => {
      const categoryButton = event.target.closest('.category-item');
      
      if (categoryButton) {
        const categoryId = categoryButton.dataset.category;
        this.selectCategory(categoryId);
      }
    });

    // Scroll horizontal con mouse wheel
    categoryList?.addEventListener('wheel', (event) => {
      if (Math.abs(event.deltaX) > Math.abs(event.deltaY)) return;
      
      event.preventDefault();
      categoryList.scrollLeft += event.deltaY;
    });
  },

  // Seleccionar categoría
  selectCategory(categoryId) {
    if (this.selectedCategory === categoryId) return;
    
    // Actualizar estado
    const previousCategory = this.selectedCategory;
    this.selectedCategory = categoryId;
    
    // Actualizar UI
    this.updateActiveCategory(previousCategory, categoryId);
    
    // Callback para notificar cambio
    if (this.onCategoryChange) {
      const selectedCat = this.categories.find(cat => cat.id === categoryId);
      this.onCategoryChange(categoryId, selectedCat);
    }
  },

  // Actualizar categoría activa en UI
  updateActiveCategory(previousId, newId) {
    // Remover clase activa anterior
    const previousButton = document.querySelector(`[data-category="${previousId}"]`);
    previousButton?.classList.remove('active');
    previousButton?.setAttribute('aria-selected', 'false');
    
    // Agregar clase activa nueva
    const newButton = document.querySelector(`[data-category="${newId}"]`);
    newButton?.classList.add('active');
    newButton?.setAttribute('aria-selected', 'true');
    
    // Auto-scroll a la categoría seleccionada
    this.scrollToActive();
  },

  // Scroll automático a categoría activa
  scrollToActive() {
    setTimeout(() => {
      const activeButton = document.querySelector('.category-item.active');
      const container = document.querySelector('.category-list');
      
      if (activeButton && container) {
        const containerRect = container.getBoundingClientRect();
        const buttonRect = activeButton.getBoundingClientRect();
        
        // Si el botón está fuera del viewport, hacer scroll
        if (buttonRect.left < containerRect.left || buttonRect.right > containerRect.right) {
          const scrollPosition = activeButton.offsetLeft - (container.offsetWidth / 2) + (activeButton.offsetWidth / 2);
          container.scrollTo({
            left: scrollPosition,
            behavior: 'smooth'
          });
        }
      }
    }, 100);
  },

  // Obtener categoría por ID
  getCategoryById(id) {
    return this.categories.find(cat => cat.id === id);
  },

  // Obtener todas las categorías
  getAllCategories() {
    return this.categories;
  },

  // Agregar nueva categoría (para futuras expansiones)
  addCategory(category) {
    if (!this.categories.find(cat => cat.id === category.id)) {
      this.categories.push(category);
    }
  }
};