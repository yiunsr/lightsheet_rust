import 'font-awesome/css/font-awesome.min.css'
import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify';
import i18n from './i18n'
import common from './common.js'
Vue.config.productionTip = false

var vm = new Vue({
  vuetify,
  i18n,
  render: h => h(App)
}).$mount('#app')

if(typeof window == "object"){
  window.vm = vm;
}

common.initApp();
window.apiCallback = function(success, cb, result){
  common.apiCallback(success, cb, result);
}
