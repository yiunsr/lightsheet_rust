import Vue from 'vue';
import Vuetify from 'vuetify/lib/framework';
import colors from 'vuetify/lib/util/colors'
import ko from '../lang/locale.ko';
import en from '../lang/locale.en';

Vue.use(Vuetify);

export default new Vuetify({
  icons: {
    iconfont: 'fa4',
  },
  lang: {
    locales: { en, ko },
      current: 'en',
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
          background: colors.indigo.base,
        }
      },
    dark: true
  },
});
