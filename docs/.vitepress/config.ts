import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'CNB CLI',
  description: 'CNB 平台专属命令行工具文档',
  lang: 'zh-CN',
  lastUpdated: true,
  cleanUrls: true,

  themeConfig: {
    nav: [{ text: '指南', link: '/guide/getting-started' }],

    sidebar: [
      {
        text: '指南',
        items: [
          { text: '快速开始', link: '/guide/getting-started' },
          { text: 'cnb', link: '/guide/cnb' },
        ],
      },
      {
        text: '常用命令',
        items: [
          {
            text: 'cnb auth',
            link: '/auth/',
            items: [
              { text: 'login', link: '/auth/login' },
              { text: 'status', link: '/auth/status' },
              { text: 'logout', link: '/auth/logout' },
            ],
          },
          { text: 'cnb chat', link: '/chat' },
          {
            text: 'cnb config',
            link: '/config/',
            items: [
              { text: 'get', link: '/config/get' },
              { text: 'list', link: '/config/list' },
              { text: 'set', link: '/config/set' },
            ],
          },
          {
            text: 'cnb issue',
            link: '/issue/',
            items: [
              { text: 'list', link: '/issue/list' },
              { text: 'mine', link: '/issue/mine' },
              { text: 'create', link: '/issue/create' },
              { text: 'close', link: '/issue/close' },
              { text: 'comment', link: '/issue/comment' },
              { text: 'exist', link: '/issue/exist' },
              { text: 'download', link: '/issue/download' },
              { text: 'assigners', link: '/issue/assigners' },
            ],
          },
          {
            text: 'cnb pull',
            link: '/pull/',
            items: [
              { text: 'list', link: '/pull/list' },
              { text: 'create', link: '/pull/create' },
              { text: 'update', link: '/pull/update' },
              { text: 'merge', link: '/pull/merge' },
            ],
          },
          {
            text: 'cnb release',
            link: '/release/',
            items: [
              { text: 'list', link: '/release/list' },
              { text: 'create', link: '/release/create' },
              { text: 'asset-stats', link: '/release/asset-stats' },
              { text: 'asset-clean', link: '/release/asset-clean' },
              { text: 'asset-upload', link: '/release/asset-upload' },
            ],
          },
          {
            text: 'cnb commit',
            link: '/commit/',
            items: [
              { text: 'asset-stats', link: '/commit/asset-stats' },
              { text: 'asset-clean', link: '/commit/asset-clean' },
              { text: 'asset-upload', link: '/commit/asset-upload' },
            ],
          },
        ],
      },
      {
        text: '仓库命令',
        items: [
          {
            text: 'cnb repo',
            link: '/repo/',
            items: [
              { text: 'archive', link: '/repo/archive' },
              { text: 'asset', link: '/repo/asset' },
              { text: 'clone', link: '/repo/clone' },
              { text: 'contributor', link: '/repo/contributor' },
              { text: 'create', link: '/repo/create' },
              { text: 'delete', link: '/repo/delete' },
              { text: 'edit', link: '/repo/edit' },
              { text: 'events', link: '/repo/events' },
              { text: 'fork', link: '/repo/fork' },
              { text: 'list', link: '/repo/list' },
              { text: 'pin', link: '/repo/pin' },
              { text: 'security', link: '/repo/security' },
              { text: 'top-contributors', link: '/repo/top-contributors' },
              { text: 'transfer', link: '/repo/transfer' },
              { text: 'unarchive', link: '/repo/unarchive' },
              { text: 'view', link: '/repo/view' },
              { text: 'visibility', link: '/repo/visibility' },
            ],
          },
          { text: 'cnb download', link: '/download' },
          { text: 'cnb info', link: '/info' },
          { text: 'cnb stats', link: '/stats' },
          { text: 'cnb stars', link: '/stars' },
        ],
      },
      {
        text: '其他命令',
        items: [
          { text: 'cnb completion', link: '/completion' },
          {
            text: 'cnb group',
            link: '/group/',
            items: [{ text: 'update-logo', link: '/group/update-logo' }],
          },
          {
            text: 'cnb knowledge',
            link: '/knowledge/',
            items: [
              { text: 'list-models', link: '/knowledge/list-models' },
              { text: 'info', link: '/knowledge/info' },
              { text: 'clean', link: '/knowledge/clean' },
              { text: 'query', link: '/knowledge/query' },
            ],
          },
          {
            text: 'cnb workspace',
            link: '/workspace/',
            items: [{ text: 'closed-clean', link: '/workspace/closed-clean' }],
          },
          { text: 'cnb version', link: '/version' },
        ],
      },
    ],

    socialLinks: [{ icon: 'github', link: 'https://cnb.cool/prevailna/cnb' }],

    footer: {
      message: '© 2026 wwvo',
    },

    outline: {
      label: '页面导航',
    },

    lastUpdated: {
      text: '最后更新于',
    },

    docFooter: {
      prev: '上一页',
      next: '下一页',
    },

    search: {
      provider: 'local',
      options: {
        translations: {
          button: { buttonText: '搜索文档' },
          modal: {
            noResultsText: '无法找到相关结果',
            resetButtonTitle: '清除查询条件',
            footer: { selectText: '选择', navigateText: '切换' },
          },
        },
      },
    },
  },
})
