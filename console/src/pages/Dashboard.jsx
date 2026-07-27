import { useState, useEffect } from 'react'
import CommandList from '../components/CommandList'
import CommandResult from '../components/CommandResult'
import CommandForm from '../components/CommandForm'
import LogTerminal from '../components/LogTerminal'
import Splitter from '../components/Splitter'
import { useAgentStatus } from '../components/AgentStatus'
import { getCommands, deleteCommand } from '../api/client'

function clamp(value, min, max) {
  return Math.min(max, Math.max(min, value))
}

function usePersistedSize(key, defaultValue) {
  const [value, setValue] = useState(() => Number(localStorage.getItem(key)) || defaultValue)
  function update(updater) {
    setValue(prev => {
      const next = typeof updater === 'function' ? updater(prev) : updater
      localStorage.setItem(key, next)
      return next
    })
  }
  return [value, update]
}

export default function Dashboard() {
  const [commands, setCommands] = useState([])
  const [selectedId, setSelectedId] = useState(null)
  const [panelWidth, setPanelWidth] = usePersistedSize('panelWidth', 360)
  const [logHeight, setLogHeight] = usePersistedSize('logHeight', 280)
  const lastSeen = useAgentStatus()

  useEffect(() => {
    fetchCommands()

    let active = true
    let ws
    let retryTimeout

    function connect() {
      const proto = location.protocol === 'https:' ? 'wss:' : 'ws:'
      ws = new WebSocket(`${proto}//${location.host}/ws/commands`)
      ws.onmessage = () => fetchCommands()
      ws.onclose = () => {
        if (!active) return
        retryTimeout = setTimeout(connect, 2000)
      }
    }
    connect()

    return () => {
      active = false
      clearTimeout(retryTimeout)
      ws.close()
    }
  }, [])

  async function fetchCommands() {
    try {
      const data = await getCommands()
      setCommands(data)
    } catch {}
  }

  async function handleDelete(id) {
    try {
      await deleteCommand(id)
      if (selectedId === id) setSelectedId(null)
    } catch {}
  }

  const selected = commands.find(c => c.id === selectedId) ?? null

  return (
    <div className="dashboard" style={{ gridTemplateRows: `1fr 6px ${logHeight}px` }}>
      <div className="dashboard-body" style={{ gridTemplateColumns: `${panelWidth}px 6px 1fr` }}>
        <div className="command-panel">
          <CommandList commands={commands} selectedId={selectedId} onSelect={setSelectedId} onDelete={handleDelete} lastSeen={lastSeen} />
          <CommandForm
            onCreated={fetchCommands}
            nextOrder={commands.length + 1}
          />
        </div>
        <Splitter direction="vertical" onResize={dx => setPanelWidth(w => clamp(w + dx, 240, 720))} />
        <CommandResult command={selected} />
      </div>

      <Splitter direction="horizontal" onResize={dy => setLogHeight(h => clamp(h - dy, 120, 640))} />
      <LogTerminal />
    </div>
  )
}
