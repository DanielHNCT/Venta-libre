import { router } from '../../utils/router.js';

// ProductCard - Componente reutilizable para mostrar productos
export const ProductCard = {
  
  // Template del card con datos del producto
  template(product) {
    const {
      id,
      title,
      price,
      condition = 'Usado',
      location = 'La Paz',
      distance = '0km',
      timeAgo = 'Hace 1h',
      image = '/placeholder-product.jpg',
      seller = 'Usuario'
    } = product;

    return `
      <div class="product-card" data-product-id="${id}">
        <div class="product-image">
          <img src="${image}" alt="${title}" loading="lazy">
          <div class="product-condition">
            <span class="condition-badge condition-${condition.toLowerCase()}">
              ${condition}
            </span>
          </div>
        </div>
        
        <div class="product-info">
          <h3 class="product-title">${title}</h3>
          <div class="product-price">Bs ${price.toLocaleString()}</div>
          
          <div class="product-meta">
            <div class="product-location">
              <span class="location-icon">📍</span>
              <span class="distance">${distance}</span>
              <span class="separator">•</span>
              <span class="time">${timeAgo}</span>
            </div>
            <div class="product-seller">
              <span class="seller-icon">👤</span>
              <span class="seller-name">${seller}</span>
            </div>
          </div>
        </div>
        
        <div class="product-actions">
          <button class="btn-favorite" data-action="favorite" data-id="${id}">
            <span class="heart-icon">🤍</span>
          </button>
          <button class="btn-contact" data-action="contact" data-id="${id}">
            <span class="contact-icon">💬</span>
            Contactar
          </button>
        </div>
      </div>
    `;
  },

  // Renderizar múltiples productos
  renderList(products, container) {
    if (!products?.length) {
      container.innerHTML = this.emptyState();
      return;
    }

    container.innerHTML = products
      .map(product => this.template(product))
      .join('');
    
    this.attachEvents(container);
  },

  // Estado vacío
  emptyState() {
    return `
      <div class="empty-products">
        <div class="empty-icon">📦</div>
        <h3>No hay productos disponibles</h3>
        <p>Prueba cambiar los filtros o buscar en otra categoría</p>
        <button class="btn btn-primary" onclick="router.navigateTo('/add-product')">
          ➕ Publicar primer producto
        </button>
      </div>
    `;
  },

  // Event listeners para los cards
  attachEvents(container) {
    container?.addEventListener('click', (event) => {
      const card = event.target.closest('.product-card');
      const action = event.target.closest('[data-action]');
      
      if (!card) return;
      
      const productId = card.dataset.productId;
      
      if (action) {
        event.stopPropagation();
        this.handleAction(action.dataset.action, productId);
      } else {
        // Click en el card -> ir a detalle
        router.navigateTo(`/product/${productId}`);
      }
    });
  },

  // Manejar acciones del card
  handleAction(action, productId) {
    switch (action) {
      case 'favorite':
        this.toggleFavorite(productId);
        break;
      case 'contact':
        this.contactSeller(productId);
        break;
    }
  },

  // Toggle favorito
  toggleFavorite(productId) {
    const heartIcon = document.querySelector(`[data-id="${productId}"] .heart-icon`);
    if (heartIcon) {
      const isFavorite = heartIcon.textContent === '❤️';
      heartIcon.textContent = isFavorite ? '🤍' : '❤️';
      
      // Aquí agregar lógica para guardar favorito en API
      this.showToast(isFavorite ? 'Eliminado de favoritos' : 'Agregado a favoritos');
    }
  },

  // Contactar vendedor
  contactSeller(productId) {
    // Navegar al chat o mostrar info de contacto
    router.navigateTo(`/chat/${productId}`);
  },

  // Mostrar notificación temporal
  showToast(message) {
    const toast = document.createElement('div');
    toast.className = 'toast';
    toast.textContent = message;
    
    document.body.appendChild(toast);
    
    setTimeout(() => {
      toast.classList.add('show');
    }, 100);
    
    setTimeout(() => {
      toast.remove();
    }, 3000);
  }
};