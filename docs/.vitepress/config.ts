import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'CNB CLI',
  description: '一个非官方的 CNB 命令行工具文档',
  lang: 'zh-CN',
  lastUpdated: true,
  cleanUrls: true,
  // ignoreDeadLinks: true,
  metaChunk: true,
  appearance: 'dark',

  rewrites: {
    'commands/:slug*': ':slug*',
  },

  markdown: {
    // See: https://vitepress.dev/reference/site-config#markdown
    theme: {
      light: 'vitesse-light',
      dark: 'vitesse-dark',
    },
    lineNumbers: true,
    defaultHighlightLang: 'bash',
    headers: {
      level: [1, 5],
    },
    // math: true,
    toc: { level: [1, 6] },
    image: {
      lazyLoading: true,
    },

    config(md: any) {},
  },

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
            text: 'cnb badge',
            link: '/badge/',
            items: [
              { text: 'get', link: '/badge/get' },
              { text: 'list', link: '/badge/list' },
              { text: 'upload', link: '/badge/upload' },
            ],
          },
          {
            text: 'cnb member',
            link: '/member/',
            items: [
              { text: 'repo-list', link: '/member/repo-list' },
              { text: 'repo-add', link: '/member/repo-add' },
              { text: 'repo-update', link: '/member/repo-update' },
              { text: 'repo-remove', link: '/member/repo-remove' },
              { text: 'repo-access-level', link: '/member/repo-access-level' },
              { text: 'repo-user-access', link: '/member/repo-user-access' },
              { text: 'repo-inherited', link: '/member/repo-inherited' },
              { text: 'repo-all', link: '/member/repo-all' },
              { text: 'group-list', link: '/member/group-list' },
              { text: 'group-add', link: '/member/group-add' },
              { text: 'group-update', link: '/member/group-update' },
              { text: 'group-remove', link: '/member/group-remove' },
              { text: 'group-access-level', link: '/member/group-access-level' },
              { text: 'group-user-access', link: '/member/group-user-access' },
              { text: 'group-inherited', link: '/member/group-inherited' },
              { text: 'collaborator-list', link: '/member/collaborator-list' },
              { text: 'collaborator-update', link: '/member/collaborator-update' },
              { text: 'collaborator-remove', link: '/member/collaborator-remove' },
            ],
          },
          { text: 'cnb browse', link: '/browse' },
          { text: 'cnb download', link: '/download' },
          { text: 'cnb info', link: '/info' },
          { text: 'cnb stats', link: '/stats' },
          { text: 'cnb stars', link: '/stars' },
        ],
      },
      {
        text: '组织与制品管理',
        items: [
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
            text: 'cnb mission',
            link: '/mission/',
            items: [
              { text: 'list', link: '/mission/list' },
              { text: 'create', link: '/mission/create' },
              { text: 'delete', link: '/mission/delete' },
              { text: 'set-visibility', link: '/mission/set-visibility' },
              { text: 'view list', link: '/mission/view-list' },
              { text: 'view get', link: '/mission/view-get' },
              { text: 'view set', link: '/mission/view-set' },
              { text: 'view add', link: '/mission/view-add' },
              { text: 'view sort', link: '/mission/view-sort' },
            ],
          },
          {
            text: 'cnb registry',
            link: '/registry/',
            items: [
              { text: 'list', link: '/registry/list' },
              { text: 'delete', link: '/registry/delete' },
              { text: 'set-visibility', link: '/registry/set-visibility' },
              { text: 'package list', link: '/registry/package-list' },
              { text: 'package detail', link: '/registry/package-detail' },
              { text: 'package delete', link: '/registry/package-delete' },
              { text: 'tag list', link: '/registry/tag-list' },
              { text: 'tag detail', link: '/registry/tag-detail' },
              { text: 'tag delete', link: '/registry/tag-delete' },
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
        ],
      },
      {
        text: '用户与安全',
        items: [
          {
            text: 'cnb user',
            link: '/user/',
            items: [
              { text: 'followers', link: '/user/followers' },
              { text: 'following', link: '/user/following' },
              { text: 'activities', link: '/user/activities' },
              { text: 'activity-detail', link: '/user/activity-detail' },
            ],
          },
          {
            text: 'cnb gpg-key',
            link: '/gpg-key/',
            items: [{ text: 'list', link: '/gpg-key/list' }],
          },
        ],
      },
      {
        text: '其他命令',
        items: [
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
          { text: 'cnb completion', link: '/completion' },
          { text: 'cnb version', link: '/version' },
        ],
      },
    ],

    socialLinks: [
      // { icon: 'github', link: 'https://cnb.cool/wwvo/cnb-cli/cnb' },
      {
        icon: {
          svg: '<svg role="img" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><g fill="none"><path d="M11.5286 1.87149C11.5769 1.73005 11.5356 1.5733 11.4233 1.47452C11.0472 1.14247 10.0965 0.443125 8.66911 0.339708C7.07054 0.223769 6.08089 0.652279 5.58096 0.969951C5.36531 1.10676 5.35326 1.41748 5.55499 1.57422L9.62723 4.73936C9.98617 5.01807 10.5125 4.8604 10.6591 4.43003L11.5286 1.87149Z" fill="#ff6200"></path><path d="M1.49017 11.2664C1.32368 11.3781 1.24855 11.584 1.30235 11.7774C1.45724 12.3339 1.91868 13.4919 3.22833 14.5456C4.53797 15.5992 6.08738 15.7128 6.74962 15.6966C6.94764 15.692 7.12016 15.5617 7.17998 15.3724L9.79046 7.11064C9.97875 6.51425 9.31048 6.01386 8.79154 6.3626L1.49017 11.2664Z" fill="#ff6200"></path><path d="M3.39813 2.54827C3.27013 2.49773 3.12683 2.50607 3.00579 2.57193C2.52256 2.83488 1.28526 3.64506 0.647135 5.30947C0.154627 6.59222 0.328071 8.01085 0.463488 8.70463C0.508009 8.9314 0.747306 9.06218 0.962489 8.97824L8.79485 5.92024C9.35414 5.70181 9.35646 4.91111 8.7981 4.6899L3.39813 2.54827Z" fill="#ff6200"></path><path d="M15.0167 8.46843C15.243 8.62194 15.5528 8.48652 15.5922 8.21569C15.6961 7.49872 15.7861 6.25076 15.371 5.30933C14.8177 4.05487 13.8786 3.28133 13.433 2.9669C13.292 2.86766 13.1019 2.87786 12.9725 2.99241L10.9959 4.74541C10.6732 5.03154 10.7066 5.54492 11.0636 5.78746L15.0167 8.46936V8.46843Z" fill="#ff6200"></path><path d="M9.49413 15.1604C9.47372 15.3937 9.67128 15.5866 9.90409 15.5616C10.6531 15.4813 12.1918 15.1841 13.3447 14.0827C14.467 13.0109 14.832 11.7384 14.9382 11.2319C14.9669 11.0951 14.9326 10.9528 14.8445 10.8442L11.3886 6.57909C11.0143 6.11719 10.2681 6.34535 10.2162 6.93757L9.49366 15.1604H9.49413Z" fill="#ff6200"></path></g></svg>',
        },
        link: 'https://cnb.cool/wwvo/cnb-cli/cnb',
        ariaLabel: 'CNB',
      },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2026-present wwvo | ❤️ <a href="https://vitepress.vuejs.org" target="_blank" rel="noopener noreferrer">VitePress</a>',
    },

    editLink: {
      pattern: 'https://cnb.cool/wwvo/cnb-cli/cnb/-/blob/main/docs/:path',
      text: '在 CNB 上编辑此页面', // 'Edit this page on CNB'
    },

    outline: {
      level: 'deep',
      label: '页面导航',
    },

    lastUpdated: {
      text: '最后更新于', // "Updated at"
    },

    docFooter: {
      prev: '上一页',
      next: '下一页',
    },

    notFound: {
      title: '页面未找到',
      quote: '但如果你不改变方向，并且继续寻找，你可能最终会到达你所前往的地方。',
      linkLabel: '前往首页',
      linkText: '带我回首页',
    },

    langMenuLabel: '多语言',
    returnToTopLabel: '回到顶部',
    sidebarMenuLabel: '菜单',
    darkModeSwitchLabel: '主题',
    lightModeSwitchTitle: '切换到浅色模式',
    darkModeSwitchTitle: '切换到深色模式',
    skipToContentLabel: '跳转到内容',

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
