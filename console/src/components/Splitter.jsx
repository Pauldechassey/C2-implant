import { useRef } from 'react'

export default function Splitter({ direction, onResize }) {
  const dragging = useRef(false)

  function handleMouseDown(e) {
    e.preventDefault()
    dragging.current = true
    document.body.style.cursor = direction === 'vertical' ? 'col-resize' : 'row-resize'
    document.body.style.userSelect = 'none'

    function handleMouseMove(ev) {
      if (!dragging.current) return
      onResize(direction === 'vertical' ? ev.movementX : ev.movementY)
    }
    function handleMouseUp() {
      dragging.current = false
      document.body.style.cursor = ''
      document.body.style.userSelect = ''
      window.removeEventListener('mousemove', handleMouseMove)
      window.removeEventListener('mouseup', handleMouseUp)
    }

    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseup', handleMouseUp)
  }

  return (
    <div
      className={`splitter splitter-${direction}`}
      onMouseDown={handleMouseDown}
    />
  )
}
