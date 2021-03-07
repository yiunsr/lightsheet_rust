<template>
  <div class="main">
    <v-style v-if="this.$vuetify.theme.dark === false">
      body {
        background-color: rgb(248, 249, 250);
        box-shadow: none;
      }
      ::selection {
        background-color: rgb(186, 212, 253);
      }
      :root {
        --demo-font-color: #222;
        --demo-bars-bkg: rgb(255, 255, 255);
        --demo-bars-shadow: 0 1px 3px 1px rgba(60, 64, 67, 0.15);
        --demo-bars-padding: 0px;
        --demo-bars-border-radius: 1px;
        --demo-text-bkg-color: white;
        --demo-text-box-shadow: 0 1px 3px 1px rgba(60, 64, 67, 0.15);

        --bar-font-color: rgb(32, 33, 36);
        --bar-font-family: Roboto, RobotoDraft, Helvetica, Arial, sans-serif;
        --bar-font-size: 15px;
        --bar-font-weight: 500;
        --bar-letter-spacing: 0.2px;
        --bar-padding: 3px;
        --bar-button-icon-size: 20px;
        --bar-button-padding: 4px 4px;
        --bar-button-radius: 4px;
        --bar-button-hover-bkg: rgb(241, 243, 244);
        --bar-button-active-color: rgb(26, 115, 232);
        --bar-button-active-bkg: rgb(232, 240, 254);
        --bar-button-open-color: rgb(32, 33, 36);
        --bar-button-open-bkg: rgb(232, 240, 254);
        --bar-menu-bkg: white;
        --bar-menu-border-radius: 0 0 3px 3px;
        --bar-menu-item-chevron-margin: 0;
        --bar-menu-item-hover-bkg: rgb(241, 243, 244);
        --bar-menu-item-padding: 5px 8px 5px 35px;
        --bar-menu-item-icon-size: 15px;
        --bar-menu-item-icon-margin: 0 9px 0 -25px;
        --bar-menu-padding: 6px 1px;
        --bar-menu-shadow: 0 2px 6px 2px rgba(60, 64, 67, 0.15);
        --bar-menu-separator-height: 1px;
        --bar-menu-separator-margin: 5px 0 5px 34px;
        --bar-menu-separator-color: rgb(227, 229, 233);
        --bar-separator-color: rgb(218, 220, 224);
        --bar-separator-width: 1px;
        --bar-sub-menu-border-radius: 3px;
      }
    </v-style>
    <v-style v-else>
      body {
        background-color: rgb(248, 249, 250);
        box-shadow: none;
      }
      ::selection {
        background-color: rgb(186, 212, 253);
      }
      :root {
        --demo-font-color: #222;
        --demo-bars-bkg: rgb(255, 255, 255);
        --demo-bars-shadow: 0 1px 3px 1px rgba(60, 64, 67, 0.15);
        --demo-bars-padding: 0px;
        --demo-bars-border-radius: 1px;
        --demo-text-bkg-color: white;
        --demo-text-box-shadow: 0 1px 3px 1px rgba(60, 64, 67, 0.15);

        --bar-font-color: rgb(255, 255, 255);
        --bar-font-family: Roboto, RobotoDraft, Helvetica, Arial, sans-serif;
        --bar-font-size: 15px;
        --bar-font-weight: 500;
        --bar-letter-spacing: 0.2px;
        --bar-padding: 3px;
        --bar-button-icon-size: 20px;
        --bar-button-padding: 4px 4px;
        --bar-button-radius: 4px;
        --bar-button-hover-bkg: rgb(241, 243, 244);
        --bar-button-active-color: rgb(26, 115, 232);
        --bar-button-active-bkg: rgb(232, 240, 254);
        --bar-button-open-color: rgb(32, 33, 36);
        --bar-button-open-bkg: rgb(232, 240, 254);
        --bar-menu-bkg: white;
        --bar-menu-border-radius: 0 0 3px 3px;
        --bar-menu-item-chevron-margin: 0;
        --bar-menu-item-hover-bkg: rgb(241, 243, 244);
        --bar-menu-item-padding: 5px 8px 5px 35px;
        --bar-menu-item-icon-size: 15px;
        --bar-menu-item-icon-margin: 0 9px 0 -25px;
        --bar-menu-padding: 6px 1px;
        --bar-menu-shadow: 0 2px 6px 2px rgba(60, 64, 67, 0.15);
        --bar-menu-separator-height: 1px;
        --bar-menu-separator-margin: 5px 0 5px 34px;
        --bar-menu-separator-color: rgb(227, 229, 233);
        --bar-separator-color: rgb(218, 220, 224);
        --bar-separator-width: 1px;
        --bar-sub-menu-border-radius: 3px;
      }
    </v-style>

    <div class="bars" style="height:36px;">
      <vue-file-toolbar-menu v-for="(content, index) in bars_content" 
        :key="'bar-'+index" :content="content"   />
    </div>
  </div>
