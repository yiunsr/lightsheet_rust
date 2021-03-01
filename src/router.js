import VueRouter from 'vue-router'
import StartPage from './views/StartPage';
import Sheet from './views/Sheet';
import SheetTest from './views/SheetTest';

const routes = [
  {
    path: '/',
    name: 'startpage',
    component: StartPage
  },
  {
    path: '/sheet',
    name: 'sheet',
    component: Sheet
  },
  {
    path: '/sheettest',
    name: 'sheettest',
    component: SheetTest
  },
  { path: '*', redirect: '/' }
]

const router = new VueRouter({
  mode: 'history',
  routes
})

export default router