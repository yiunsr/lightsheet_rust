import Vue from 'vue';
import Vuetify from 'vuetify/lib/framework';
import colors from 'vuetify/lib/util/colors';
import '@mdi/font/css/materialdesignicons.css';

Vue.use(Vuetify);

export default new Vuetify({
  icons: {
    iconfont: ['fa4', 'mdi'],
  },
  theme:{
    themes: {
      light: {
        primary: '#3f51b5',
        secondary: '#b0bec5',
        accent: '#8c9eff',
        error: '#b71c1c',
        background: '#FFFFFF',
      },
      dark: {
        primary: colors.blue.lighten3,
        background: colors.grey.darken3,
      }
    },
    dark: false
  },
});
