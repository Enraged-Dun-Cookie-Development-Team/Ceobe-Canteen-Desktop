import { createApp } from 'vue'
import App from './App.vue'
import Vuetify from 'vuetify'
import 'vuetify/dist/vuetify.min.css'

const app = createApp(App);
app.use(Vuetify);
app.mount('#app')