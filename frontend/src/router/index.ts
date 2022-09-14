import { createRouter, createWebHistory } from "vue-router";
import HomeView from "../views/HomeView.vue";
import PastesView from "../views/PastesView.vue"
import SinglePasteView from "../views/SinglePasteView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: HomeView,
    },
    {
      path: "/paste/:hash",
      name: "paste",
      component: SinglePasteView,
      props: true,
    },
    {
      path: "/pastes",
      name: "pastes",
      component: PastesView,
    },
    {
      path: "/about",
      name: "about",
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import("../views/AboutView.vue"),
    },
  ],
});

export default router;
