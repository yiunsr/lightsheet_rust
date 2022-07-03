<template>
  <v-app ref="app" 
    :style="{background: $vuetify.theme.themes[themeMode].background}">
    <v-app-bar
      app
      height="36px"
      dense
      class="pa-0 ls-menu-header"
    >
      <v-toolbar-title style="width:36px;text-align:center" 
        class="font-weight-black rounded secondary ">LS</v-toolbar-title>

      <AppMenu :menu-info="menuinfo" />
      <v-spacer></v-spacer>
      <v-btn
        href="#" onclick="location.reload();"
        target="_blank"
        text
        tabindex="-1"
      >
        <span class="mr-2">reload</span>
        <v-icon>mdi-refresh</v-icon>
      </v-btn>

      <v-menu offset-y>
        <template v-slot:activator="{ on }">
          <v-btn  dense height="24px" width="24px"  v-on="on" color="info" x-small fab
            tabindex="-1"
          >
            <v-icon >fa-language</v-icon>
          </v-btn>
        </template>
        <v-list>
          <v-list-item @click="changeLang('en')" >
            <v-list-item-title>English</v-list-item-title>
          </v-list-item>
          <v-list-item @click="changeLang('ko')">
            <v-list-item-title>한국어</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>

      <div>
        <v-tooltip v-if="!$vuetify.theme.dark" bottom>
          <template v-slot:activator="{ on }">
            <v-btn dense height="24px" width="24px" v-on="on" color="primary" x-small
             fab @click="darkMode" tabindex="-1">
              <v-icon class="mr-1">mdi-moon-waxing-crescent</v-icon>
            </v-btn>
          </template>
          <span>Dark Mode On</span>
        </v-tooltip>

        <v-tooltip v-else bottom>
          <template v-slot:activator="{ on }">
            <v-btn  dense height="24px" width="24px"  v-on="on" color="primary" x-small 
              fab @click="darkMode" tabindex="-1">
              <v-icon>mdi-white-balance-sunny</v-icon>
            </v-btn>
          </template>
          <span>Dark Mode Off</span>
        </v-tooltip>
      </div>
    </v-app-bar>

    <v-main>
      <router-view></router-view>
    </v-main>

    <v-footer padless outlined a-0 height="24px" tabindex="-1">
      <v-col
        class="text-left py-0 subtitle-2"
        cols="12"
      >
        {{ statusbar }}
      </v-col>
    </v-footer>

    <v-dialog
      v-model="progress_dialog" max-width="600" persistent
      content-class="v-app-bar"
    >
      <v-card>
        <v-card-title class="headline">
          {{ progress_dialog_title }}
        </v-card-title>

        <v-card-text>
          <v-progress-linear
            color="primary"
            :buffer-value="progress_percent" stream height = "20"
          >
            <template v-slot:default>
              <strong>{{ progress_percent }}%</strong>
            </template>
          </v-progress-linear>
        </v-card-text>

        <v-card-actions>
          <v-spacer></v-spacer>
        </v-card-actions>
      </v-card>
    </v-dialog>

  </v-app>
</template>

<style scoped>
  /*  .slick-cell.editable is z-idexL:11  */
  .v-app-bar.v-app-bar--fixed.ls-menu-header{
    z-index: 15;
  }
</style>

<style>
  @font-face {
    font-family: 'Noto Sans KR';
    src: url("./assets/NotoSansKR-Regular.woff2");
    font-weight: normal;
    font-style: normal;
  }

  @font-face {
    font-family: 'Noto Sans KR';
    src: url("./assets/NotoSansKR-Bold.woff2");
    font-weight: bold;
    font-style: normal;
  }

  html, body{
    overflow-y: hidden;
    font-family: 'Noto Sans KR', sans-serif;
  }
</style>

<script>
import AppMenu from './components/AppMenu';


export default {
  name: 'App',

  components: {
    AppMenu
  },

  data: () => ({
    progress_dialog: false, progress_percent: 0, progress_dialog_title: "",
    statusbar: "",
    menuinfo: {
      file_export: {disabled: true}
    },
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
    changeLang: function(lang){
      // debugger;  // eslint-disable-line no-debugger
      this.$i18n.locale = lang;
    },
    show_progress_dialog(title){
      this.progress_dialog = true;
      this.progress_percent = 0;
      this.progress_dialog_title = title;
    },
    hide_progress_dialog(){
      this.progress_dialog = false;
      this.progress_percent = 0;
      this.progress_dialog_title = "";
    },
    progress_dialog_percent(percent){
      this.progress_percent = percent;
    },
  },
};
</script>
