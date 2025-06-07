// Router SPA con objetos literales y sintaxis moderna
export const router = {
  routes: new Map(),
  currentPage: null,

  // Inicializar router
  init() {
    // Event listener con arrow function moderna
    window.addEventListener('popstate', () => this.loadCurrentPage());
    this.loadCurrentPage();
  },

  // Registrar ruta con Map (más eficiente que objeto)
  addRoute(path, pageComponent) {
    this.routes.set(path, pageComponent);
  },

  // Navegación con History API
  navigateTo(path) {
    window.history.pushState({}, '', path);
    this.loadCurrentPage();
  },

  // Cargar página actual
  loadCurrentPage() {
    const path = window.location.pathname;
    const page = this.routes.get(path) ?? this.routes.get('/');
    
    if (page && page !== this.currentPage) {
      this.currentPage = page;
      this.renderPage(page);
    }
  },

  // Renderizar con optional chaining
  renderPage(pageComponent) {
    const appContainer = document.getElementById('app');
    pageComponent?.render?.(appContainer);
  },

  // Helper para links SPA (previene refresh)
  handleLink(event, path) {
    event.preventDefault();
    this.navigateTo(path);
  }
};