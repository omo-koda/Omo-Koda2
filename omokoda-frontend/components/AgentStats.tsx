'use client'

import { useMemo } from 'react'

const statistics = [
  { label: 'Agents Active', value: '42' },
  { label: 'Tools Registered', value: '8' },
  { label: 'Providers Available', value: '4' },
  { label: 'Sandbox Invocations', value: '1.2K' },
]

export function AgentStats() {
  const stats = useMemo(() => statistics, [])

  return (
    <div className="space-y-4">
      <h2 className="text-2xl font-bold text-white">Platform Statistics</h2>
      <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
        {stats.map((stat) => (
          <div key={stat.label} className="rounded-3xl bg-white/10 p-5 border border-white/10 shadow-sm">
            <p className="text-xs uppercase tracking-[0.2em] text-gray-400">{stat.label}</p>
            <p className="mt-3 text-3xl font-semibold text-white">{stat.value}</p>
          </div>
        ))}
      </div>
    </div>
  )
}
