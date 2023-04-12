import {createRouter, createWebHistory} from 'vue-router'
import Navbar from '@/view/navbar'

const routes = [
    {
        path: '',
        component: Navbar,
        redirect: '/timeline',
        children: [
            {
                path: '/timeline',
                component: () => import("@/view/timeline"),
                name: 'TimeLine'
            }
        ]
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router