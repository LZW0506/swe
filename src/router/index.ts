import {
    createRouter,
    createWebHashHistory,
} from 'vue-router'
import Layout from '../layout/index.vue'
// 创建路由
const router = createRouter({
    // 创建路由模式 history模式--createWebHashHistory 哈希模式--createWebHistory
    history: createWebHashHistory(),
    // 配置路由规则
    routes: [
        {
            path: '/', component:Layout,
            redirect:'index',
            children:[
                {
                    path:'index',
                    component:()=>import("../views/index.vue")
                }
            ]
        },

    ],
})

// 导出路由
export default router
