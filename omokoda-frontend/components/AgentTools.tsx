'use client'

import { useMemo } from 'react'
import { Tool } from '@/types'

const tools: Tool[] = [
  {
    name: 'wasm',
    description: 'Execute a sandboxed WebAssembly module',
    requiredTier: 2,
    category: 'computation',
  },
  {
    name: 'read_file',
    description: 'Read a file from the local workspace',
    requiredTier: 0,
    category: 'system',
  },
  {
    name: 'web_search',
    description: 'Search the web for relevant context',
    requiredTier: 1,
    category: 'web',
  },
]

export function AgentTools() {
  const sortedTools = useMemo(
    () => [...tools].sort((a, b) => a.requiredTier - b.requiredTier),
    []
  )

  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold text-white">Agent Tools</h2>
      <p className="text-gray-300">Browse available Omo-Koda tools and capabilities.</p>

      <div className="grid gap-4 md:grid-cols-2">
        {sortedTools.map((tool) => (
          <div key={tool.name} className="rounded-2xl bg-white/10 p-4 border border-white/10 shadow-sm">
            <div className="flex items-center justify-between mb-3">
              <h3 className="text-lg font-semibold text-white">{tool.name}</h3>
              <span className="rounded-full bg-blue-600 px-3 py-1 text-xs uppercase tracking-wide text-white">
                Tier {tool.requiredTier}
              </span>
            </div>
            <p className="text-gray-300 mb-3">{tool.description}</p>
            <p className="text-xs text-gray-500">Category: {tool.category}</p>
          </div>
        ))}
      </div>
    </div>
  )
}
