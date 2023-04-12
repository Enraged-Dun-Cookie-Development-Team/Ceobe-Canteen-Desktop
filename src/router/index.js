import {createRouter, createWebHistory} from 'vue-router'
import Navbar from '@/view/navbar'

const routes = [
    {
        path: '',
        component: Navbar,
        redirect: '/home',
        children: [
            {
                path: '/home',
                component: () => import("@/view/home"),
                name: 'Home'
            }
        ]
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default {
    router
}