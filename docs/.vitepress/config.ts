import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'CNB CLI',
  description: '一个非官方的 CNB 命令行工具文档',
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
              { text: 'view', link: '/issue/view' },
              { text: 'create', link: '/issue/create' },
              { text: 'edit', link: '/issue/edit' },
              { text: 'close', link: '/issue/close' },
              { text: 'reopen', link: '/issue/reopen' },
              { text: 'comment', link: '/issue/comment' },
              { text: 'exist', link: '/issue/exist' },
              { text: 'download', link: '/issue/download' },
              { text: 'assigners', link: '/issue/assigners' },
              { text: 'label', link: '/issue/label' },
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
              { text: 'view', link: '/release/view' },
              { text: 'create', link: '/release/create' },
              { text: 'update', link: '/release/update' },
              { text: 'delete', link: '/release/delete' },
              { text: 'latest', link: '/release/latest' },
              { text: 'download', link: '/release/download' },
              { text: 'asset-upload', link: '/release/asset-upload' },
              { text: 'asset-delete', link: '/release/asset-delete' },
              { text: 'asset-stats', link: '/release/asset-stats' },
              { text: 'asset-clean', link: '/release/asset-clean' },
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
              { text: 'settings', link: '/repo/settings' },
              { text: 'top-contributors', link: '/repo/top-contributors' },
              { text: 'transfer', link: '/repo/transfer' },
              { text: 'unarchive', link: '/repo/unarchive' },
              { text: 'view', link: '/repo/view' },
              { text: 'visibility', link: '/repo/visibility' },
            ],
          },
          {
            text: 'cnb label',
            link: '/label/',
            items: [
              { text: 'list', link: '/label/list' },
              { text: 'create', link: '/label/create' },
              { text: 'update', link: '/label/update' },
              { text: 'delete', link: '/label/delete' },
              { text: 'issue-list', link: '/label/issue-list' },
              { text: 'issue-add', link: '/label/issue-add' },
              { text: 'issue-set', link: '/label/issue-set' },
              { text: 'issue-remove', link: '/label/issue-remove' },
              { text: 'issue-clear', link: '/label/issue-clear' },
              { text: 'pull-list', link: '/label/pull-list' },
              { text: 'pull-add', link: '/label/pull-add' },
              { text: 'pull-set', link: '/label/pull-set' },
              { text: 'pull-remove', link: '/label/pull-remove' },
              { text: 'pull-clear', link: '/label/pull-clear' },
            ],
          },
          {
            text: 'cnb build',
            link: '/build/',
            items: [
              { text: 'start', link: '/build/start' },
              { text: 'stop', link: '/build/stop' },
              { text: 'status', link: '/build/status' },
              { text: 'list', link: '/build/list' },
              { text: 'stage', link: '/build/stage' },
              { text: 'download-log', link: '/build/download-log' },
              { text: 'delete-log', link: '/build/delete-log' },
              { text: 'crontab-sync', link: '/build/crontab-sync' },
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
            items: [
              { text: 'list', link: '/group/list' },
              { text: 'view', link: '/group/view' },
              { text: 'create', link: '/group/create' },
              { text: 'update', link: '/group/update' },
              { text: 'delete', link: '/group/delete' },
              { text: 'transfer', link: '/group/transfer' },
              { text: 'sub-groups', link: '/group/sub-groups' },
              { text: 'settings', link: '/group/settings' },
              { text: 'quota', link: '/group/quota' },
              { text: 'member', link: '/group/member' },
              { text: 'update-logo', link: '/group/update-logo' },
            ],
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
            items: [
              { text: 'list', link: '/workspace/list' },
              { text: 'start', link: '/workspace/start' },
              { text: 'stop', link: '/workspace/stop' },
              { text: 'delete', link: '/workspace/delete' },
              { text: 'detail', link: '/workspace/detail' },
              { text: 'closed-clean', link: '/workspace/closed-clean' },
            ],
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
