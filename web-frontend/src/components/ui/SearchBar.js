// SearchBar - Barra de búsqueda con filtros
export const SearchBar = {
  
  // Estado del componente
  searchTerm: '',
  filters: {
    maxDistance: 50, // km
    minPrice: 0,
    maxPrice: 10000, // Bs
    condition: 'all', // all, nuevo, usado
    sortBy: 'recent' // recent, price_low, price_high, distance
  },
  
  onSearch: null,
  onFilterChange: null,
  showFilters: false,

  // Template principal
  template() {
    return `
      <div class="search-bar-container">
        <!-- Barra de búsqueda principal -->
        <div class="search-input-container">
          <div class="search-input-wrapper">
            <span class="search-icon">🔍</span>
            <input 
              type="text" 
              class="search-input" 
              placeholder="Buscar productos..."
              value="${this.searchTerm}"
              autocomplete="off"
            >
            <button class="clear-search ${this.searchTerm ? 'visible' : ''}" type="button">
              <span>✕</span>
            </button>
          </div>
          
          <button class="filter-toggle ${this.hasActiveFilters() ? 'active' : ''}" type="button">
            <span class="filter-icon">🔧</span>
            ${this.hasActiveFilters() ? '<span class="filter-badge">●</span>' : ''}
          </button>
        </div>

        <!-- Panel de filtros (inicialmente oculto) -->
        <div class="filters-panel ${this.showFilters ? 'show' : ''}">
          ${this.filtersTemplate()}
        </div>

        <!-- Chips de filtros activos -->
        ${this.activeFiltersTemplate()}
      </div>
    `;
  },

  // Template de filtros
  filtersTemplate() {
    return `
      <div class="filters-content">
        <div class="filters-header">
          <h3>Filtros</h3>
          <button class="btn-reset-filters" type="button">Limpiar todo</button>
        </div>

        <!-- Distancia -->
        <div class="filter-group">
          <label class="filter-label">📍 Distancia máxima</label>
          <div class="distance-slider">
            <input 
              type="range" 
              class="slider" 
              id="distance-slider"
              min="1" 
              max="100" 
              value="${this.filters.maxDistance}"
            >
            <div class="slider-value">Hasta ${this.filters.maxDistance}km</div>
          </div>
        </div>

        <!-- Precio -->
        <div class="filter-group">
          <label class="filter-label">💰 Rango de precio (Bs)</label>
          <div class="price-inputs">
            <input 
              type="number" 
              class="price-input" 
              id="min-price"
              placeholder="Mín"
              value="${this.filters.minPrice || ''}"
              min="0"
            >
            <span class="price-separator">-</span>
            <input 
              type="number" 
              class="price-input" 
              id="max-price"
              placeholder="Máx"
              value="${this.filters.maxPrice || ''}"
              min="0"
            >
          </div>
        </div>

        <!-- Condición -->
        <div class="filter-group">
          <label class="filter-label">🏷️ Condición</label>
          <div class="condition-buttons">
            <button 
              class="condition-btn ${this.filters.condition === 'all' ? 'active' : ''}" 
              data-condition="all"
            >
              Todas
            </button>
            <button 
              class="condition-btn ${this.filters.condition === 'nuevo' ? 'active' : ''}"
              data-condition="nuevo"
            >
              Nuevo
            </button>
            <button 
              class="condition-btn ${this.filters.condition === 'usado' ? 'active' : ''}"
              data-condition="usado"
            >
              Usado
            </button>
          </div>
        </div>

        <!-- Ordenamiento -->
        <div class="filter-group">
          <label class="filter-label">📊 Ordenar por</label>
          <select class="sort-select" id="sort-select">
            <option value="recent" ${this.filters.sortBy === 'recent' ? 'selected' : ''}>
              Más recientes
            </option>
            <option value="price_low" ${this.filters.sortBy === 'price_low' ? 'selected' : ''}>
              Precio: menor a mayor
            </option>
            <option value="price_high" ${this.filters.sortBy === 'price_high' ? 'selected' : ''}>
              Precio: mayor a menor
            </option>
            <option value="distance" ${this.filters.sortBy === 'distance' ? 'selected' : ''}>
              Más cercanos
            </option>
          </select>
        </div>
      </div>
    `;
  },

  // Template de filtros activos (chips)
  activeFiltersTemplate() {
    const activeFilters = this.getActiveFilters();
    
    if (!activeFilters.length) return '';
    
    return `
      <div class="active-filters">
        ${activeFilters.map(filter => `
          <span class="filter-chip" data-filter="${filter.key}">
            ${filter.label}
            <button class="remove-filter" data-filter="${filter.key}">✕</button>
          </span>
        `).join('')}
      </div>
    `;
  },

  // Renderizar componente
  render(container, options = {}) {
    this.onSearch = options.onSearch || null;
    this.onFilterChange = options.onFilterChange || null;
    
    container.innerHTML = this.template();
    this.attachEvents(container);
  },

  // Event listeners
  attachEvents(container) {
    const searchInput = container.querySelector('.search-input');
    const clearBtn = container.querySelector('.clear-search');
    const filterToggle = container.querySelector('.filter-toggle');
    
    // Búsqueda en tiempo real con debounce
    let searchTimeout;
    searchInput?.addEventListener('input', (event) => {
      clearTimeout(searchTimeout);
      this.searchTerm = event.target.value;
      
      // Mostrar/ocultar botón limpiar
      clearBtn?.classList.toggle('visible', !!this.searchTerm);
      
      // Búsqueda con delay para no spamear
      searchTimeout = setTimeout(() => {
        this.performSearch();
      }, 300);
    });

    // Limpiar búsqueda
    clearBtn?.addEventListener('click', () => {
      this.searchTerm = '';
      searchInput.value = '';
      clearBtn.classList.remove('visible');
      this.performSearch();
    });

    // Toggle filtros
    filterToggle?.addEventListener('click', () => {
      this.toggleFilters();
    });

    // Event listeners de filtros
    this.attachFilterEvents(container);
  },

  // Event listeners específicos de filtros
  attachFilterEvents(container) {
    const filtersPanel = container.querySelector('.filters-panel');
    
    // Slider de distancia
    const distanceSlider = container.querySelector('#distance-slider');
    distanceSlider?.addEventListener('input', (event) => {
      this.filters.maxDistance = parseInt(event.target.value);
      this.updateSliderDisplay();
      this.performFilterChange();
    });

    // Inputs de precio
    ['min-price', 'max-price'].forEach(id => {
      const input = container.querySelector(`#${id}`);
      input?.addEventListener('change', (event) => {
        const key = id === 'min-price' ? 'minPrice' : 'maxPrice';
        this.filters[key] = parseFloat(event.target.value) || 0;
        this.performFilterChange();
      });
    });

    // Botones de condición
    filtersPanel?.addEventListener('click', (event) => {
      if (event.target.matches('.condition-btn')) {
        this.selectCondition(event.target.dataset.condition);
      }
      
      if (event.target.matches('.btn-reset-filters')) {
        this.resetFilters();
      }
    });

    // Select de ordenamiento
    const sortSelect = container.querySelector('#sort-select');
    sortSelect?.addEventListener('change', (event) => {
      this.filters.sortBy = event.target.value;
      this.performFilterChange();
    });
  },

  // Toggle panel de filtros
  toggleFilters() {
    this.showFilters = !this.showFilters;
    const panel = document.querySelector('.filters-panel');
    panel?.classList.toggle('show', this.showFilters);
  },

  // Seleccionar condición
  selectCondition(condition) {
    // Actualizar estado
    this.filters.condition = condition;
    
    // Actualizar UI
    document.querySelectorAll('.condition-btn').forEach(btn => {
      btn.classList.toggle('active', btn.dataset.condition === condition);
    });
    
    this.performFilterChange();
  },

  // Resetear filtros
  resetFilters() {
    this.filters = {
      maxDistance: 50,
      minPrice: 0,
      maxPrice: 10000,
      condition: 'all',
      sortBy: 'recent'
    };
    
    // Re-renderizar para actualizar UI
    const container = document.querySelector('.search-bar-container').parentElement;
    this.render(container, { onSearch: this.onSearch, onFilterChange: this.onFilterChange });
    
    this.performFilterChange();
  },

  // Actualizar display del slider
  updateSliderDisplay() {
    const display = document.querySelector('.slider-value');
    if (display) {
      display.textContent = `Hasta ${this.filters.maxDistance}km`;
    }
  },

  // Ejecutar búsqueda
  performSearch() {
    if (this.onSearch) {
      this.onSearch(this.searchTerm, this.filters);
    }
  },

  // Ejecutar cambio de filtros
  performFilterChange() {
    // Actualizar indicador de filtros activos
    const filterToggle = document.querySelector('.filter-toggle');
    filterToggle?.classList.toggle('active', this.hasActiveFilters());
    
    // Actualizar chips de filtros activos
    this.updateActiveFiltersDisplay();
    
    if (this.onFilterChange) {
      this.onFilterChange(this.filters);
    }
    
    // También ejecutar búsqueda con nuevos filtros
    this.performSearch();
  },

  // Verificar si hay filtros activos
  hasActiveFilters() {
    return (
      this.filters.maxDistance !== 50 ||
      this.filters.minPrice !== 0 ||
      this.filters.maxPrice !== 10000 ||
      this.filters.condition !== 'all' ||
      this.filters.sortBy !== 'recent'
    );
  },

  // Obtener filtros activos para mostrar como chips
  getActiveFilters() {
    const active = [];
    
    if (this.filters.maxDistance !== 50) {
      active.push({ key: 'distance', label: `📍 ${this.filters.maxDistance}km` });
    }
    
    if (this.filters.minPrice !== 0 || this.filters.maxPrice !== 10000) {
      const min = this.filters.minPrice || 0;
      const max = this.filters.maxPrice === 10000 ? '∞' : this.filters.maxPrice;
      active.push({ key: 'price', label: `💰 Bs ${min} - ${max}` });
    }
    
    if (this.filters.condition !== 'all') {
      active.push({ key: 'condition', label: `🏷️ ${this.filters.condition}` });
    }
    
    if (this.filters.sortBy !== 'recent') {
      const sortLabels = {
        price_low: 'Precio ↑',
        price_high: 'Precio ↓',
        distance: 'Distancia'
      };
      active.push({ key: 'sort', label: `📊 ${sortLabels[this.filters.sortBy]}` });
    }
    
    return active;
  },

  // Actualizar display de filtros activos
  updateActiveFiltersDisplay() {
    const container = document.querySelector('.search-bar-container');
    const existing = container?.querySelector('.active-filters');
    
    if (existing) {
      const newTemplate = this.activeFiltersTemplate();
      if (newTemplate) {
        existing.outerHTML = newTemplate;
      } else {
        existing.remove();
      }
    } else if (this.hasActiveFilters()) {
      container?.insertAdjacentHTML('beforeend', this.activeFiltersTemplate());
    }
  }
};