</template>

<script>
import Vue from 'vue'
import VueFileToolbarMenu from 'vue-file-toolbar-menu';
import common from '../common.js'
import ui from '../ui.js'
import router from '../router'
// for dynamic css variables
Vue.component("v-style", {
  render (createElement) { return createElement("style", this.$slots.default); }
});
export default {
  components: { VueFileToolbarMenu },
  data () {
    return {
      color: "rgb(74, 238, 164)",
      font: "Avenir",
      theme: "default",
      edit_mode: true,
      check1: false,
      check2: false,
      check3: true
    }
  },
  computed: {
    test(){
      Vue.prototype
      return 0;
    },
    // Read the API documentation about the available menu content options
    bars_content () {
      return [
        [
          {
            text: this.$t("menu.file"),
            menu: [
              { 
                text: this.$t("menu.file_open") + "...", 
                hotkey: this.isMacLike ? "command+o" : "ctrl+o",
                click: () => { 
                  ui.fileOpen();
                },
                id: "menu__file_open",
                title: this.$t("menu.file_open__sb"),
              },
              { 
                text: this.$t("menu.file_save") + "...", 
                click: () => alert("You're amazing, "+(prompt("What's your name?")||"friend")+"!") ,
                disabled: true,
              },
              { is: "separator" },
              { text: this.$t("menu.file_exit"), click () { common.callAPI("exit") } },
            ]
          },
          {
            text: this.$t("menu.edit"),
            menu: [
              { text: this.$t("menu.edit_cut"), click: () => document.execCommand("cut") },
              { text: this.$t("menu.edit_copy"), click: () => document.execCommand("copy") },
              { text: this.$t("menu.edit_paste"), click () { navigator.clipboard.readText().then(text => { document.execCommand("insertText", false, text) }) } }
            ]
          },
          {
            text: "Formats",
            menu: [
              { text: "Basic" },
              { text: "Disabled", disabled: true },
              {
                text: "Sub-menus",
                custom_chevron: this.theme != "default" ? "►" : false,
                menu: [
                  { text: "Hello!" },
                  {
                    text: "I'm a sub-menu",
                    custom_chevron: this.theme != "default" ? "►" : false,
                    menu: [
                      { text: "And I'm another sub-menu!" },
                    ],
                    menu_width: 240
                  }
                ],
                menu_width: 200
              },
              {
                text: "Hotkey",
                hotkey: this.isMacLike ? "command+e" : "ctrl+e",
                click () {
                  alert("Hotkey menu triggered either via clicking or shortcut.");
                }
              },
              { text: "Material icon", icon: "shopping_cart", click: () => window.open("https://material.io/resources/icons", "_blank") },
              { text: "Platform emoji", emoji: "call_me_hand", click: () => window.open("https://raw.githubusercontent.com/omnidan/node-emoji/master/lib/emoji.json", "_blank") },
              { text: "Menu text is wrapped when it is too long" },
              { is: "separator" },
              {
                text: "Option 1",
                icon: this.check1 ? "radio_button_unchecked" : "radio_button_checked",
                click: (e) => {
                  e.stopPropagation(); // prevent menu close when clicking
                  this.check1 = false;
                }
              },
              {
                text: "Option 2",
                icon: this.check1 ? "radio_button_checked" : "radio_button_unchecked",
                click: (e) => {
                  e.stopPropagation(); // prevent menu close when clicking
                  this.check1 = true;
                }
              },
              { is: "separator" },
              {
                text: "Option 3",
                icon: this.check2 ? "check_box" : "check_box_outline_blank",
                click: (e) => {
                  e.stopPropagation(); // prevent menu close when clicking
                  this.check2 = !this.check2;
                }
              },
              {
                text: "Option 4",
                icon: this.check3 ? "check_box" : "check_box_outline_blank",
                click: (e) => {
                  e.stopPropagation(); // prevent menu close when clicking
                  this.check3 = !this.check3;
                }
              }
            ],
            menu_width: 220
          },
          {
            text: this.$t("menu.help"),
            menu: [
              { text: this.$t("menu.help_about"), icon: "help", 
                click: () => {
                  alert("lightsheet_rust\nhttps://github.com/yiunsr/lightsheet_rust\nby Yiun Seungryong");
                }
              },
              { is: "separator" },
              { text: "Repository", icon: "exit_to_app",
               click: () => {
                 common.openURL("https://github.com/yiunsr/lightsheet_rust")
               }
              },
            ],
            menu_width: 220
          },
          {
            text: "Debug",
            menu: [
              { text: "Title test", click:() =>  common.setTitle("setTitle (한국어)") },
              { text: "Alert test", click:() =>  alert("Alert Test (한국어)") },
              { text: "confirm test", click:() =>  {
                window.__menu__confirm_test = function(result){
                  alert(""+result);
                };
                common.confirm("Confirm Test(한국어)", "__menu__confirm_test", "입력");
              }},
              { text: "File Open Dialog", click:() =>  {
                window.__menu__fileopendialog_test = function(result){
                  alert(""+result);
                };
                common.fileOpenDialog("__menu__fileopendialog_test");
              }},
              { text: "Prompt test", click:() =>   {
                window.__menu__prompt_test = function(result){
                  alert(""+result);
                };
                common.prompt("Prompt test", "__menu__prompt_test");
              }},
              { text: "Change Status", click:() =>   {
                common.setStatusbar("statusbar 설정");
              }},
              { is: "separator" },
              { text: "home", click:() =>  {
                router.push({ path: '/' })
                }
              },
              { text: "sheettest", click:() =>  {
                router.push({ path: '/sheettest' })
                }
              },
              { text: "history back", click:() =>  {
                router.go(-1);
                }
              },

            ]
          },
          { is: "spacer" },
        ],
      ]
    },
    isMacLike: () => /(Mac|iPhone|iPod|iPad)/i.test(navigator.platform),
  },
}


