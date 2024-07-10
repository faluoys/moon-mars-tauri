import {defineConfig} from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from 'tailwindcss';
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'
import {resolve} from "path";
// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [
        vue(),
        AutoImport({
            include: [
                /.(vue)$/,
                /.(js|jsx|ts|tsx)$/,
                /.(cjs|mjs)$/,
                /.(json|json5)$/,
            ],
            imports: [
                'vue',
                'vue-router',
                {
                    'naive-ui': [
                        'useDialog',
                        'useMessage',
                        'useNotification',
                        'useLoadingBar'
                    ]
                }
            ],
            dts: 'types/auto-imports.d.ts'
        }),
        Components({
            resolvers: [NaiveUiResolver()],
            dts: 'types/components.d.ts'
        }),
    ],
    resolve: {
        alias: {
            '@': resolve(__dirname, 'src'),
        }
    },
    css: {
        postcss: {
            plugins: [tailwindcss],
        }
    },
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 1420,
        strictPort: true,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
}));
