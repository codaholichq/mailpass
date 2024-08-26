import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'Home',
      alias: "/home",
      meta: { requiresAuth: false },
      component: () => import('@/views/Home.vue')
    },

    {
      path: '/login',
      name: 'Login',
      meta: { requiresAuth: false },
      component: () => import('@/views/auth/Login.vue')
    },

    {
      path: '/dashboard/users/add',
      name: 'AddUser',
      meta: { requiresAuth: true },
      component: () => import('@/views/users/AddUser.vue')
    },

    {
      path: '/dashboard/users',
      name: 'GetUsers',
      meta: { requiresAuth: true },
      component: () => import('@/views/users/GetUsers.vue')
    },

    {
      path: '/:pathMatch(.*)*',
      name: 'NotFound',
      meta: { requiresAuth: false },
      component: () => import('@/views/NotFound.vue')
    }
  ]
})

router.beforeEach((to, from, next) => {
  if (to.meta.requiresAuth) {
    const token = localStorage.getItem('token');
    if (token) {
      // User is authenticated, proceed to the route
      next();
    } else {
      // User is not authenticated, redirect to login
      next('/login');
    }
  } else {
    // Non-protected route, allow access
    next();
  }
});

export default router
