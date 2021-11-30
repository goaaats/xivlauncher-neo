import {Ref, provide, inject} from 'vue'
import {AccountEntry, LauncherConfig} from '@/services/backend'

class Provisioner<T> {
  private readonly key: string

  constructor(key: string) {
    this.key = key
  }

  public provide(value: T): T {
    provide(this.key, value)
    return value
  }

  public inject(): T {
    return inject(this.key) as T
  }
}

// App
export const CONFIG = new Provisioner<Ref<LauncherConfig>>('config')

// Main
export const SHOW_MAINTENANCE_DIALOG = new Provisioner<Ref<boolean>>('showMaintenanceDialog')
export const SHOW_ACCOUNTS_DIALOG = new Provisioner<Ref<boolean>>('showAccountsDialog')
export const SHOW_EDIT_ACCOUNT_DIALOG = new Provisioner<Ref<boolean>>('showEditAccountDialog')

// Accounts
export const CURRENT_ACCOUNT = new Provisioner<Ref<AccountEntry>>('currentAccount')
