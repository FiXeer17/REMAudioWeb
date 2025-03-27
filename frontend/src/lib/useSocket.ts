import { useEffect, useRef } from "react";


export const useSocket=(uri: string): WebSocket => {
  const { current: socket } = useRef(new WebSocket(uri))
  console.log(socket)
  useEffect(()=>{
    return () => {
      if (socket) socket.close();

    }
  }, [socket])
  return socket
}
