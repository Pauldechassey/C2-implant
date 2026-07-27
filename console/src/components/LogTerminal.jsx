import { useEffect, useRef, useState } from 'react'

const NEAR_BOTTOM_PX = 40

export default function LogTerminal() {
  const [logs, setLogs] = useState([])
  const [autoScroll, setAutoScroll] = useState(true)
  const contentRef = useRef(null)

  useEffect(() => {
    let active = true
    let ws
    let retryTimeout

    function pushLog(text) {
      const time = new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' })
      setLogs(prev => [...prev.slice(-200), `${time}  ${text}`])
    }

    function connect() {
      const proto = location.protocol === 'https:' ? 'wss:' : 'ws:'
      ws = new WebSocket(`${proto}//${location.host}/ws/logs`)

      ws.onopen = () => { if (active) pushLog('-- connected --') }
      ws.onmessage = e => { if (active) pushLog(e.data) }
      ws.onclose = () => {
        if (!active) return
        pushLog('-- disconnected, retrying... --')
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

  useEffect(() => {
    if (autoScroll && contentRef.current) {
      contentRef.current.scrollTo({ top: contentRef.current.scrollHeight, behavior: 'smooth' })
    }
  }, [logs, autoScroll])

  function handleScroll() {
    const el = contentRef.current
    if (!el) return
    const nearBottom = el.scrollHeight - el.scrollTop - el.clientHeight < NEAR_BOTTOM_PX
    setAutoScroll(nearBottom)
  }

  return (
    <div className="log-terminal">
      <div className="panel-header">LOGS</div>
      <div className="log-content" ref={contentRef} onScroll={handleScroll}>
        {logs.map((log, i) => (
          <div key={i} className="log-line">{log}</div>
        ))}
      </div>
      {!autoScroll && (
        <button className="jump-bottom-btn" onClick={() => setAutoScroll(true)}>jump to bottom ↓</button>
      )}
    </div>
  )
}
