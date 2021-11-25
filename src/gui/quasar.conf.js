/*
 * This file runs in a Node context (it's NOT transpiled by Babel), so use only
 * the ES6 features that are supported by your Node version. https://node.green/
 */

// Configuration for your app
// https://quasar.dev/quasar-cli/quasar-conf-js

/* eslint-env node */
/* eslint-disable @typescript-eslint/no-var-requires */
const {configure} = require('quasar/wrappers')
const path = require('path')

module.exports = configure(function (ctx) {
  return {
    // https://quasar.dev/quasar-cli/supporting-ts
    supportTS: {
      tsCheckerConfig: {
        eslint: {
          enabled: true,
          files: './src/**/*.{ts,tsx,js,jsx,vue}',
        },
      }
    },

    // https://quasar.dev/quasar-cli/boot-files
    boot: [
      'logging',
      'i18n',
      'error',
    ],

    // https://quasar.dev/quasar-cli/quasar-conf-js#Property%3A-css
    css: [
      'app.sass'
    ],

    // https://github.com/quasarframework/quasar/tree/dev/extras
    extras: [
      // 'ionicons-v4',
      // 'fontawesome-v5',
      // 'eva-icons',
      // 'themify',
      // 'line-awesome',
      // 'material-icons',
      // 'roboto-font-latin-ext', // this or either 'roboto-font', NEVER both!
      'roboto-font',
      'mdi-v6',
    ],

    // Full list of options: https://quasar.dev/quasar-cli/quasar-conf-js#Property%3A-build
    build: {
      env: {
        NODE_OPTIONS: '--trace-warnings'
      },

      // transpile: false,
      // publicPath: '/',

      // Add dependencies for transpiling with Babel (Array of string/regex)
      // (from node_modules, which are by default not transpiled).
      // Applies only if "transpile" is set to true.
      // transpileDependencies: [],

      // rtl: true, // https://quasar.dev/options/rtl-support

      // https://quasar.dev/quasar-cli/handling-webpack
      chainWebpack(chain) {
        chain.resolve
          .alias.set('@', path.resolve(__dirname, 'src/'))
      },
    },

    // Full list of options: https://quasar.dev/quasar-cli/quasar-conf-js#Property%3A-devServer
    devServer: {
      https: false,
      host: 'localhost',
      port: 8080,
      open: false
    },

    // https://quasar.dev/quasar-cli/quasar-conf-js#Property%3A-framework
    framework: {
      config: {
        'notify': {}
      },
      iconSet: 'mdi-v5',
      plugins: [
        'Notify'
      ]
    },

    // https://quasar.dev/options/animations
    animations: 'all',
  }
})
