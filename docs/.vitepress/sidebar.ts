type SidebarItem = {
  text: string
  link?: string
  items?: SidebarItem[]
}

function sortSidebarItems(items: SidebarItem[]): SidebarItem[] {
  return [...items]
    .map((item) => ({
      ...item,
      items: item.items ? sortSidebarItems(item.items) : undefined,
    }))
    .sort((left, right) => left.text.localeCompare(right.text, 'en'))
}

const guideSidebarItems: SidebarItem[] = [
  { text: '快速开始', link: '/guide/getting-started' },
  { text: 'Linux 安装说明', link: '/guide/linux-install' },
  { text: 'Windows 安装说明', link: '/guide/windows-install' },
  { text: '从 cnb 迁移到 cnb-rs', link: '/guide/migrate-cnb-to-cnb-rs' },
  { text: 'cnb-rs', link: '/guide/cnb' },
  { text: '发布流程', link: '/guide/release-process' },
]

const commandSidebarItems = sortSidebarItems([
  {
    text: 'auth',
    link: '/auth/',
    items: [
      { text: 'login', link: '/auth/login' },
      { text: 'status', link: '/auth/status' },
      { text: 'logout', link: '/auth/logout' },
    ],
  },
  {
    text: 'badge',
    link: '/badge/',
    items: [
      { text: 'get', link: '/badge/get' },
      { text: 'list', link: '/badge/list' },
      { text: 'upload', link: '/badge/upload' },
    ],
  },
  { text: 'browse', link: '/browse' },
  {
    text: 'build',
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
  { text: 'chat', link: '/chat' },
  {
    text: 'commit',
    link: '/commit/',
    items: [
      { text: 'asset-stats', link: '/commit/asset-stats' },
      { text: 'asset-clean', link: '/commit/asset-clean' },
      { text: 'asset-upload', link: '/commit/asset-upload' },
    ],
  },
  { text: 'completion', link: '/completion' },
  {
    text: 'config',
    link: '/config/',
    items: [
      { text: 'get', link: '/config/get' },
      { text: 'list', link: '/config/list' },
      { text: 'set', link: '/config/set' },
    ],
  },
  { text: 'download', link: '/download' },
  {
    text: 'gpg-key',
    link: '/gpg-key/',
    items: [{ text: 'list', link: '/gpg-key/list' }],
  },
  {
    text: 'group',
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
  { text: 'info', link: '/info' },
  {
    text: 'issue',
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
    text: 'knowledge',
    link: '/knowledge/',
    items: [
      { text: 'list-models', link: '/knowledge/list-models' },
      { text: 'info', link: '/knowledge/info' },
      { text: 'clean', link: '/knowledge/clean' },
      { text: 'query', link: '/knowledge/query' },
    ],
  },
  {
    text: 'label',
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
    text: 'member',
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
  {
    text: 'mission',
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
    text: 'pr',
    link: '/pr/',
    items: [
      { text: 'list', link: '/pr/list' },
      { text: 'create', link: '/pr/create' },
      { text: 'update', link: '/pr/update' },
      { text: 'merge', link: '/pr/merge' },
    ],
  },
  {
    text: 'release',
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
    text: 'registry',
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
    text: 'repo',
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
  { text: 'stars', link: '/stars' },
  { text: 'stats', link: '/stats' },
  {
    text: 'user',
    link: '/user/',
    items: [
      { text: 'followers', link: '/user/followers' },
      { text: 'following', link: '/user/following' },
      { text: 'activities', link: '/user/activities' },
      { text: 'activity-detail', link: '/user/activity-detail' },
    ],
  },
  { text: 'version', link: '/version' },
  {
    text: 'workspace',
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
])

export const sidebar: SidebarItem[] = [
  {
    text: '指南',
    items: guideSidebarItems,
  },
  ...commandSidebarItems,
]
