import {h,Component} from "vue";
import {NIcon} from "naive-ui";

export function renderIcon(icon:Component) {
  return () => h(NIcon, null, {default: () => h(icon)});
}

// 通用防抖函数
export function debounce<T extends (...args: any[]) => any>(fn: T, delay = 200) {
  let timer: ReturnType<typeof setTimeout> | null = null;
  return function(this: any, ...args: Parameters<T>) {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => {
      fn.apply(this, args)
    }, delay)
  }
}
