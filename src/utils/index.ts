import {h,Component} from "vue";
import {NIcon} from "naive-ui";

export function renderIcon(icon:Component) {
  return () => h(NIcon, null, {default: () => h(icon)});
}