</script>

<style>
:root {
  --demo-font-color: rgb(74, 238, 164);
}
</style>

<style scoped>
a {
  color: inherit;
}
select {
  outline: none;
}
.main {
  width: 100%;
  height: 100%;
}
.title {
  text-align: center;
  font-size: 50px;
  padding-top: 30px;
}
.subtitle {
  font-size: 18px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding-bottom: 50px;
}
.experiment {
  width: 95%;
  margin: auto;
  max-width: 1150px;
}
.bars {
  border-radius: var(--demo-bars-border-radius, 5px);
  padding: var(--demo-bars-padding, 8px);
  transition: .5s;
}
::v-deep.bars * {
  transition: font-size .1s linear, padding .1s linear, margin .1s linear;
}
.styles {
  position: fixed;
  top: 10px;
  right: 10px;
}
.text {
  font-family: var(--bar-font-family);
  width: 90%;
  margin: 30px auto;
  font-size: 20px;
  min-height: 250px;
  background-color: var(--demo-text-bkg-color);
  background-image: var(--demo-text-bkg-img, url("data:image/svg+xml,%3csvg width='100%25' height='100%25' xmlns='http://www.w3.org/2000/svg'%3e%3crect width='100%25' height='100%25' fill='none' stroke='%23FFFFFF30' stroke-width='2' stroke-dasharray='15' stroke-dashoffset='0' stroke-linecap='square'/%3e%3c/svg%3e"));
  border: var(--demo-text-border);
  box-shadow: var(--demo-text-box-shadow);
  padding: 10px 15px;
  transition: .5s;
}
</style>