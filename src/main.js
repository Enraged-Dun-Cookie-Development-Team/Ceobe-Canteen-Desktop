import {createApp} from 'vue'
import App from './App.vue'
import router from './router'
import vuetify from './plugins/vuetify'
import {loadFonts} from './plugins/webfontloader'
import {getImg} from './utils'
import "@/assets/css/index.scss"

loadFonts()

const app = createApp(App);

app.config.globalProperties.getImg = getImg;

app.use(router);
app.use(vuetify);
app.mount('#app');
