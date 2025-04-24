import {createContext} from "react";


export interface ISocketContextState {
  socket: WebSocket | undefined;
  message: string
 
}

export const defaultSocketContextState: ISocketContextState = {
  socket: undefined,
  message: ""
  
};


export type TSocketContextActions = "update_socket"|"new_message";

export type TSocketContextPayload= WebSocket|string

export interface ISocketContextActions {
  type: TSocketContextActions;
  payload: TSocketContextPayload;
}


export const SocketReducer = (state: ISocketContextState, action: ISocketContextActions) => {
  switch (action.type) {
    case "update_socket":
      return { ...state, socket: action.payload as WebSocket };
    case "new_message":
      return { ...state, message: action.payload as string };
    default:
      return state;
  }
};


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