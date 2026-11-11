import React, { createContext, useContext, useReducer, ReactNode } from 'react';
import { ModelMessage } from '../types';

type Action =
  | { type: 'ADD_MESSAGE'; payload: ModelMessage }
  | { type: 'SET_STATUS'; payload: string }
  | { type: 'APPROVE_TOOL' }
  | { type: 'CANCEL_TURN' }
  | { type: 'RESET' };

interface State {
  messages: ModelMessage[];
  status: string;
  pendingToolApproval: { toolCallId: string; tool: string; args: unknown } | null;
}

const initialState: State = {
  messages: [],
  status: 'ready',
  pendingToolApproval: null,
};

function reducer(state: State, action: Action): State {
  switch (action.type) {
    case 'ADD_MESSAGE':
      return { ...state, messages: [...state.messages, action.payload] };
    case 'SET_STATUS':
      return { ...state, status: action.payload };
    case 'APPROVE_TOOL':
      return { ...state, pendingToolApproval: null };
    case 'CANCEL_TURN':
      return { ...state, messages: state.messages.filter(m => m.role !== 'assistant') };
    case 'RESET':
      return initialState;
    default:
      return state;
  }
}

interface ContextType extends State {
  sendMessage: (text: string) => void;
  approveTool: () => void;
  cancelTurn: () => void;
}

const Context = createContext<ContextType | undefined>(undefined);

export function AgentProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(reducer, initialState);

  const sendMessage = (text: string) => {
    dispatch({ type: 'SET_STATUS', payload: 'streaming' });
    const msg: ModelMessage = {
      id: Math.random().toString(36).slice(2),
      role: 'assistant',
      content: 'Streaming...',
      status: 'streaming',
    };
    dispatch({ type: 'ADD_MESSAGE', payload: msg });
  };

  const approveTool = () => dispatch({ type: 'APPROVE_TOOL' });
  const cancelTurn = () => {
    dispatch({ type: 'CANCEL_TURN' });
    dispatch({ type: 'SET_STATUS', payload: 'ready' });
  };

  return (
    <Context.Provider value={{ ...state, sendMessage, approveTool, cancelTurn }}>
      {children}
    </Context.Provider>
  );
}

export function useAgent() {
  const ctx = useContext(Context);
  if (!ctx) throw new Error('useAgent must be used within AgentProvider');
  return ctx;
}
