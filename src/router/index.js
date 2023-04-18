import {createRouter, createWebHistory} from 'vue-router'

const routes = [
    {
        path: '',
        component: () => import("@/components/Navbar"),
        redirect:'/home/tool',
        children: [
            {
                path: '/home',
                component: () => import("@/view/home/index"),
                name: 'Home',
                children: [
                    {
                        path: 'tool',
                        component: () => import("@/view/home/infoAndTool"),
                        name: 'Tool'
                    },
                    {
                        path: 'browser',
                        component: () => import("@/components/browser"),
                        name: 'Browser'
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