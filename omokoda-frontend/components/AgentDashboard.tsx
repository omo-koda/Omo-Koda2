'use client'

import { useState } from 'react'
import { AgentChat } from './AgentChat'
import { AgentProfile } from './AgentProfile'
import { AgentTools } from './AgentTools'
import { AgentStats } from './AgentStats'

export function AgentDashboard() {
  const [activeTab, setActiveTab] = useState<'chat' | 'profile' | 'tools' | 'stats'>('chat')

  const tabs = [
    { id: 'chat', label: 'Chat', component: AgentChat },
    { id: 'profile', label: 'Profile', component: AgentProfile },
    { id: 'tools', label: 'Tools', component: AgentTools },
    { id: 'stats', label: 'Statistics', component: AgentStats },
  ] as const

  const ActiveComponent = tabs.find(tab => tab.id === activeTab)?.component || AgentChat

  return (
    <div className="max-w-6xl mx-auto">
      {/* Tab Navigation */}
      <div className="flex space-x-1 mb-6 bg-white/10 rounded-lg p-1 backdrop-blur-sm">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={`flex-1 py-2 px-4 rounded-md text-sm font-medium transition-all ${
              activeTab === tab.id
                ? 'bg-white text-gray-900 shadow-lg'
                : 'text-gray-300 hover:text-white hover:bg-white/10'
            }`}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Tab Content */}
      <div className="bg-white/10 backdrop-blur-sm rounded-lg p-6">
        <ActiveComponent />
      </div>
    </div>
  )
}