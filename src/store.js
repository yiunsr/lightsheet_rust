import Vue from "vue";
import Vuex from "vuex";

Vue.use(Vuex);

export let store = new Vuex.Store({
  state: {
    path: '/new', 
    dialog_type: ''
  },
  getters: {
    getPath: (state) => {
      return state.path
    },
    getDialogType: (state) => {
      return state.dialog_type
    }

  },
  mutations: {
    setPath: function (state, path) {
      state.path = path;
    },
    setDialogType: function (state, dialog_type) {
      state.dialog_type = dialog_type;
    }
  }
});
