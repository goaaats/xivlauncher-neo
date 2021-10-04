import { defineComponent } from "@vue/runtime-core"
import styles from './test.module.scss'

const Test = defineComponent({
    render() {
        return <h1 class={styles.myHeader}>Hello World</h1>
    }
})

export {
    Test,
}