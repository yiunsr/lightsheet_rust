<template>
  <div>
    <v-text-field v-model='file_path' @click="fileDialog"	 :label="label" :dense="dense" readonly>
      <template v-slot:prepend>
        <v-tooltip bottom>
          <template v-slot:activator="{ on }">
            <v-btn dark small color="primary" @click="fileDialog">
              <v-icon v-on="on">
                mdi-folder-open-outline
              </v-icon>
            </v-btn>
          </template>
          {{ $t("dialog_import.filepath_tooltip")}}
        </v-tooltip>
      </template>
    </v-text-field>
  </div>
</template>


<script>
import ui from '../ui.js'
import common from '../common.js'
export default {
  model: {
    prop: 'fileInfo',
    event: 'change'
  },
  props: ['label', 'change', 'dense'],
  data() {
    return {
      fileInfo: {},
      file_path: ''
    }
  },
  methods: {
    fileDialog() {
      ui.fileImportNativeDialog().then(path => {
        this.file_path = path; 
        let this_ = this;
        if(!path)return;
        common.fileGetInfo(path).then(function(res){
          let arraybuffer = Uint8Array.from(atob(res.buffer), c => c.charCodeAt(0));
          // let td = new TextDecoder(res.enc);
          // let buffer = td.decode(arraybuffer);
          // debugger;  // eslint-disable-line no-debugger
          this_.fileInfo = {result: res.result, buffer: arraybuffer, sep: res.sep, enc: res.enc};
          this_.$emit('change', this_.fileInfo);
      });
    });
    }
  },
  computed: {
  },
}
</script>