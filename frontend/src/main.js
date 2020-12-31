import 'font-awesome/css/font-awesome.min.css'
import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify';
Vue.config.productionTip = false

var vm = new Vue({
  vuetify,
  render: h => h(App),
}).$mount('#app')

if(typeof window == "object"){
  window.vm = vm;
}