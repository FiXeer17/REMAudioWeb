import { createContext, useContext, useEffect, useState } from "react";
import { getUUID,getSocket } from "../services";
import { useNavigate } from "react-router-dom";

interface UUIDContextType {
    uuid:string|undefined
    sockets:{ ip: string; port: number }[]|null
}
const UUIDContext = createContext<UUIDContextType | undefined>(undefined);
const SocketContext = createContext<UUIDContextType | undefined>(undefined);



export const UUIDProvider: React.FC<{ children: React.ReactNode }> = ({ children }) =>{
    const [uuid, setUuid] = useState<string>()
    const [sockets, setSockets] = useState<{ ip: string; port: number }[] | null>(null)
    
    const navigate=useNavigate()
       useEffect(()=>{
        const fetchUUID = async () => {
          try {
            const value = await getUUID();
            setUuid(value.data.uuid); 
          } catch (error) {
            console.error("Error fetching UUID:", error);
          }
        }
        fetchUUID()
       },[])
       useEffect(()=>{
        if (!uuid) return
        const fetchSocket=async()=>{

          try {
            const value = await getSocket();
            setSockets(value.data.sockets)
            if(value.data.sockets===null)
              return navigate("/newconnections")
            else
              return navigate("/recentconnections")
            
          } catch (error) {
            console.error("Error getting Socket:", error);
          }
        }
        fetchSocket()
       },[uuid])
       
    return(
        <UUIDContext.Provider value={{uuid,sockets}} >
            <SocketContext.Provider value={{ uuid, sockets }}> 
              {children}
            </SocketContext.Provider>
        </UUIDContext.Provider>
    )
}
export default UUIDProvider

export const useUUID = (): UUIDContextType => {
  const context = useContext(UUIDContext);
  if (!context) {
    throw new Error('useUUID must be used within a UUIDProvider');
  }
  return context;
};
export const useSockets = (): UUIDContextType => {
  const context = useContext(SocketContext);
  if (!context) {
    throw new Error('useUUID must be used within a UUIDProvider');
  }
  return context;
};