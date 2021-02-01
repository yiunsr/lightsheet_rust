import 'font-awesome/css/font-awesome.min.css'
import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify';
import i18n from './i18n'
import common from './common.js'
import ui from './ui.js'
Vue.config.productionTip = false

var vm = new Vue({
  vuetify,
  i18n,
  render: h => h(App)
}).$mount('#app')

window.vm = vm;
window.vm_app = vm.$root.$children[0];

common.initApp();
window.apiCallback = function(success, cb, result){
  common.apiCallback(success, cb, result);
}
window.common = common;
window.ui = ui;
