import type { DefaultTheme } from 'vitepress'
import { readdirSync, readFileSync, statSync } from 'node:fs'
import { dirname, extname, join, relative, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

interface DocMeta {
  title: string
  sidebar: boolean
}

interface SortableItem {
  item: DefaultTheme.SidebarItem
  sortKey: string
}

const vitepressRoot = dirname(fileURLToPath(import.meta.url))
const docsRoot = resolve(vitepressRoot, '..')
const guideRoot = join(docsRoot, 'guide')
const commandsRoot = join(docsRoot, 'commands')
const guideCollator = new Intl.Collator('zh-CN', {
  numeric: true,
  sensitivity: 'base',
})
const commandCollator = new Intl.Collator('en', {
  numeric: true,
  sensitivity: 'base',
})

export const sidebar: DefaultTheme.Sidebar = [
  {
    text: '指南',
    base: '/guide/',
    items: buildGuideItems(),
  },
  {
    text: '命令',
    base: '/',
    items: buildCommandItems(),
  },
]

function buildGuideItems(): DefaultTheme.SidebarItem[] {
  const items: SortableItem[] = []

  for (const entry of readdirSync(guideRoot, { withFileTypes: true })) {
    if (!entry.isFile() || !isMarkdown(entry.name)) {
      continue
    }

    const fullPath = join(guideRoot, entry.name)
    const meta = readDocMeta(fullPath)
    if (!meta.sidebar) {
      continue
    }

    items.push({
      item: {
        text: meta.title,
        link: toGuideLink(fullPath),
      },
      sortKey: meta.title,
    })
  }

  items.sort((left, right) => guideCollator.compare(left.sortKey, right.sortKey))
  return items.map(({ item }) => item)
}

function buildCommandItems(): DefaultTheme.SidebarItem[] {
  const items: SortableItem[] = []

  for (const entry of readdirSync(commandsRoot, { withFileTypes: true })) {
    if (entry.name.startsWith('.')) {
      continue
    }

    const fullPath = join(commandsRoot, entry.name)

    if (entry.isFile()) {
      if (!isMarkdown(entry.name)) {
        continue
      }

      const meta = readDocMeta(fullPath)
      if (!meta.sidebar) {
        continue
      }

      const text = deriveRootCommandText(meta.title, fullPath)
      items.push({
        item: {
          text,
          link: toCommandLink(fullPath),
        },
        sortKey: text,
      })
      continue
    }

    if (!entry.isDirectory()) {
      continue
    }

    const indexPath = join(fullPath, 'index.md')
    const childPaths = collectMarkdownFiles(fullPath).filter(path => path !== indexPath)

    if (!safeStat(indexPath)) {
      if (childPaths.length) {
        throw new Error(`Command directory is missing index.md: ${formatPath(fullPath)}`)
      }
      continue
    }

    const meta = readDocMeta(indexPath)
    if (!meta.sidebar) {
      continue
    }

    const text = deriveRootCommandText(meta.title, indexPath)
    const link = toCommandLink(indexPath)
    const childBase = toAbsoluteBase(link)
    const item: DefaultTheme.SidebarItem = {
      text,
      link,
    }

    const childItems = childPaths
      .map((path) => {
        const childMeta = readDocMeta(path)
        if (!childMeta.sidebar) {
          return undefined
        }

        const childText = deriveChildCommandText(childMeta.title, meta.title, path)
        return {
          item: {
            text: childText,
            link: toRelativeLink(path, fullPath),
            base: childBase,
          },
          sortKey: childText,
        } satisfies SortableItem
      })
      .filter((value): value is SortableItem => value !== undefined)
      .sort((left, right) => commandCollator.compare(left.sortKey, right.sortKey))
      .map(({ item }) => item)

    if (childItems.length) {
      item.items = childItems
    }

    items.push({
      item,
      sortKey: text,
    })
  }

  items.sort((left, right) => commandCollator.compare(left.sortKey, right.sortKey))
  return items.map(({ item }) => item)
}

function collectMarkdownFiles(dir: string): string[] {
  const files: string[] = []

  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (entry.name.startsWith('.')) {
      continue
    }

    const fullPath = join(dir, entry.name)
    if (entry.isDirectory()) {
      files.push(...collectMarkdownFiles(fullPath))
      continue
    }

    if (entry.isFile() && isMarkdown(entry.name)) {
      files.push(fullPath)
    }
  }

  return files
}

