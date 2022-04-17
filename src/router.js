import VueRouter from 'vue-router'
import StartPage from './views/StartPage';
import Sheet from './views/Sheet';
import SheetTest from './views/SheetTest';
import SheetTest2 from './views/SheetTest2';
import SheetTest3 from './views/SheetTest3';

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
  {
    path: '/sheettest2',
    name: 'sheettest2',
    component: SheetTest2
  },
  {
    path: '/sheettest3',
    name: 'sheettest3',
    component: SheetTest3
  },
  { path: '*', redirect: '/' }
]

const router = new VueRouter({
  mode: 'history',
  routes
})

export default router