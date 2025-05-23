import {createContext} from "react";


export interface ISocketContextState {
  socket: WebSocket | undefined;
  message_matrix: string,
  message_camera: string,
  matrix_status: string,
  camera_status: string
 
}

export const defaultSocketContextState: ISocketContextState = {
  socket: undefined,
  message_matrix: "",
  message_camera: "",
  matrix_status: "",
  camera_status: ""

};


export type TSocketContextActions = "update_socket"|"new_message_matrix"|"new_message_camera"|"matrix_status"|"camera_status";

export type TSocketContextPayload= WebSocket|string

export interface ISocketContextActions {
  type: TSocketContextActions;
  payload: TSocketContextPayload;
}


export const SocketReducer = (state: ISocketContextState, action: ISocketContextActions) => {
  switch (action.type) {
    case "update_socket":
      return { ...state, socket: action.payload as WebSocket };
    case "new_message_matrix":
      return { ...state, message_matrix: action.payload as string };
    case "new_message_camera":
      return { ...state, message_camera: action.payload as string };
    case "matrix_status":
      return { ...state, matrix_status: action.payload as string };
    case "camera_status":
      return { ...state, camera_status: action.payload as string };
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