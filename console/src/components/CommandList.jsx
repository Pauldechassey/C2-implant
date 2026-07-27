import { useState } from 'react'
import AgentStatus from './AgentStatus'

export default function CommandList({ commands, selectedId, onSelect, onDelete, lastSeen }) {
  const [openId, setOpenId] = useState(null)

  function toggleMenu(e, id) {
    e.stopPropagation()
    setOpenId(prev => prev === id ? null : id)
  }

  function handleDelete(e, id) {
    e.stopPropagation()
    setOpenId(null)
    if (window.confirm('Supprimer cette commande ?')) {
      onDelete(id)
    }
  }

  return (
    <div className="command-list">
      <div className="panel-header panel-header-row">
        <span>COMMANDS</span>
        <AgentStatus lastSeen={lastSeen} />
      </div>
      <table className="cmd-table">
        <thead>
          <tr>
            <th>#</th>
            <th>COMMAND</th>
            <th>STATUS</th>
            <th>TIME</th>
            <th></th>
          </tr>
        </thead>
        <tbody>
          {commands.length === 0 && (
            <tr><td colSpan={5} className="empty">no commands</td></tr>
          )}
          {commands.map(cmd => (
            <tr
              key={cmd.id}
              className={`cmd-row ${selectedId === cmd.id ? 'active' : ''}`}
              onClick={() => { setOpenId(null); onSelect(cmd.id) }}
            >
              <td className="dim">{cmd.order}</td>
              <td className="cmd-text">{cmd.command}</td>
              <td><StatusBadge status={cmd.status} /></td>
              <td className="dim">{formatTime(cmd.updated_at)}</td>
              <td className="cmd-menu-cell">
                <button className="menu-dots" onClick={e => toggleMenu(e, cmd.id)}>⋯</button>
                {openId === cmd.id && (
                  <button className="menu-delete" onClick={e => handleDelete(e, cmd.id)}>
                    supprimer
                  </button>
                )}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  )
}

function formatTime(iso) {
  if (!iso) return ''
  return new Date(iso).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' })
}

function StatusBadge({ status }) {
  const cls = { DONE: 'status-done', PENDING: 'status-pending', FAILED: 'status-failed' }[status] ?? ''
  return <span className={`status ${cls}`}>{status}</span>
}
