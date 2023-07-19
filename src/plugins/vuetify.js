// Styles
import '@mdi/font/css/materialdesignicons.css';
import '@fortawesome/fontawesome-free/css/all.css';
import 'vuetify/styles';
import { mdi } from 'vuetify/iconsets/mdi';
import { aliases, fa } from 'vuetify/iconsets/fa';

// Vuetify
import { createVuetify } from 'vuetify';

export default createVuetify({
  icons: {
    defaultSet: 'fa',
    aliases,
    sets: {
      fa,
      mdi
    }
  }
});
