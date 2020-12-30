import 'font-awesome/css/font-awesome.min.css'
import Vue from 'vue'
import App from './App.vue'
import vuetify from './plugins/vuetify';
import ko from './locale.ko';
import en from './locale.en';
Vue.config.productionTip = false

var vm = new Vue({
  vuetify,
  render: h => h(App),
  theme:{
    themes: {
        light: {
            primary: '#3f51b5',
            secondary: '#b0bec5',
            accent: '#8c9eff',
            error: '#b71c1c',
          },
        dark: {
          //here you will define primary secondary stuff for dark theme
        }
    },
    dark: true
  },
  icons: {
    iconfont: 'fa4',
  },
  lang: {
    locales: { en, ko },
    current: 'en',
  },
}).$mount('#app')

if(typeof window == "object"){
  window.vm = vm;
}