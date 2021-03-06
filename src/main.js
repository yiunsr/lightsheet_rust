import 'font-awesome/css/font-awesome.min.css'
import Vue from 'vue'
import VueRouter from 'vue-router'

import App from './App.vue'
import router from './router'
import vuetify from './plugins/vuetify';
import i18n from './i18n'
import common from './common.js'
import ui from './ui.js'
import util_grid from './js/util_grid.js'
import _ from 'lodash'

window.common = common;
window.temp = {};
window.ui = ui;
window.util_grid = util_grid;
window._ = _;
let window_id;
if(localStorage.window_id)
  window_id = parseInt(localStorage.window_id);
else{
  window_id = 1;
  localStorage.window_id = 1;
}
window.window_id = window_id;

Vue.config.productionTip = false

Vue.use(VueRouter);
var vm = new Vue({
  vuetify,
  i18n,
  router,
  render: h => h(App)
}).$mount('#app')

window.vm = vm;
window.vm_app = vm.$root.$children[0];

common.initApp();
window.apiCallback = function(success, cb, result){
  common.apiCallback(success, cb, result);
}

