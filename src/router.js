import VueRouter from 'vue-router'
import StartPage from './views/StartPage';
import Sheet from './views/Sheet';

const routes = [
  {
    path: '/',
    name: 'startpage',
    component: StartPage
  },
  {
    path: '/',
    name: 'sheet',
    component: Sheet
  },
  { path: '*', redirect: '/' }
]

const router = new VueRouter({
  mode: 'history',
  routes
})

export default router