import {createRouter, createWebHistory} from 'vue-router'
import Navbar from '@/components/Navbar'

const routes = [
    {
        path: '',
        component: Navbar,
        redirect: '/home',
        children: [
            {
                path: '/home',
                component: () => import("@/view/home/index"),
                name: 'Home'
            }
        ]
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router