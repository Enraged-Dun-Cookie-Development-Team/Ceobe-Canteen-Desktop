import { createRouter, createWebHistory } from 'vue-router';

const routes = [
  {
    path: '',
    component: () => import('@/components/Navbar'),
    redirect: '/home/tool',
    children: [
      {
        path: '/home',
        component: () => import('@/view/home/index'),
        name: 'Home',
        children: [
          {
            path: 'tool',
            component: () => import('@/view/home/infoAndTool'),
            name: 'Tool'
          },
          {
            path: 'browser',
            component: () => import('@/components/Browser'),
            name: 'Browser'
          }
        ]
      }
    ]
  },
  {
    path: '/notification',
    component: () => import('@/view/notification')
  },
  {
    path: '/background',
    component: () => import('@/view/background')
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
