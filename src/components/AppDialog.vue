<template>
  <div>
    <v-row justify="center">
      <v-dialog persistent :value="getDialogType=='import'">
        <v-card>
          <v-card-title>
            <span class="text-h5">{{ $t('dialog_import.header') }}</span>
            <v-spacer></v-spacer>
              <v-btn icon @click="close" >
              <v-icon>mdi-close</v-icon>
            </v-btn>
          </v-card-title>
          <v-card-text>
            <v-container>
              <v-row dense class="mt-2  pa-0">
                <v-col cols="12">
                  <FileInput :key="componentKey" label='csv file' v-model="fileInfo" dense />
                </v-col>
              </v-row>
              <v-row dense class="mt-n2 pa-0">
                <v-col cols="12" md="4">
                  <v-select v-model="col_sep"
                    :label="$t('dialog_import.col_sep_label')" :items="colItems"></v-select>
                </v-col>
                <v-col cols="12" md="4">
                  <v-select  v-model="enc"
                    :label="$t('dialog_import.chacter_encoding')" :items="characterEncodings"></v-select>
                </v-col>
              </v-row>
              <v-row dense class="mt-n2 pa-0">
                <v-col cols="12">
                  <v-subheader>{{ $t('dialog_import.decoded_text') }}</v-subheader>
                  <v-card elevation="1" height="200" width="100%" class="decode_buffer" v-html="decodeBufferHTML">
                  </v-card>
                  
                </v-col>
              </v-row>
            </v-container>
          </v-card-text>
          <v-card-text>
            <v-container>
            </v-container>
          </v-card-text>
          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn
              color="blue darken-1"
              text
              @click="close"
            >
              {{ $t('dialog_import.close_btn') }}
            </v-btn>
            <v-btn
              color="blue darken-1"
              text
              @click="import_file"
            >
              {{ $t('dialog_import.import_btn') }}
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
    </v-row>
  </div>
</template>

<style scoped>
.decode_buffer{
  overflow: scroll;
  white-space: nowrap;
}
</style>

<style>
.decode_buffer span.warning{
  font-weight: bold;
}
</style>

<script>
import FileInput from '../widget/FileInput';
export default {
  components: {
    FileInput,
  },
  data () {
    return {
      fileInfo: {},
      enc: "",
      buffer: "",
      decode_buffer: "",
      col_sep: "",
      colItems: [
        {text: 'tab', value: "tab"},
        {text: '|', value: "|"},
        {text: ',', value: ","},
      ],
      characterEncodings: [
        {text: 'UTF-8', value: "UTF-8"},
        {text: 'CP949(EUC-KR)', value: "EUC-KR"},
      ],
      // https://stackoverflow.com/a/47466574/6652082
      componentKey: 0,
    }
  },
  methods: {
    close () {
      this.fileInfo = {};
      this.enc = "";
      this.buffer = "",
      this.decode_buffer = "";
      this.$store.commit('setDialogType', "");
      this.componentKey = !this.componentKey;
    },
    import_file(){
      this.clear();
      //this.$store.commit('setDialogType', "");
    }
  },
  computed: {
    getDialogType() {
      return this.$store.getters.getDialogType;
    },
    decodeBufferHTML() {
      if(this.decode_buffer == "")return "";
      // debugger; // eslint-disable-line no-debugger
      let newHTML = "";
      let decode_buffer = this.decode_buffer;
      decode_buffer = decode_buffer.replaceAll("\n\r", "\n");
      let lines = decode_buffer.split("\n");
      for (let line of lines) {
        line = line.replaceAll(this.col_sep, "<span class='warning'>" + this.col_sep + "</span>");
        newHTML += line + "<br/>";
      }
      return newHTML;
    },
  },
  watch: {
    fileInfo: {
      deep: true,
      handler(newValue){
        this.buffer = newValue.buffer;
        this.enc = newValue.enc;
        this.col_sep = newValue.sep;
      }
    },
    enc: {
      handler(){
        // debugger; // eslint-disable-line no-debugger
        if(!this.buffer){
          this.decode_buffer = "";
          return;
        }
        let td = new TextDecoder(this.enc);
        this.decode_buffer = td.decode(this.buffer);
      }
    }
  }
}
</script>