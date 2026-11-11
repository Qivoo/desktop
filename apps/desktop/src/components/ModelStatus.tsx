import React from 'react';

interface Props { status: string; }

export function ModelStatus({ status }: Props) {
  const colors: Record<string, string> = {
    'ready': '#00ff88', 'streaming': '#88ff00', 'model_switching': '#ffaa00',
    'model_failover': '#ff4400', 'tool_approval_pending': '#ff6600', 'error': '#ff3355',
  };
  const color = colors[status] || '#ffaa00';
  return <span style={{ color, background: 'rgba(255,255,255,0.15)', padding: '4px 10px', borderRadius: '12px', fontSize: '12px' }}>{status}</span>;
}
