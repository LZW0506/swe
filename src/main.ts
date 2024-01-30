import {createApp} from 'vue';
import App from './App.vue';
import router from "./router";
import Antd, {message} from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';

const app = createApp(App);
message.config({
    duration: 2,
});
app.use(Antd).use(router).mount('#app');