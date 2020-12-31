<template>
  <v-app :style="{background: $vuetify.theme.themes[themeMode].background}">
    <v-app-bar
      app
      height="36px"
      dense

    >
      <v-toolbar-title style="width:36px">LS</v-toolbar-title>

      <AppMenu height="36px" />
      <v-spacer></v-spacer>
      <v-btn
        href="#" onclick="location.reload();"
        target="_blank"
        text
      >
        <span class="mr-2">reload</span>
        <v-icon>mdi-refresh</v-icon>
      </v-btn>

      <v-menu offset-y>
        <template v-slot:activator="{ on }">
          <v-btn  dense height="24px" width="24px"  v-on="on" color="info" x-small fab @click="changeLang">
            <v-icon >fa-language</v-icon>
          </v-btn>
        </template>
      <v-list>
        <v-list-item>
          <v-list-item-title>English</v-list-item-title>
        </v-list-item>
        <v-list-item>
          <v-list-item-title>한국어</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>

      <div>
        <v-tooltip v-if="!$vuetify.theme.dark" bottom>
          <template v-slot:activator="{ on }">
            <v-btn dense height="24px" width="24px" v-on="on" color="primary" x-small fab @click="darkMode">
              <v-icon class="mr-1">mdi-moon-waxing-crescent</v-icon>
            </v-btn>
          </template>
          <span>Dark Mode On</span>
        </v-tooltip>

        <v-tooltip v-else bottom>
          <template v-slot:activator="{ on }">
            <v-btn  dense height="24px" width="24px"  v-on="on" color="primary" x-small fab @click="darkMode">
              <v-icon>mdi-white-balance-sunny</v-icon>
            </v-btn>
          </template>
          <span>Dark Mode Off</span>
        </v-tooltip>
      </div>
    </v-app-bar>

    <v-main>
      <HelloWorld/>
    </v-main>
  </v-app>
</template>

<script>
import HelloWorld from './components/HelloWorld';
import AppMenu from './components/AppMenu';


export default {
  name: 'App',

  components: {
    HelloWorld, AppMenu
  },

  data: () => ({
    //
  }),
  computed:{
    themeMode(){
      console.log(this.$vuetify.theme.dark);
      return (this.$vuetify.theme.dark) ? 'dark' : 'light' 
    }
  },  
  methods: {
    darkMode: function(){
      this.$vuetify.theme.dark = !this.$vuetify.theme.dark;
    },
    changeLang: function(){

    },
  },
};
</script>
