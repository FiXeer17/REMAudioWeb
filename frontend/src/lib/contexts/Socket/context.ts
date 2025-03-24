import { createContext } from "react";
import { Socket } from "socket.io-client";
import { Action } from "sonner";

export interface ISocketContextState{
    socket: Socket | undefined;
}

export const defaultSocketContextState: ISocketContextState={
    socket:undefined
}

export type TSocketContextActions="update_socket"

export type TSocketContextPayolad= Socket

export interface ISocketContextActions{
    type: TSocketContextActions
    payload: TSocketContextPayolad
}

export const SocketReducer = (state: ISocketContextState, action: ISocketContextActions) => {
        switch(action.type){
            case 'update_socket':
                return {...state, socket:action.payload as Socket}
            default:
                return {...state}
        }
} 

export interface ISocketContextProps {
    socketState: ISocketContextState
    socketDispatch: React.Dispatch<ISocketContextActions>
}

const SocketContext= createContext<ISocketContextProps>({
    socketState: defaultSocketContextState,
    socketDispatch: ()=>{}


})

export const SocketContextConsumer= SocketContext.Consumer
export const SocketContextProvider= SocketContext.Provider 

export default SocketContext;