import {createRouter, createWebHistory} from 'vue-router'
import Navbar from '@/components/Navbar'

const routes = [
    {
        path: '',
        component: Navbar,
        redirect:'/tool',
        children: [
            {
                path: '/home',
                component: () => import("@/view/home/index"),
                name: 'Home',
                children: [
                    {
                        path: '/tool',
                        component: () => import("@/view/home/infoAndTool"),
                        name: 'Tool'
                    }
                ]
            }
        ]
    },
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router