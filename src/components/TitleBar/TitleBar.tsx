import { appWindow } from '@tauri-apps/api/window';
import { defineComponent } from "@vue/runtime-core"
import styles from './TitleBar.module.scss'

const TitleBar = defineComponent({
    mounted() {
        document
            ?.getElementById('titlebar-minimize')
            ?.addEventListener('click', () => appWindow.minimize());

        document
            ?.getElementById('titlebar-maximize')
            ?.addEventListener('click', () => appWindow.toggleMaximize());

        document
            ?.getElementById('titlebar-close')
            ?.addEventListener('click', () => appWindow.close());
    },
    render() {
        return <div data-tauri-drag-region class={styles.titlebar}>
            <span class={styles.titlebarText}>{this.title}</span>
            <div>
                <div class={styles.titlebarButton} id="titlebar-minimize">
                    <img
                    src="https://api.iconify.design/mdi:window-minimize.svg"
                    alt="minimize"
                    />
                </div>
                <div class={styles.titlebarButton} id="titlebar-maximize">
                    <img
                    src="https://api.iconify.design/mdi:window-maximize.svg"
                    alt="maximize"
                    />
                </div>
                <div class={styles.titlebarButtonClose} id="titlebar-close">
                    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
                </div>
            </div>
        </div>
    },
    props: {
        title: String,
    },
})

export {
    TitleBar,
};
