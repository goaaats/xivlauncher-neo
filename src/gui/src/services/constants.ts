import {InjectionKey, Ref} from 'vue'
import {LauncherConfig} from '@/services/backend'

// provide/inject typing keys
export const CONFIG_KEY: InjectionKey<Ref<LauncherConfig>> = Symbol('config')
