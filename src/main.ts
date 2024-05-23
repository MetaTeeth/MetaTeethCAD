// import { devtools } from "@vue/devtools";
import { createApp } from "vue";
import App from "./App.vue";

import "vuetify/styles";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

import "./assets/main.postcss";

if (process.env.NODE_ENV === "development") {
    // devtools.connect('http://localhost', 8098)
}

const vuetify = createVuetify({
    components,
    directives,
});

createApp(App).use(vuetify).mount("#app");