function readDocMeta(filePath: string): DocMeta {
  const frontmatter = parseFrontmatter(readFileSync(filePath, 'utf8'), filePath)
  if (typeof frontmatter.title !== 'string' || !frontmatter.title.trim()) {
    throw new Error(`Missing title frontmatter: ${formatPath(filePath)}`)
  }

  if (frontmatter.sidebar !== undefined && typeof frontmatter.sidebar !== 'boolean') {
    throw new Error(`Invalid sidebar frontmatter in ${formatPath(filePath)}; expected boolean`)
  }

  return {
    title: frontmatter.title.trim(),
    sidebar: frontmatter.sidebar !== false,
  }
}

function parseFrontmatter(raw: string, filePath: string): Record<string, unknown> {
  const lines = raw.split(/\r?\n/)
  if (lines[0] !== '---') {
    throw new Error(`Missing frontmatter block: ${formatPath(filePath)}`)
  }

  let endLine = -1
  for (let index = 1; index < lines.length; index += 1) {
    if (lines[index] === '---') {
      endLine = index
      break
    }
  }

  if (endLine === -1) {
    throw new Error(`Unterminated frontmatter block: ${formatPath(filePath)}`)
  }

  const frontmatter: Record<string, unknown> = {}
  for (const line of lines.slice(1, endLine)) {
    if (!line.trim() || line.startsWith(' ') || line.startsWith('\t')) {
      continue
    }

    const match = /^([A-Za-z_][A-Za-z0-9_-]*):\s*(.*)$/.exec(line)
    if (!match) {
      continue
    }

    const key = match[1]
    if (key !== 'title' && key !== 'sidebar') {
      continue
    }

    frontmatter[key] = parseScalar(match[2])
  }

  return frontmatter
}

function parseScalar(rawValue: string): unknown {
  const value = rawValue.trim()
  if (!value) {
    return ''
  }

  if (value === 'true') {
    return true
  }

  if (value === 'false') {
    return false
  }

  if (
    (value.startsWith('"') && value.endsWith('"'))
    || (value.startsWith('\'') && value.endsWith('\''))
  ) {
    return value.slice(1, -1)
  }

  return value
}

function deriveRootCommandText(title: string, filePath: string): string {
  const prefix = 'cnb-rs '
  if (!title.startsWith(prefix)) {
    throw new Error(`Command title must start with "cnb-rs ": ${formatPath(filePath)}`)
  }

  const text = title.slice(prefix.length).trim()
  if (!text) {
    throw new Error(`Command title is missing command text: ${formatPath(filePath)}`)
  }

  return text
}

function deriveChildCommandText(title: string, parentTitle: string, filePath: string): string {
  const prefix = `${parentTitle} `
  if (!title.startsWith(prefix)) {
    throw new Error(
      `Child command title must start with "${parentTitle} ": ${formatPath(filePath)}`,
    )
  }

  const text = title.slice(prefix.length).trim()
  if (!text) {
    throw new Error(`Child command title is missing subcommand text: ${formatPath(filePath)}`)
  }

  return text
}

function toGuideLink(filePath: string): string {
  return toRelativeLink(filePath, guideRoot)
}

function toCommandLink(filePath: string): string {
  return toRelativeLink(filePath, commandsRoot)
}

function toRelativeLink(filePath: string, root: string): string {
  const rel = relative(root, filePath).replace(/\\/g, '/')
  const withoutExt = rel.replace(/\.md$/i, '')

  if (withoutExt.toLowerCase() === 'index') {
    return ''
  }

  if (withoutExt.toLowerCase().endsWith('/index')) {
    return `${withoutExt.slice(0, -'/index'.length)}/`
  }

  return withoutExt
}

function toAbsoluteBase(link: string): string {
  if (!link) {
    return '/'
  }

  return link.startsWith('/') ? link : `/${link}`
}

function safeStat(path: string) {
  try {
    return statSync(path)
  }
  catch {
    return undefined
  }
}

function formatPath(path: string): string {
  return relative(docsRoot, path).replace(/\\/g, '/')
}

function isMarkdown(name: string): boolean {
  return extname(name).toLowerCase() === '.md'
}
