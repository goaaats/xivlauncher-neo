import { appWindow } from '@tauri-apps/api/window';
import { defineComponent } from "@vue/runtime-core"
import styles from './TitleBar.module.scss'

const TitleBar = defineComponent({
    render() {
        return <div data-tauri-drag-region class={styles.titlebar}>
            <span class={styles.titlebarText}>{this.title}</span>
            <div>
                <div class={styles.titlebarButton} id="titlebar-minimize" onClick={() => appWindow.minimize()}>
                    <img
                    src="https://api.iconify.design/mdi:window-minimize.svg"
                    alt="minimize"
                    />
                </div>
                <div class={styles.titlebarButton} id="titlebar-maximize" onClick={() => appWindow.toggleMaximize()}>
                    <img
                    src="https://api.iconify.design/mdi:window-maximize.svg"
                    alt="maximize"
                    />
                </div>
                <div class={styles.titlebarButtonClose} id="titlebar-close" onClick={() => appWindow.close()}>
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
