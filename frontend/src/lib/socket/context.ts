import { act, createContext} from "react";

// Stato iniziale del WebSocket
export interface ISocketContextState {
  socket: WebSocket | undefined;
 
}

export const defaultSocketContextState: ISocketContextState = {
  socket: undefined,
  
};

// Tipi di azioni
export type TSocketContextActions = "update_socket";

export type TSocketContextPayolad= WebSocket

export interface ISocketContextActions {
  type: TSocketContextActions;
  payload: TSocketContextPayolad;
}

// Reducer per aggiornare lo stato
export const SocketReducer = (state: ISocketContextState, action: ISocketContextActions) => {
  switch (action.type) {
    case "update_socket":
      return { ...state, socket: action.payload };
    default:
      return state;
  }
};

// Proprietà del Context
export interface ISocketContextProps {
  socketState: ISocketContextState;
  socketDispatch: React.Dispatch<ISocketContextActions>;
}

const SocketContext = createContext<ISocketContextProps>({
  socketState: defaultSocketContextState,
  socketDispatch: () => {},
});


export const SocketContextConsumer = SocketContext.Consumer;
export const SocketContextProvider = SocketContext.Provider;
export default SocketContext;