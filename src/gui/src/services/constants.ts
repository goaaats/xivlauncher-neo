import {InjectionKey, Ref} from 'vue'
import {AccountEntry, AddonEntry, LauncherSettings, UidCacheEntry} from '@/services/backend'

// provide/inject typing keys
export const SETTINGS_KEY: InjectionKey<Ref<LauncherSettings>> = Symbol('settings')
export const ADDONS_KEY: InjectionKey<Ref<AddonEntry[]>> = Symbol('addons')
export const ACCOUNTS_KEY: InjectionKey<Ref<AccountEntry[]>> = Symbol('accounts')
export const UID_CACHE_KEY: InjectionKey<Ref<UidCacheEntry[]>> = Symbol('uidCache')
