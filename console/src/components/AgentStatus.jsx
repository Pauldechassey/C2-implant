import { useState, useEffect } from 'react'

const GREEN_THRESHOLD_SEC = 15
const ORANGE_THRESHOLD_SEC = 45

export function useAgentStatus() {
  const [lastSeen, setLastSeen] = useState(null)
  const [, setTick] = useState(0)

  useEffect(() => {
    let cancelled = false

    async function poll() {
      try {
        const res = await fetch('/api/agent/status')
        if (res.ok) {
          const data = await res.json()
          if (!cancelled) setLastSeen(data.last_seen)
        }
      } catch {}
      if (!cancelled) setTick(t => t + 1)
    }

    poll()
    const interval = setInterval(poll, 3000)
    return () => { cancelled = true; clearInterval(interval) }
  }, [])

  return lastSeen
}

function getStatus(lastSeen) {
  if (!lastSeen) return { level: 'red', label: 'no agent' }
  const elapsed = Date.now() / 1000 - lastSeen
  if (elapsed <= GREEN_THRESHOLD_SEC) return { level: 'green', label: 'agent online' }
  if (elapsed <= ORANGE_THRESHOLD_SEC) return { level: 'orange', label: 'agent stale' }
  return { level: 'red', label: 'agent offline' }
}

export default function AgentStatus({ lastSeen }) {
  const { level, label } = getStatus(lastSeen)
  return (
    <span className="agent-status">
      <span className={`dot ${level}`} />
      {label}
    </span>
  )
}
