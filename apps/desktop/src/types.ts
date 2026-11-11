export type ModelMessage =
  | { id: string; role: 'user'; content: string }
  | { id: string; role: 'assistant'; content: string; toolCalls?: Array<{ name: string; arguments: any }>; status: 'streaming' | 'completed'; toolResult?: string }
  | { id: string; role: 'system'; content: string; status: 'completed' };
