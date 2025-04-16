import { createContext, useContext, useEffect,useState } from "react";
import { getUUID, getSocket } from "../services";
import { useLocation } from "react-router-dom";


interface ConnectionsContextType {
  uuid: string | undefined;
  sockets: { name: string; ip: string; port: number }[] | null;
  isAdmin:boolean;
  triggerRedirect: () => void;
}
const ConnectionsContext = createContext<ConnectionsContextType | undefined>(undefined);


export const ConnectionsProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [uuid, setUuid] = useState<string>();
  const [prevUuid, setPrevUuid] = useState<string>();
  const [sockets, setSockets] = useState<{ name: string; ip: string; port: number }[] | null>(null);
  const location = useLocation();
  


  const [isAdmin]=useState<boolean>(()=>{
    if (location.state===null)
      return Boolean(localStorage.getItem("isAdmin"))
    else
      return location.state.isAdmin
  })
  

  useEffect(() => {
    const fetchUUID = async () => {
      try {
        const value = await getUUID();
        setPrevUuid(value.data.uuid);
      } catch (error) {
        console.error("Error fetching UUID:", error);
      }
    };
    fetchUUID();
  }, []);

  const fetchSocket = async () => {
    if (!prevUuid) return;
    try {
      const value = await getSocket();
      if (isAdmin) {
        setSockets(value.data.sockets);
      } else {
        setSockets(value.data.latest_socket ? [value.data.latest_socket] : null);
      }
      setUuid(prevUuid)
    } catch (error) {
      console.error("Error getting Socket:", error);
    }  
  }

  useEffect(() => {
    if (prevUuid) {
      fetchSocket();
    }
  }, [prevUuid]);

 

  return (

    <ConnectionsContext.Provider value={{ uuid, sockets,isAdmin, triggerRedirect: fetchSocket }}>
        
        {children}
    </ConnectionsContext.Provider>

  );
};
export default ConnectionsProvider;

export const useConnections = (): ConnectionsContextType => {
  const context = useContext(ConnectionsContext);
  if (!context) {
    throw new Error("useConnections must be used within a UUIDProvider");
  }
  return context;
};

